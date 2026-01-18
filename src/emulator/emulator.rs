use std::sync::atomic::Ordering;
use std::thread;
use sdl2::render::WindowCanvas;
use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::chip8::chip_8::{Chip8, KeyPad};
use sdl2::audio::{AudioDevice, AudioSpecDesired};
use std::time::{Duration, Instant};
use crate::emulator::parameters::*;
use crate::emulator::square_wave::SquareWave;

extern crate sdl2;

pub struct Emulator {
    context: Sdl,
    canvas: WindowCanvas,
    event_pump: EventPump,
    chip8: Chip8,
    audio_device: Option<AudioDevice<SquareWave>>,
    playing_sounds: bool,
}

impl Emulator{
    pub fn new(chip8: Chip8) -> Emulator {
        let mut sdl_context = sdl2::init().expect("SDL initialization failed");
        let video_subsystem = sdl_context.video().expect("SDL initialization failed");

        let window = video_subsystem
            .window("Chip8 emulator", (DISPLAY_WIDTH as u32 * PIXEL_SIZE) , (DISPLAY_HEIGHT as u32 * PIXEL_SIZE))
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).expect("Could not build window");

        let canvas = window.into_canvas().build().map_err(|e| e.to_string()).expect("Could not build canvas");

        let event_pump = sdl_context.event_pump().expect("Could not get SDL event pump");

        let audio_device = Self::get_audio_device(&mut sdl_context);

        Emulator{
            context: sdl_context,
            canvas,
            event_pump,
            chip8,
            playing_sounds: false,
            audio_device
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
                    _ => self.handle_keypad_presses(&event),
                }
            }

            self.canvas.clear();
            self.draw_screen();
            self.make_sounds();
            self.canvas.present();

            let elapsed = start.elapsed().as_nanos() as u64;
            thread::sleep(Duration::from_nanos(16_666_667u64.saturating_sub(elapsed) ));
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

                if display[idx] {
                    let rect = Rect::new(
                        (x as u32 * PIXEL_SIZE) as i32,
                        (y as u32 * PIXEL_SIZE) as i32,
                        PIXEL_SIZE,
                        PIXEL_SIZE,
                    );

                    self.canvas.fill_rect(rect).expect("Could not draw the screen");
                }
            }
        }

        self.canvas.present();
    }

    fn get_display_copy(&self) -> [bool; DISPLAY_SIZE]{
        let display = self.chip8.display.lock().unwrap();
        *display
    }

    pub fn get_audio_device(context: &mut Sdl) -> Option<AudioDevice<SquareWave>>{
        let audio_subsystem = context.audio().ok()?;

        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.05
            }
        }).ok()?;
        Some(device)
    }

    fn make_sounds(&mut self){
        if let Some(device) = &self.audio_device{
            let sound_timer = self.chip8.state.lock().unwrap().sound_timer;

            if !self.playing_sounds && sound_timer > 0{
                device.resume();
                self.playing_sounds = true;
            }
            else if sound_timer == 0 && self.playing_sounds{
                device.pause();
                self.playing_sounds = false;
            }
        }
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

}