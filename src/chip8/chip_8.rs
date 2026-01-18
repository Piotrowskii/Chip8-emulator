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
    XoChip
}
pub struct Chip8{
    pub state: Arc<Mutex<CpuState>>,
    pub display: Arc<Mutex<[bool; DISPLAY_SIZE]>>,
    pub running: Arc<AtomicBool>,
    pub keys: Arc<Mutex<[bool; 16]>>,
    pub hires_mode: Arc<AtomicBool>,
}

impl Chip8{
    pub fn new() -> Chip8{
        Chip8{
            state: Arc::new(Mutex::new(CpuState::default())),
            display: Arc::new(Mutex::new([false; DISPLAY_SIZE])),
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
                cpu.alt_8XY6_8XYE = true;
                cpu.alt_BNNN = true;
            }
            Mode::XoChip => {
                cpu.alt_8XY6_8XYE = true;
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
                thread::sleep(Duration::from_nanos(16_666_667u64.saturating_sub(elapsed) ));
            }
        });
    }

    //700 Hz
    fn start_execution_thread(&mut self) {
        let display = Arc::clone(&self.display);
        let state = Arc::clone(&self.state);
        let running = Arc::clone(&self.running);
        let keys = Arc::clone(&self.keys);
        let hires_mode = Arc::clone(&self.hires_mode);
        
        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                let start = Instant::now();

                {
                    let mut cpu_state = state.lock().unwrap();
                    let mut display = display.lock().unwrap();
                    let keys = keys.lock().unwrap();

                    let raw_instruction = {
                        Self::fetch(&mut cpu_state)
                    };
    
                    if let Some(instruction) = Self::decode(raw_instruction){
                        instruction.execute(&mut cpu_state, &mut display, &keys, hires_mode.as_ref(), running.as_ref());
                    }

                }

                let elapsed = start.elapsed().as_nanos() as u64;
                thread::sleep(Duration::from_nanos(1_430_000u64.saturating_sub(elapsed) ));
            }
        });
    }

    pub fn handle_input(&mut self, pressed_key: KeyPad, pressed: bool){
        let mut keys = self.keys.lock().unwrap();
        keys[pressed_key as usize] = pressed;
    }
    

    pub fn fetch(cpu_state: &mut CpuState) -> u16{
        let pc = cpu_state.pc;

        let instruction: u16 = ((cpu_state.memory[pc] as u16) << 8) | (cpu_state.memory[pc+1] as u16);

        cpu_state.pc += 2;
        instruction
    }

    pub fn decode(instruction: u16) -> Option<Instruction>{

        //0xF000 - masks for bits, I want
        
        let opcode = ((instruction & 0xF000) >> 12) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;
        let n = (instruction & 0x000F) as u8;
        let nn = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        let di = DecodedInstruction{ opcode,x,y,n,nn,nnn };

        di.to_instruction()
    }


}



