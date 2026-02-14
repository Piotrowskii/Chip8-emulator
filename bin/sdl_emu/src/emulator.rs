use std::path::{PathBuf};
use std::sync::atomic::Ordering;
use std::thread;
use sdl2::render::WindowCanvas;
use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
use chip8_lib::chip_8::{Chip8, KeyPad, Mode};
use chip8_lib::display::Display;
use chip8_lib::parameters::*;
use crate::file_picker;
use crate::sound::audio_manager::AudioManager;

extern crate sdl2;

pub struct Emulator {
    context: Sdl,
    canvas: WindowCanvas,
    event_pump: EventPump,
    chip8: Chip8,
    current_game: PathBuf,
    fps: u16,
    fps_ns: u64,
    audio_manager: AudioManager
}

impl Emulator{
    pub fn new(file: PathBuf) -> Emulator {
        let starting_mode = Mode::Chip8;

        let mut sdl_context = sdl2::init().expect("SDL initialization failed");
        let video_subsystem = sdl_context.video().expect("SDL initialization failed");

        let window = video_subsystem
            .window("Chip8 sdl_emu", (DISPLAY_WIDTH as u32 * PIXEL_SIZE) , (DISPLAY_HEIGHT as u32 * PIXEL_SIZE))
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).expect("Could not build window");

        let canvas = window.into_canvas().build().map_err(|e| e.to_string()).expect("Could not build canvas");

        let event_pump = sdl_context.event_pump().expect("Could not get SDL event pump");

        let audio_manager = AudioManager::new(&mut sdl_context, starting_mode);

        let fps = 60;
        let fps_ns = Self::get_ns_from_fps(fps);

        let chip8 = Chip8::get_new_and_start(&file, starting_mode);

