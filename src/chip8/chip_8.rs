use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::{fs, thread};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use sdl2::keyboard::Scancode::Mute;
use crate::chip8::cpu_state::CpuState;
use crate::chip8::decoded_instruction::DecodedInstruction;
use crate::chip8::instructions::Instruction;
use crate::emulator::parameters::*;

#[repr(usize)]
#[derive(PartialEq, Copy, Clone)]
pub enum KeyPad{
    Num0 = 0,
    Num1 = 1,
    Num2 = 2,
    Num3 = 3,
    Num4 = 4,
    Num5 = 5,
    Num6 = 6,
    Num7 = 7,
    Num8 = 8,
    Num9 = 9,
    A = 10,
    B = 11,
    C = 12,
    D = 13,
    E = 14,
    F = 15,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Mode{
    Chip8,
    SuperChip,
    XoChip,
    Experimental
}

#[derive(PartialEq, Copy, Clone)]
pub struct Display{
    pub plane_1: [bool; DISPLAY_SIZE],
    pub plane_2: [bool; DISPLAY_SIZE],
    pub selected_plane: u8,
}

impl Display{
    pub fn new() -> Display{
        Display{
            plane_1: [false; DISPLAY_SIZE],
            plane_2: [false; DISPLAY_SIZE],
            selected_plane: 1,
        }
    }
    pub fn get_selected_planes(&mut self) -> Vec<&mut [bool; DISPLAY_SIZE]>{
        match self.selected_plane{
            1 => vec![&mut self.plane_1],
            2 => vec![&mut self.plane_2],
            3 => vec![&mut self.plane_1, &mut self.plane_2],
            _ => vec![]
        }
    }

    pub fn execute_scroll(&mut self, scroll_function: fn(display: &mut [bool;DISPLAY_SIZE], n: usize), n: usize){
        for plane in self.get_selected_planes(){
            scroll_function(plane,n);
        }
    }
}
pub struct Chip8{
    pub state: Arc<Mutex<CpuState>>,
    pub display: Arc<Mutex<Display>>,
    pub running: Arc<AtomicBool>,
    pub keys: Arc<Mutex<[bool; 16]>>,
    pub hires_mode: Arc<AtomicBool>,
}

impl Chip8{
    pub fn new() -> Chip8{
        Chip8{
            state: Arc::new(Mutex::new(CpuState::default())),
            display: Arc::new(Mutex::new(Display::new())),
            running: Arc::new(AtomicBool::new(true)),
            keys: Arc::new(Mutex::new([false; 16])),
            hires_mode: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn get_new_and_start(rom_file: PathBuf, mode: Mode) -> Chip8{
        let mut chip8 = Chip8::new();
        chip8.set_compatibility_mode(mode);
        chip8.start(rom_file);
        chip8
    }

    fn start(&mut self, rom_file: PathBuf){
        self.load_font_into_memory();
        self.load_cartridge(rom_file);
        self.start_timer_thread();
        self.start_execution_thread();
    }

    fn set_compatibility_mode(&mut self, mode: Mode){
        let cpu = &mut self.state.lock().unwrap();

        match mode {
            Mode::Chip8 => {
                cpu.alt_8XY123 = true;
                cpu.alt_FX55_FX65 = true;
            }
            Mode::SuperChip => {
                cpu.alt_FX55_FX65 = true;
                cpu.alt_allow_scrolling = true;
            }
            Mode::XoChip => {
                cpu.alt_allow_scrolling = false;
            }
            Mode::Experimental => {
                cpu.alt_FX55_FX65 = false;
                cpu.alt_allow_scrolling = true;
            }
        }
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

    pub fn load_cartridge(&mut self, rom_file: PathBuf){
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

        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                let start = Instant::now();

                {
                    let mut cpu_state = state.lock().unwrap();
                    cpu_state.delay_timer = cpu_state.delay_timer.saturating_sub(1u8);
                    cpu_state.sound_timer = cpu_state.sound_timer.saturating_sub(1u8);
                }

                let elapsed = start.elapsed().as_nanos() as u64;
                thread::sleep(Duration::from_nanos(HZ.saturating_sub(elapsed)));
            }
        });
    }
    
    fn start_execution_thread(&mut self) {
        let display = Arc::clone(&self.display);
        let state = Arc::clone(&self.state);
        let running = Arc::clone(&self.running);
        let keys = Arc::clone(&self.keys);
        let hires_mode = Arc::clone(&self.hires_mode);

        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                let start = Instant::now();
                let mut num = 0;
                {
                    let mut cpu_state = state.lock().unwrap();
                    let mut display = display.lock().unwrap();
                    let mut keys = keys.lock().unwrap();

                    for i in 0..IPF{
                        if let Some(instruction) = cpu_state.get_current_instruction(true){
                            instruction.execute(&mut cpu_state, &mut display, &keys, hires_mode.as_ref(), running.as_ref());
                        }
                        num += 1;
                    }
                }

                let elapsed = start.elapsed().as_nanos() as u64;
                thread::sleep(Duration::from_nanos(HZ.saturating_sub(elapsed))); 
            }
        });
    }

    pub fn handle_input(&mut self, pressed_key: KeyPad, pressed: bool){
        let mut keys = self.keys.lock().unwrap();
        keys[pressed_key as usize] = pressed;
    }

}



