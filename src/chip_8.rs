use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::{fs, thread};
use std::time::{Duration, Instant};
use crate::parameters::{DISPLAY_HEIGHT, DISPLAY_WIDTH, FONT_DATA};

#[repr(u8)]
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

pub struct DecodedInstruction{
    opcode: u8,
    x: u8,
    y: u8,
    n: u8,
    nn: u8,
    nnn: u16
}

pub struct CpuState {
    pub memory: [u8; 4096],
    pub pc: usize,
    pub i: u16,
    pub stack: Vec<u16>,
    pub registers: [u8; 16], // named V0 through VF , VF - is a carry flag
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub ins_8XY6_and_8XYE_alt: bool,
    pub ins_BNNN_alt: bool,
    pub ins_FX55_and_FX65_alt: bool
}

pub struct Chip8{
    pub state: Arc<Mutex<CpuState>>,
    pub display: Arc<Mutex<[bool; DISPLAY_HEIGHT * DISPLAY_WIDTH]>>,
    pub running: Arc<AtomicBool>,
    pub pressed_key: Arc<AtomicU8>
}

impl Chip8{
    pub fn new() -> Chip8{
        Chip8{
            state: Arc::new(Mutex::new(CpuState{
                memory: [0; 4096],
                pc: 0x200,
                i: 0,
                stack: vec![],
                delay_timer: 0,
                sound_timer: 0,
                registers: [0; 16],
                ins_8XY6_and_8XYE_alt: false,
                ins_BNNN_alt: false,
                ins_FX55_and_FX65_alt: false,
            })),
            display: Arc::new(Mutex::new([false; 64 * 32])),
            running: Arc::new(AtomicBool::new(true)),
            pressed_key: Arc::new(AtomicU8::new(0)),
        }
    }

    pub fn get_new_and_start() -> Chip8{
        let mut chip8 = Chip8::new();
        chip8.start();
        chip8
    }

    fn start(&mut self){
        self.load_font_into_memory();
        self.load_cartridge("roms/flightrunner.ch8");
        self.start_timer_thread();
        self.start_execution_thread();
    }

    pub fn load_font_into_memory(&self){
        let mut state = self.state.lock().unwrap();

        for i in 0..80usize{
            state.memory[i + 80] = FONT_DATA[i];
        }
    }

