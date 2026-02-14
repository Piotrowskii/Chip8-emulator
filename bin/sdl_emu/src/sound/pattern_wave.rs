use sdl2::audio::AudioCallback;

pub struct PatternWave {
    pub phase: f32,
    pub phase_inc: f32,
    pub volume: f32,
    pub pattern: [u8; 16],
    pub pitch_register: u8, // 0–255
}

impl AudioCallback for PatternWave {
    type Channel = f32;
    fn callback(&mut self, out: &mut [f32]) {
        for sample in out.iter_mut() {
            let index = (self.phase * 128.0) as usize % 128;

            let byte = self.pattern[index / 8];
            let bit = (byte >> (7 - (index % 8))) & 1;

            *sample = if bit == 1 {
                self.volume
            } else {
                -self.volume
            };

            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

impl PatternWave {
    pub fn update_pitch(&mut self, spec_freq: f32) {
        let pitch = self.pitch_register as f32;
        let playback_rate = 4000.0 * 2f32.powf((pitch - 64.0) / 48.0);

        let pattern_frequency = playback_rate / 128.0;

        self.phase_inc = pattern_frequency / spec_freq;
    }
}