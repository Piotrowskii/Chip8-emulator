use sdl2::render::WindowCanvas;
use sdl2::{AudioSubsystem, EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;
use crate::chip_8::{Chip8, KeyPad};
use crate::parameters::*;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use std::time::Duration;
use crate::square_wave::SquareWave;

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
    pub fn new() -> Emulator {
        let mut sdl_context = sdl2::init().expect("SDL initialization failed");
        let video_subsystem = sdl_context.video().expect("SDL initialization failed");

        let window = video_subsystem
            .window("Chip8 emulator", 800, 600)
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
            chip8: Chip8::new(),
            playing_sounds: false,
            audio_device
        }
    }

    pub fn run(&mut self){
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running },
                    _ => {}
                }
            }

            self.canvas.clear();
            self.draw_screen();
            self.chip8.decrement_timers();
            self.make_sounds();
            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 16_666_667));
        }
    }

    fn draw_screen(&mut self){
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);

        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let idx = y * DISPLAY_WIDTH + x;

                if self.chip8.display[idx] {
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
                volume: 0.25
            }
        }).ok()?;
        Some(device)
    }

    fn make_sounds(&mut self){
        if let Some(device) = &self.audio_device{
            if self.chip8.sound_timer > 0 && !self.playing_sounds {
                device.resume();
            }
            else{
                device.pause();
            }
        }
    }

    fn handle_keypress(&mut self, event: Event){
        match event {
            Event::KeyDown {keycode: Some(key), ..} =>{
                match key{
                    Keycode::NUM_1 => {self.chip8.handle_input(KeyPad::Num1)}
                    Keycode::NUM_2 => {self.chip8.handle_input(KeyPad::Num2)}
                    Keycode::NUM_3 => {self.chip8.handle_input(KeyPad::Num3)}
                    Keycode::NUM_4 => {self.chip8.handle_input(KeyPad::C)}
                    Keycode::Q => {self.chip8.handle_input(KeyPad::Num4)}
                    Keycode::W => {self.chip8.handle_input(KeyPad::Num5)}
                    Keycode::E => {self.chip8.handle_input(KeyPad::Num6)}
                    Keycode::R => {self.chip8.handle_input(KeyPad::D)}
                    Keycode::A => {self.chip8.handle_input(KeyPad::Num7)}
                    Keycode::S => {self.chip8.handle_input(KeyPad::Num8)}
                    Keycode::D => {self.chip8.handle_input(KeyPad::Num9)}
                    Keycode::F => {self.chip8.handle_input(KeyPad::E)}
                    Keycode::Z => {self.chip8.handle_input(KeyPad::A)}
                    Keycode::X => {self.chip8.handle_input(KeyPad::Num0)}
                    Keycode::C => {self.chip8.handle_input(KeyPad::B)}
                    Keycode::V => {self.chip8.handle_input(KeyPad::F)}
                    _ => {}
                }
            }
            _ => {}
        }

    }
}