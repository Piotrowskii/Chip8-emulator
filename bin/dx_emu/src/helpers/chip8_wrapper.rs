use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use web_time::Instant;
use chip8_lib::chip_8::{Chip8, Mode};
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use chip8_lib::display::Display;

pub struct Chip8Web{
    pub chip8: Chip8
}

impl Chip8Web {
    pub fn new(mode: Mode) -> Chip8Web {
        Chip8Web{
            chip8: Chip8::new(mode)
        }
    }
    pub fn start(&mut self, rom_bytes: &[u8]){
        self.chip8.load_font_into_memory();
        self.load_cartridge(rom_bytes);
        self.start_timer_thread();
        self.start_execution_thread();
    }

    fn start_timer_thread(&mut self){
        let state = Arc::clone(&self.chip8.state);
        let running = Arc::clone(&self.chip8.running);
        let fps_ns = Arc::clone(&self.chip8.fps_ns);

        spawn(async move {
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
    }

    fn start_execution_thread(&mut self){
        let display = Arc::clone(&self.chip8.display);
        let state = Arc::clone(&self.chip8.state);
        let running = Arc::clone(&self.chip8.running);
        let keys = Arc::clone(&self.chip8.keys);
        let hires_mode = Arc::clone(&self.chip8.hires_mode);
        let ipf = Arc::clone(&self.chip8.ipf);
        let fps_ns = Arc::clone(&self.chip8.fps_ns);

        spawn(async move {
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
    }
    fn load_cartridge(&mut self, rom_bytes: &[u8]){
        let mut state = self.chip8.state.lock().unwrap();
        for (i, byte) in rom_bytes.iter().enumerate(){
            state.memory[0x200 + i] = *byte;
        }
    }
    
    pub fn get_display_copy(&self) -> Option<Display>{
        let display = Arc::clone(&self.chip8.display);
        if let Ok(display) = display.try_lock() {
            return Some(*display);
        };
        None
    }
}