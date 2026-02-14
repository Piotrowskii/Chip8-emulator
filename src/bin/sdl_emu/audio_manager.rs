use sdl2::audio::{AudioDevice, AudioSpecDesired};
use sdl2::{AudioSubsystem, Sdl};
use Chip8_emulator::chip8::chip_8::Mode;
use crate::pattern_wave::PatternWave;
use crate::square_wave::SquareWave;

pub struct AudioManager{
    pub pattern_wave: Option<AudioDevice<PatternWave>>,
    pub square_wave: Option<AudioDevice<SquareWave>>,
    mode: Mode,
    playing_sounds: bool,
}

impl AudioManager{
    pub fn new(context: &mut Sdl, mode: Mode) -> AudioManager{
        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1),
            samples: None,
        };

        AudioManager{
            pattern_wave: Self::get_pattern_wave(&desired_spec, &context),
            square_wave: Self::get_square_wave(&desired_spec, &context),
            mode,
            playing_sounds: false,
        }
    }
    fn get_pattern_wave(spec: &AudioSpecDesired, context: &Sdl) -> Option<AudioDevice<PatternWave>>{
        let audio_subsystem = context.audio().ok()?;
        let pattern_device = audio_subsystem.open_playback(None, &spec, |spec| {
            PatternWave {
                phase: 0.0,
                phase_inc: 0.0,
                volume: 0.05,
                pattern: [0; 16],
                pitch_register: 64,
            }
        }).ok()?;
        Some(pattern_device)
    }
    fn get_square_wave(spec: &AudioSpecDesired, context: &Sdl) -> Option<AudioDevice<SquareWave>>{
        let audio_subsystem = context.audio().ok()?;
        let square_device = audio_subsystem.open_playback(None, &spec, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.05
            }
        }).ok()?;
        Some(square_device)
    }

    fn stop_sounds(&mut self){
        if let Some(pattern_device) = &self.pattern_wave{
            pattern_device.pause();
        }
        if let Some(square_device) = &self.square_wave{
            square_device.pause();
        }
    }

    pub fn play_sounds(&mut self, mode: &Mode, sound_timer: u8, sound_pattern_buffer: [u8; 16], pitch_register: u8){
        if *mode != self.mode {
           self.stop_sounds();
            self.mode = *mode;
        }

        match self.mode {
            Mode::XoChip => self.play_pattern_wave(sound_timer, sound_pattern_buffer, pitch_register),
            _ => self.play_square_wave(sound_timer),
        }
    }

    fn play_square_wave(&mut self, sound_timer: u8){
        if let Some(device) = &self.square_wave{

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

    fn play_pattern_wave(&mut self, sound_timer: u8, sound_pattern_buffer: [u8; 16], pitch_register: u8){
        if let Some(device) = self.pattern_wave.as_mut() {

            if sound_timer > 0 {
                {
                    let mut lock = device.lock();
                    lock.pattern.copy_from_slice(&sound_pattern_buffer);
                    lock.pitch_register = pitch_register;
                    lock.update_pitch(44_100.0);

                    if !self.playing_sounds {
                        lock.phase = 0.0;
                    }
                }
            }

            if sound_timer > 0 && !self.playing_sounds {
                device.resume();
                self.playing_sounds = true;
            } else if sound_timer == 0 && self.playing_sounds {
                device.pause();
                self.playing_sounds = false;
            }
        }
    }
}