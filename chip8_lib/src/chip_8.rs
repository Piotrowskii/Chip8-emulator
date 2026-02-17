use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, Ordering};
use std::{fs, thread};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use crate::chip_8;
use crate::cpu_state::CpuState;
use crate::display::Display;
use crate::keypad::KeyPad;
use crate::parameters::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Mode{
    Chip8,
    SuperChip,
    XoChip,
    Experimental
}
pub struct Chip8{
    pub state: Arc<Mutex<CpuState>>,
    pub display: Arc<Mutex<Display>>,
    pub running: Arc<AtomicBool>,
    pub keys: Arc<Mutex<[bool; 16]>>,
    pub hires_mode: Arc<AtomicBool>,
    pub fps_ns: Arc<AtomicU64>, //in nano seconds
    pub ipf: Arc<AtomicU32>,
    pub compatibility_mode: Arc<Mutex<Mode>>
}

impl Chip8{
    pub fn new(mode: Mode) -> Chip8{
        let mut chip_8 = Chip8{
            state: Arc::new(Mutex::new(CpuState::default())),
            display: Arc::new(Mutex::new(Display::new())),
            running: Arc::new(AtomicBool::new(true)),
            keys: Arc::new(Mutex::new([false; 16])),
            hires_mode: Arc::new(AtomicBool::new(false)),
            fps_ns: Arc::new(AtomicU64::new(16_666_667)),
            ipf: Arc::new(AtomicU32::new(100)),
            compatibility_mode: Arc::new(Mutex::new(mode)),
        };
        chip_8.set_compatibility_mode(&mode);
        chip_8
    }

    pub fn get_new_and_start(rom_file: &PathBuf, mode: Mode) -> Chip8{
        let mut chip8 = Chip8::new(mode);
        chip8.start(rom_file);
        chip8
    }

    pub fn set_compatibility_mode(&mut self, mode: &Mode){
        let mut current_compatibility = self.compatibility_mode.lock().unwrap();
        *current_compatibility = *mode;
        
        let mut cpu = self.state.lock().unwrap();
        cpu.set_compatibility_mode(&mode);
        match mode {
            Mode::Chip8 => {self.ipf.store(100, Ordering::Relaxed);}
            Mode::SuperChip => {self.ipf.store(500, Ordering::Relaxed);}
            Mode::XoChip => {self.ipf.store(1000, Ordering::Relaxed);}
            Mode::Experimental => {self.ipf.store(500, Ordering::Relaxed);}
        }
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }

    pub fn start(&mut self, rom_file: &PathBuf){
        self.load_font_into_memory();
        self.load_cartridge(rom_file);
        self.start_timer_thread();
        self.start_execution_thread();
    }

    pub fn load_font_into_memory(&self){
        let mut state = self.state.lock().unwrap();

        for i in 0..FONT_DATA.len(){
            state.memory[FONT_MEMORY_START + i] = FONT_DATA[i];
        }

        for i in 0..BIG_FONT_DATA.len(){
            state.memory[FONT_MEMORY_START + FONT_DATA.len() + i] = BIG_FONT_DATA[i];
        }
    }

    pub fn load_cartridge(&mut self, rom_file: &PathBuf){
        let mut state = self.state.lock().unwrap();

        let file = fs::read(rom_file);
        if let Ok(rom) = file{
            for (i, byte) in rom.iter().enumerate(){
                state.memory[0x200 + i] = *byte;
            }
        }
        else{
            panic!("Could not read ROM file");
        }
    }

    fn start_timer_thread(&mut self) {
        let state = Arc::clone(&self.state);
        let running = Arc::clone(&self.running);
        let fps = Arc::clone(&self.fps_ns);

        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                let start = Instant::now();

                {
                    let mut cpu_state = state.lock().unwrap();
                    cpu_state.delay_timer = cpu_state.delay_timer.saturating_sub(1u8);
                    cpu_state.sound_timer = cpu_state.sound_timer.saturating_sub(1u8);
                }

                let elapsed = start.elapsed().as_nanos() as u64;
                thread::sleep(Duration::from_nanos(fps.load(Ordering::Relaxed).saturating_sub(elapsed)));
            }
        });
    }
    
    fn start_execution_thread(&mut self) {
        let display = Arc::clone(&self.display);
        let state = Arc::clone(&self.state);
        let running = Arc::clone(&self.running);
        let keys = Arc::clone(&self.keys);
        let hires_mode = Arc::clone(&self.hires_mode);
        let ipf = Arc::clone(&self.ipf);
        let fps = Arc::clone(&self.fps_ns);

        thread::spawn(move || {
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
                let elapsed = start.elapsed().as_nanos() as u64;
                thread::sleep(Duration::from_nanos(fps.load(Ordering::Relaxed).saturating_sub(elapsed)));
            }
        });
    }

    pub fn handle_input(&mut self, pressed_key: KeyPad, pressed: bool){
        let mut keys = self.keys.lock().unwrap();
        keys[pressed_key as usize] = pressed;
    }

}



