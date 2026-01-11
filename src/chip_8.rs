use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{fs, thread};
use std::time::{Duration, Instant};
use crate::parameters::{DISPLAY_HEIGHT, DISPLAY_WIDTH, FONT_DATA};

pub enum KeyPad{
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    A, B, C, D, E, F,
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
}

pub struct Chip8{
    pub state: Arc<Mutex<CpuState>>,
    pub display: Arc<Mutex<[bool; DISPLAY_HEIGHT * DISPLAY_WIDTH]>>,
    pub running: Arc<AtomicBool>
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
            })),
            display: Arc::new(Mutex::new([false; 64 * 32])),
            running: Arc::new(AtomicBool::new(true))
        }
    }

    pub fn get_new_and_start() -> Chip8{
        let mut chip8 = Chip8::new();
        chip8.start();
        chip8
    }

    fn start(&mut self){
        self.load_font_into_memory();
        self.load_cartridge("roms/test_opcode.ch8");
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
        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                let start = Instant::now();

                let mut cpu_state = state.lock().unwrap();
                let mut display = display.lock().unwrap();

                let instruction = {
                    Self::fetch(&mut cpu_state)
                };

                let decoded_instruction = Self::decode(instruction);

                Self::execute(decoded_instruction, &mut cpu_state, &mut display);


                let elapsed = start.elapsed().as_nanos() as u64;
                thread::sleep(Duration::from_nanos(1_430_000u64.saturating_sub(elapsed) ));
            }
        });
    }

    pub fn handle_input(&mut self, key: KeyPad){

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

    pub fn execute(di: DecodedInstruction, cpu_state: &mut CpuState, display: &mut [bool; 2048]){
        match di {
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xE, n: 0x0, ..} => { display.fill(false) },
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xE, n: 0xE, ..} => {
                if let Some(from_stack) = cpu_state.stack.pop(){
                    cpu_state.pc = from_stack as usize;
                }
                else{
                    println!("Stack underflow");
                }
            }
            DecodedInstruction {opcode: 0x1, ..} => { cpu_state.pc = di.nnn as usize },
            DecodedInstruction {opcode: 0x2, ..} => {
                cpu_state.stack.push(cpu_state.pc as u16);
                cpu_state.pc = di.nnn as usize;
            },
            DecodedInstruction {opcode: 0x6, ..} => { cpu_state.registers[di.x as usize] = di.nn }
            DecodedInstruction {opcode: 0x7, ..} => {
                let result = cpu_state.registers[di.x as usize].overflowing_add(di.nn);

                cpu_state.registers[di.x as usize] = result.0;
                cpu_state.registers[0xF] = if result.1 { 1 } else { 0 };
            }
            DecodedInstruction {opcode: 0xA, ..} => { cpu_state.i = di.nnn }
            DecodedInstruction {opcode: 0xC, ..} => {
                let rand: u8 = rand::random_range(0..=di.nn) & di.nn;
                cpu_state.registers[di.x as usize] = rand;
            }
            DecodedInstruction {opcode: 0xD, ..} => {
                let x = cpu_state.registers[di.x as usize] % DISPLAY_WIDTH as u8;
                let y = cpu_state.registers[di.y as usize] % DISPLAY_HEIGHT as u8;
                cpu_state.registers[0xF] = 0;

                for row in 0..di.n{
                    let byte = cpu_state.memory[cpu_state.i as usize + row as usize];

                    for col in  0..8u8 {
                        let is_on = if (byte >> (7 - col)) & 1 == 1 {true} else {false};
                        let position = &mut display[(y as usize + row as usize) * DISPLAY_WIDTH + (x as usize + col as usize)];

                        if(is_on){
                            if(*position){
                                *position = !*position;
                                cpu_state.registers[0xF] = 1;
                            }
                            else{
                                *position = true;
                            }
                        }
                    }
                }

            }
            DecodedInstruction {opcode: 0xF, y: 0x0, n: 0x7, ..} => { cpu_state.registers[di.x as usize] = cpu_state.delay_timer }
            DecodedInstruction {opcode: 0xF, y: 0x1, n: 0x5, ..} => { cpu_state.delay_timer = cpu_state.registers[di.x as usize] }
            DecodedInstruction {opcode: 0xF, y: 0x1, n: 0x8, ..} => { cpu_state.sound_timer = cpu_state.registers[di.x as usize] }
            DecodedInstruction {opcode: 0xF, y: 0x1, n: 0xE, ..} => { cpu_state.i += cpu_state.registers[di.x as usize] as u16}
                _ => {}
        }
    }


}



