use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use crate::parameters::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub enum KeyPad{
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    A, B, C, D, E, F,
}

pub struct Chip8{
    memory: Arc<Mutex<[u8; 4096]>>,
    pub display: Arc<Mutex<[bool; DISPLAY_HEIGHT * DISPLAY_WIDTH]>>,
    pc: Arc<Mutex<usize>>,
    i: Arc<Mutex<u16>>,
    stack: Arc<Mutex<Vec<u16>>>,
    delay_timer: Arc<Mutex<u8>>,
    pub sound_timer: Arc<Mutex<u8>>,
    registers: Arc<Mutex<[u8; 16]>>, // named V0 through VF
    pub running: Arc<AtomicBool>
}

impl Chip8{
    pub fn new() -> Chip8{
        Chip8{
            memory: Arc::new(Mutex::new([0; 4096])),
            display: Arc::new(Mutex::new([false; 64 * 32])),
            pc: Arc::new(Mutex::new(0)),
            i: Arc::new(Mutex::new(0)),
            stack: Arc::new(Mutex::new(vec![])),
            delay_timer: Arc::new(Mutex::new(0)),
            sound_timer: Arc::new(Mutex::new(0)),
            registers: Arc::new(Mutex::new([0; 16])),
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
        self.start_timer_thread();
        self.start_execution_thread();
    }

    pub fn load_font_into_memory(&mut self){
        let memory_arc = Arc::clone(&self.memory);
        let mut memory = memory_arc.lock().unwrap();

        for i in 0..80usize{
            memory[i + 80] = FONT_DATA[i];
        }
    }

    fn start_timer_thread(&mut self) {
        let delay_timer = Arc::clone(&self.delay_timer);
        let sound_timer = Arc::clone(&self.sound_timer);
        let running = Arc::clone(&self.running);

        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                let start = Instant::now();

                {
                    let mut delay_value = delay_timer.lock().unwrap();
                    *delay_value = delay_value.saturating_sub(1);
                }
                {
                    let mut sound_value = sound_timer.lock().unwrap();
                    *sound_value = sound_value.saturating_sub(1);
                }

                let elapsed = start.elapsed().as_nanos() as u64;
                thread::sleep(Duration::from_nanos(16_666_667u64.saturating_sub(elapsed) ));
            }
        });
    }

    //700 Hz
    fn start_execution_thread(&mut self) {

        let running = Arc::clone(&self.running);
        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                let start = Instant::now();


                let elapsed = start.elapsed().as_nanos() as u64;
                thread::sleep(Duration::from_nanos(1_430_000u64.saturating_sub(elapsed) ));
            }
        });
    }

    pub fn handle_input(&mut self, key: KeyPad){

    }
    

    pub fn fetch(memory: &mut [u8; 4096], pc: &mut usize) -> u16{
        let instruction: u16 = memory[*pc] as u16 + memory[*pc + 1] as u16;
        *pc += 2;
        instruction
    }

    pub fn decode(&mut self, instruction: u16) -> u16{

        //0xF000 - masks for bits, I want
        
        let first_nibble = (instruction & 0xF000) >> 12;
        let second_nibble = (instruction & 0x0F00) >> 8;
        let third_nibble = (instruction & 0x00F0) >> 4;
        let fourth_nibble = instruction & 0x000F;

        0
    }


}

const FONT_DATA: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];


