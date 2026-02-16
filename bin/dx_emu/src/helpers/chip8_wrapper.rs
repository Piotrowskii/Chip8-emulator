use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use dioxus::dioxus_core::Task;
use web_time::Instant;
use chip8_lib::chip_8::{Chip8, KeyPad, Mode};
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use chip8_lib::display::Display;
use rand::random;
use crate::helpers::game::Game;

pub struct Chip8Web{
    chip8: Chip8,
    timer_thread: Option<Task>,
    execution_thread: Option<Task>,
    display_thread: Option<Task>,
    ipf_before_pause: u32,
    fps_before_pause: u64,
}

impl Chip8Web {
    pub fn new(mode: Mode) -> Chip8Web {
        let chip8 = Chip8::new(mode);
        let ipf_before_pause = chip8.ipf.load(Ordering::Relaxed);
        
        Chip8Web{
            chip8,
            execution_thread: None,
            timer_thread: None,
            display_thread: None,
            ipf_before_pause,
            fps_before_pause: 0,
        }
    }
    pub fn start(&mut self, game: &Game, display_signal: &mut Signal<Display>){
        self.chip8.load_font_into_memory();
        self.load_cartridge(game.bytes);
        self.start_timer_thread();
        self.start_execution_thread();
        self.start_display_thread(display_signal)
    }

    pub fn stop(&mut self) {
        self.chip8.running.store(false, Ordering::Relaxed);

        if let Some(timer_thread) = self.timer_thread.take() {
            timer_thread.cancel();
        }
        if let Some(execution_thread) = self.execution_thread.take() {
            execution_thread.cancel();
        }
        if let Some(display_thread) = self.display_thread.take() {
            display_thread.cancel();
        }
    }

    fn start_timer_thread(&mut self){
        let state = Arc::clone(&self.chip8.state);
        let running = Arc::clone(&self.chip8.running);
        let fps_ns = Arc::clone(&self.chip8.fps_ns);

        let timer_thread = spawn(async move {
            while running.load(Ordering::Relaxed) {
                let start = Instant::now();

                if let Ok(mut cpu_state) = state.lock() {
                    cpu_state.delay_timer = cpu_state.delay_timer.saturating_sub(1);
                    cpu_state.sound_timer = cpu_state.sound_timer.saturating_sub(1);
                }

                let elapsed_ns = start.elapsed().as_nanos() as u64;
                let wait_time_ms = ((fps_ns.load(Ordering::Relaxed).saturating_sub(elapsed_ns) as f64) / 1_000_000f64).round() as u32;

                TimeoutFuture::new(wait_time_ms.max(1)).await;
            }

        });

        self.timer_thread = Some(timer_thread);
    }

    fn start_execution_thread(&mut self){
        let display = Arc::clone(&self.chip8.display);
        let state = Arc::clone(&self.chip8.state);
        let running = Arc::clone(&self.chip8.running);
        let keys = Arc::clone(&self.chip8.keys);
        let hires_mode = Arc::clone(&self.chip8.hires_mode);
        let ipf = Arc::clone(&self.chip8.ipf);
        let fps_ns = Arc::clone(&self.chip8.fps_ns);

        let execution_thread = spawn(async move {
            while running.load(Ordering::Relaxed) {
                let start = Instant::now();
                {
                    let mut cpu_state = state.lock().unwrap();
                    let mut display = display.lock().unwrap();
                    let keys = keys.lock().unwrap();

                    for _ in 0..ipf.load(Ordering::Relaxed){
                        if let Some(instruction) = cpu_state.get_current_instruction(true){
                            instruction.execute(&mut cpu_state, &mut display, &keys, hires_mode.as_ref(), running.as_ref());
                        }

                    }

                }
                let elapsed_ns = start.elapsed().as_nanos() as u64;
                let wait_time_ms = ((fps_ns.load(Ordering::Relaxed).saturating_sub(elapsed_ns) as f64) / 1_000_000f64).round() as u32;
                TimeoutFuture::new(wait_time_ms.max(1)).await;
            }
        });

        self.execution_thread = Some(execution_thread);
    }

    fn start_display_thread(&mut self, display_signal: &mut Signal<Display>){
        let fps_ns = Arc::clone(&self.chip8.fps_ns);
        let chip8_display = Arc::clone(&self.chip8.display);
        let running = Arc::clone(&self.chip8.running);
        let mut display_signal = display_signal.clone();

        let display_thread = spawn(async move {
            while running.load(Ordering::Relaxed) {
                let start = Instant::now();

                if let Ok(display) = chip8_display.try_lock(){
                    display_signal.set(*display);
                }

                TimeoutFuture::new(16).await;

                let elapsed_ns = start.elapsed().as_nanos() as u64;
                let wait_time_ms = ((fps_ns.load(Ordering::Relaxed).saturating_sub(elapsed_ns) as f64) / 1_000_000f64).round() as u32;
                TimeoutFuture::new(wait_time_ms.max(1)).await;
            }

        });

        self.display_thread = Some(display_thread);
    }

    fn load_cartridge(&mut self, rom_bytes: &[u8]){
        let mut state = self.chip8.state.lock().unwrap();
        for (i, byte) in rom_bytes.iter().enumerate(){
            state.memory[0x200 + i] = *byte;
        }
    }
    pub fn handle_key_press(&mut self, key: Key, pressed: bool){
        if let Some(key) = Self::get_keypad(&key){
            self.chip8.handle_input(key, pressed);
        }
    }

    pub fn pause(&mut self){
        self.ipf_before_pause = self.chip8.ipf.load(Ordering::Relaxed);
        self.chip8.ipf.store(0, Ordering::Relaxed);
    }
    
    pub fn resume(&mut self){
        self.chip8.ipf.store(self.ipf_before_pause, Ordering::Relaxed);
    }

    pub fn clear_keys(&mut self){
        if let Ok(mut keys) = self.chip8.keys.try_lock(){
            keys.fill(false);
        }
    }
    fn get_keypad(key: &Key) -> Option<KeyPad> {
        match key {
            Key::Character(c) => match c.to_lowercase().as_str() {
                "1" => Some(KeyPad::Num1),
                "2" => Some(KeyPad::Num2),
                "3" => Some(KeyPad::Num3),
                "4" => Some(KeyPad::C),
                "q" => Some(KeyPad::Num4),
                "w" => Some(KeyPad::Num5),
                "e" => Some(KeyPad::Num6),
                "r" => Some(KeyPad::D),
                "a" => Some(KeyPad::Num7),
                "s" => Some(KeyPad::Num8),
                "d" => Some(KeyPad::Num9),
                "f" => Some(KeyPad::E),
                "z" => Some(KeyPad::A),
                "x" => Some(KeyPad::Num0),
                "c" => Some(KeyPad::B),
                "v" => Some(KeyPad::F),
                _ => None,
            },
            _ => None,
        }
    }
}