    pub fn load_cartridge(&mut self, rom_path: &str){
        let mut state = self.state.lock().unwrap();

        let rom = fs::read(rom_path);
        if let Ok(rom) = rom{
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
                    cpu_state.delay_timer = cpu_state.delay_timer.saturating_sub(1);
                    cpu_state.sound_timer = cpu_state.sound_timer.saturating_sub(1);
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
        let pressed_key = Arc::clone(&self.pressed_key);
        
        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                let start = Instant::now();

                {
                    let mut cpu_state = state.lock().unwrap();
                    let mut display = display.lock().unwrap();
                    let pressed_key: &AtomicU8 = &pressed_key;
    
                    let instruction = {
                        Self::fetch(&mut cpu_state)
                    };
    
                    let decoded_instruction = Self::decode(instruction);
    
                    Self::execute(decoded_instruction, &mut cpu_state, &mut display, &pressed_key);
                }

                let elapsed = start.elapsed().as_nanos() as u64;
                thread::sleep(Duration::from_nanos(1_430_000u64.saturating_sub(elapsed) ));
            }
        });
    }

    pub fn handle_input(&mut self, key: KeyPad, pressed: bool){
        let current_key = self.pressed_key.load(Ordering::Relaxed);
        if pressed{
            self.pressed_key.store(key as u8, Ordering::Relaxed);
        }
        else{
            if key as u8 == current_key{
                self.pressed_key.store(0, Ordering::Relaxed);
            }
        }
    }
    

    pub fn fetch(cpu_state: &mut CpuState) -> u16{
        let pc = cpu_state.pc;

        let instruction: u16 = ((cpu_state.memory[pc] as u16) << 8) | (cpu_state.memory[pc+1] as u16);

        cpu_state.pc += 2;
        instruction
    }

    pub fn decode(instruction: u16) -> DecodedInstruction{

        //0xF000 - masks for bits, I want
        
        let opcode = ((instruction & 0xF000) >> 12) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;
        let n = (instruction & 0x000F) as u8;
        let nn = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        DecodedInstruction{
            opcode,x,y,n,nn,nnn
        }
    }

    pub fn execute(di: DecodedInstruction, cpu: &mut CpuState, display: &mut [bool; 2048], pressed_key: &AtomicU8){
        match di {
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xE, n: 0x0, ..} => { display.fill(false) },
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xE, n: 0xE, ..} => {
                if let Some(from_stack) = cpu.stack.pop(){
                    cpu.pc = from_stack as usize;
                }
                else{
                    println!("Stack underflow");
                }
            }
            DecodedInstruction {opcode: 0x1, ..} => { cpu.pc = di.nnn as usize },
            DecodedInstruction {opcode: 0x2, ..} => {
                cpu.stack.push(cpu.pc as u16);
                cpu.pc = di.nnn as usize;
            },
            DecodedInstruction {opcode: 0x3, ..} => {
                if cpu.registers[di.x as usize] == di.nn { cpu.pc += 2; }
            }
            DecodedInstruction {opcode: 0x4, ..} => {
                if cpu.registers[di.x as usize] != di.nn { cpu.pc += 2; }
            }
            DecodedInstruction {opcode: 0x5, n: 0x0, ..} => {
                if cpu.registers[di.x as usize] == cpu.registers[di.y as usize]{ cpu.pc += 2; }
            }
            DecodedInstruction {opcode: 0x6, ..} => { cpu.registers[di.x as usize] = di.nn }
            DecodedInstruction {opcode: 0x7, ..} => {
                let result = cpu.registers[di.x as usize].overflowing_add(di.nn);

                cpu.registers[di.x as usize] = result.0;
                cpu.registers[0xF] = if result.1 { 1 } else { 0 };
            }
            DecodedInstruction {opcode: 0x8, n: 0x0, .. } => {
                cpu.registers[di.x as usize] = cpu.registers[di.y as usize];
            }
            DecodedInstruction {opcode: 0x8, n: 0x1, .. } => {
                cpu.registers[di.x as usize] = cpu.registers[di.x as usize] | cpu.registers[di.y as usize];
            }
            DecodedInstruction {opcode: 0x8, n: 0x2, .. } => {
                cpu.registers[di.x as usize] = cpu.registers[di.x as usize] & cpu.registers[di.y as usize];
            }
            DecodedInstruction {opcode: 0x8, n: 0x3, .. } => {
                cpu.registers[di.x as usize] = cpu.registers[di.x as usize] ^ cpu.registers[di.y as usize];
            }
            DecodedInstruction {opcode: 0x8, n: 0x4, .. } => {
                let result = cpu.registers[di.x as usize].overflowing_add(cpu.registers[di.y as usize]);

                cpu.registers[di.x as usize] = result.0;
                cpu.registers[0xF] = if result.1 {1} else {0};
            }
            DecodedInstruction {opcode: 0x8, n: 0x5, .. } => {
                let result = cpu.registers[di.x as usize].overflowing_sub(cpu.registers[di.y as usize]);

                cpu.registers[di.x as usize] = result.0;
                cpu.registers[0xF] = if result.1 {1} else {0};
            }
            DecodedInstruction {opcode: 0x8, n: 0x6, .. } => {
                if !cpu.ins_8XY6_and_8XYE_alt { cpu.registers[di.x as usize] = cpu.registers[di.y as usize] };

                cpu.registers[0xF] = (cpu.registers[di.x as usize] >> 0) & 1;
                cpu.registers[di.x as usize] = cpu.registers[di.x as usize] >> 1;
            }
            DecodedInstruction {opcode: 0x8, n: 0x7, .. } => {
                let result = cpu.registers[di.y as usize].overflowing_sub(cpu.registers[di.x as usize]);

                cpu.registers[di.x as usize] = result.0;
                cpu.registers[0xF] = if result.1 {1} else {0};
            }
            DecodedInstruction {opcode: 0x8, n: 0xE, .. } => {
                if !cpu.ins_8XY6_and_8XYE_alt { cpu.registers[di.x as usize] = cpu.registers[di.y as usize] };

                cpu.registers[0xF] = (cpu.registers[di.x as usize] << 0) & 1;
                cpu.registers[di.x as usize] = cpu.registers[di.x as usize] << 1;
            }
            DecodedInstruction {opcode: 0x9, n: 0x0, ..} => {
                if cpu.registers[di.x as usize] != cpu.registers[di.y as usize]{ cpu.pc += 2; }
            }
            DecodedInstruction {opcode: 0xA, ..} => { cpu.i = di.nnn }
            DecodedInstruction {opcode: 0xB, ..} => {
                let value = if !cpu.ins_BNNN_alt {di.nnn} else {di.nnn + cpu.registers[di.x as usize] as u16};
                cpu.pc = value as usize;
            }
            DecodedInstruction {opcode: 0xC, ..} => {
                let rand: u8 = rand::random_range(0..=di.nn) & di.nn;
                cpu.registers[di.x as usize] = rand;
            }
            DecodedInstruction {opcode: 0xD, ..} => {
                let x = cpu.registers[di.x as usize] % DISPLAY_WIDTH as u8;
                let y = cpu.registers[di.y as usize] % DISPLAY_HEIGHT as u8;
                cpu.registers[0xF] = 0;

                for row in 0..di.n{
                    let byte = cpu.memory[cpu.i as usize + row as usize];

                    for col in  0..8u8 {
                        let is_on = if (byte >> (7 - col)) & 1 == 1 {true} else {false};
                        let position = &mut display[(y as usize + row as usize) * DISPLAY_WIDTH + (x as usize + col as usize)];

                        if(is_on){
                            if(*position){
                                *position = !*position;
                                cpu.registers[0xF] = 1;
                            }
                            else{
                                *position = true;
                            }
                        }
                    }
                }

            }
            DecodedInstruction {opcode: 0xE, y: 0x9, n: 0xE, ..} => {
                if pressed_key.load(Ordering::Relaxed) == cpu.registers[di.x as usize] {
                    cpu.pc += 2;
                }
            }
            DecodedInstruction {opcode: 0xE, y: 0xA, n: 0x1, ..} => {
                if pressed_key.load(Ordering::Relaxed) != cpu.registers[di.x as usize] {
                    cpu.pc += 2;
                }
            }
            DecodedInstruction {opcode: 0xF, y: 0x0, n: 0x7, ..} => { cpu.registers[di.x as usize] = cpu.delay_timer }
            DecodedInstruction {opcode: 0xF, y: 0x0, n: 0xA, ..} => {
                let key = pressed_key.load(Ordering::Relaxed);
                if(key == 0){
                    cpu.pc -= 2;
                }
                else{
                    cpu.registers[di.x as usize] = key;
                }
            }
            DecodedInstruction {opcode: 0xF, y: 0x1, n: 0x5, ..} => { cpu.delay_timer = cpu.registers[di.x as usize] }
            DecodedInstruction {opcode: 0xF, y: 0x1, n: 0x8, ..} => { cpu.sound_timer = cpu.registers[di.x as usize] }
            DecodedInstruction {opcode: 0xF, y: 0x1, n: 0xE, ..} => {
                let value = cpu.i.overflowing_add(cpu.registers[di.x as usize] as u16);

                cpu.i = value.0;
                cpu.registers[0xF] = if value.1 { 1 } else { 0 };
            }
            DecodedInstruction {opcode: 0xF, y: 0x2, n: 0x9, ..} => {
                cpu.i = 80 + (cpu.registers[di.x as usize] as u16 * 5);
            }
            DecodedInstruction {opcode: 0xF, y: 0x3, n: 0x3, ..} => {
                let value = cpu.registers[di.x as usize];
                let hundred = value / 100;
                let tens = (value % 100) / 10;
                let ones = value % 10;

                cpu.memory[cpu.i as usize] = hundred;
                cpu.memory[cpu.i as usize + 1] = tens;
                cpu.memory[cpu.i as usize + 2] = ones;
            }
            DecodedInstruction {opcode: 0xF, y: 0x5, n: 0x5, ..} => {
                if !cpu.ins_FX55_and_FX65_alt {
                    for i in 0..=di.x as usize {
                        cpu.memory[cpu.i as usize + i] = cpu.registers[i];
                    }
                }
                else{
                    for i in 0..=di.x as usize {
                        cpu.memory[cpu.i as usize] = cpu.registers[i];
                        cpu.i += 1;
                    }
                }

            }
            DecodedInstruction {opcode: 0xF, y: 0x6, n: 0x5, ..} => {
                if !cpu.ins_FX55_and_FX65_alt {
                    for i in 0..=di.x as usize {
                        cpu.registers[i] = cpu.memory[cpu.i as usize + i];
                    }
                }
                else{
                    for i in 0..=di.x as usize {
                        cpu.registers[i] = cpu.memory[cpu.i as usize];
                        cpu.i += 1;
                    }
                }

            }
                _ => {}
        }
    }


}