        Emulator{
            context: sdl_context,
            canvas,
            event_pump,
            chip8,
            current_game: file,
            fps_ns,
            fps,
            audio_manager
        }
    }

    pub fn run(&mut self){
        'running: while self.chip8.running.load(Ordering::Relaxed){
            let start = Instant::now();

            let events: Vec<Event> = self.event_pump.poll_iter().collect::<Vec<Event>>();

            for event in events {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                    Event::KeyUp { keycode: Some(Keycode::Kp4), .. } => self.increase_ipf(100),
                    Event::KeyUp { keycode: Some(Keycode::Kp1), .. } => self.decrease_ipf(100),
                    Event::KeyUp { keycode: Some(Keycode::Kp6), .. } => self.increase_fps(10),
                    Event::KeyUp { keycode: Some(Keycode::Kp3), .. } => self.decrease_fps(10),
                    Event::KeyUp { keycode: Some(Keycode::Kp5), .. } => self.restart_chip8(),
                    Event::KeyUp { keycode: Some(Keycode::Kp7), .. } => self.change_compatibility_mode(Mode::Chip8),
                    Event::KeyUp { keycode: Some(Keycode::Kp8), .. } => self.change_compatibility_mode(Mode::SuperChip),
                    Event::KeyUp { keycode: Some(Keycode::Kp9), .. } => self.change_compatibility_mode(Mode::XoChip),
                    Event::KeyUp { keycode: Some(Keycode::Kp2), .. } => self.change_game(),
                    _ => self.handle_keypad_presses(&event),
                }
            }

            self.canvas.clear();
            self.draw_screen();
            self.play_sounds();

            let elapsed = start.elapsed().as_nanos() as u64;
            thread::sleep(Duration::from_nanos(self.fps_ns.saturating_sub(elapsed) ));
        }
    }

    fn draw_screen(&mut self){
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::WHITE);

        let display = self.get_display_copy();

        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let idx = y * DISPLAY_WIDTH + x;

                let color = Self::get_pixel_color(&display, idx);
                self.canvas.set_draw_color(color);
                
                let rect = Rect::new(
                    (x as u32 * PIXEL_SIZE) as i32,
                    (y as u32 * PIXEL_SIZE) as i32,
                    PIXEL_SIZE,
                    PIXEL_SIZE,
                );

                self.canvas.fill_rect(rect).expect("Could not draw the screen");

            }
        }

        self.canvas.present();
    }

    fn get_pixel_color(display: &Display, idx: usize) -> Color{
        if display.plane_1[idx] && display.plane_2[idx]{
            Color::RGB(238, 238, 255)
        }
        else if display.plane_2[idx]{
            Color::RGB(69, 101, 67)
        }
        else if display.plane_1[idx]{
            Color::RGB(85, 68, 34)
        }
        else{
            Color::RGB(135, 206, 235)
        }
    }

    fn play_sounds(&mut self){
        let (mode, sound_timer, sound_pattern_buffer, pitch_register) = {
            let mode = self.chip8.compatibility_mode.lock().unwrap();
            let state = self.chip8.state.lock().unwrap();
            (*mode,state.sound_timer, state.sound_pattern_buffer, state.pitch_register)
        };
        self.audio_manager.play_sounds(&mode, sound_timer, sound_pattern_buffer, pitch_register);
    }

    fn handle_keypad_presses(&mut self, event: &Event){
        match event{
            Event::KeyDown{keycode: Some(key), ..} => {
                if let Some(valid_key) = Self::get_keypad_number(key){
                    self.chip8.handle_input(valid_key, true);
                }
            }
            Event::KeyUp{keycode: Some(key), ..} => {
                if let Some(valid_key) = Self::get_keypad_number(key){
                    self.chip8.handle_input(valid_key, false);
                }
            }
            _ => {}
        }
    }

    fn get_keypad_number(key: &Keycode) -> Option<KeyPad> {
        match *key {
            Keycode::NUM_1 => Some(KeyPad::Num1),
            Keycode::NUM_2 => {Some(KeyPad::Num2)}
            Keycode::NUM_3 => {Some(KeyPad::Num3)}
            Keycode::NUM_4 => {Some(KeyPad::C)}
            Keycode::Q => {Some(KeyPad::Num4)}
            Keycode::W => {Some(KeyPad::Num5)}
            Keycode::E => {Some(KeyPad::Num6)}
            Keycode::R => {Some(KeyPad::D)}
            Keycode::A => {Some(KeyPad::Num7)}
            Keycode::S => {Some(KeyPad::Num8)}
            Keycode::D => {Some(KeyPad::Num9)}
            Keycode::F => {Some(KeyPad::E)}
            Keycode::Z => {Some(KeyPad::A)}
            Keycode::X => {Some(KeyPad::Num0)}
            Keycode::C => {Some(KeyPad::B)}
            Keycode::V => {Some(KeyPad::F)}
            _ => {None}
        }
    }
    fn get_display_copy(&self) -> Display{
        let display = self.chip8.display.lock().unwrap();
        *display
    }
    fn increase_ipf(&mut self, value: u32){
        let value = self.chip8.ipf.load(Ordering::Relaxed).saturating_add(value);
        self.chip8.ipf.store(value, Ordering::Relaxed);
        println!("IPF increased to {}", value);
    }
    fn decrease_ipf(&mut self, value: u32){
        let value = self.chip8.ipf.load(Ordering::Relaxed).saturating_sub(value);
        self.chip8.ipf.store(value, Ordering::Relaxed);
        println!("IPF decreased to {}", value);
    }
    fn increase_fps(&mut self, additional_fps: u16){
        self.fps = self.fps.saturating_add(additional_fps);
        self.fps_ns = Self::get_ns_from_fps(self.fps);
        self.chip8.fps_ns.store(self.fps_ns, Ordering::Relaxed);
        println!("FPS increased to {}", self.fps);
    }
    fn decrease_fps(&mut self, additional_fps: u16){
        self.fps = self.fps.saturating_sub(additional_fps);
        self.fps_ns = Self::get_ns_from_fps(self.fps);
        self.chip8.fps_ns.store(self.fps_ns, Ordering::Relaxed);
        println!("FPS decreased to {}", self.fps);
    }
    fn get_ns_from_fps(value: u16) -> u64{
        1_000_000_000 / value.max(1) as u64
    }
    fn restart_chip8(&mut self){
        let compatibility = {
            let lock = self.chip8.compatibility_mode.lock().unwrap();
            *lock
        };
        self.chip8.running.store(false, Ordering::Relaxed);
        self.chip8 = Chip8::get_new_and_start(&self.current_game, compatibility);
    }
    fn change_compatibility_mode(&mut self, compatibility_mode: Mode){
        self.chip8.set_compatibility_mode(&compatibility_mode);
        println!("Compatibility mode changed to {:?}", compatibility_mode);
    }
    fn change_game(&mut self){
        let file = file_picker::pick_file();
        if let Some(file) = file {
            self.current_game = file;
        }
        self.restart_chip8();
    }


}