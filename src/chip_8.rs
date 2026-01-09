use std::fmt::{Binary, LowerHex};

pub enum KeyPad{
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    A, B, C, D, E, F,
}

pub struct Chip8{
    memory: [u8; 4096],
    pub display: [bool; 64 * 32],
    pc: usize,
    i: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    pub sound_timer: u8,
    registers: [u8; 16] // named V0 through VF
}

impl Chip8{
    pub fn new() -> Chip8{
        let mut chip8 = Chip8{
            memory: [0; 4096],
            display: [false; 64 * 32],
            pc: 0,
            i: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
        };

        chip8.load_font_into_memory();
        chip8
    }

    pub fn load_font_into_memory(&mut self){
        for i in 0..80usize{
            self.memory[i + 80] = FONT_DATA[i];
        }
    }

    pub fn push_to_stack(&mut self, address: u16){
        self.stack.push(address);
    }

    pub fn pop_from_stack(&mut self) -> Option<u16>{
        self.stack.pop()
    }

    pub fn decrement_timers(&mut self){
        if self.delay_timer > 0{
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0{
            self.sound_timer -= 1;
        }
    }

    pub fn handle_input(&mut self, key: KeyPad){

    }

    fn get_nibble_from_u16(number: u16, nibble_number: usize){
        for i in 0..16i32{
            let multiplayer = 2.pow(i);
        }
    }

    pub fn fetch(&mut self) -> u16{
        let instruction: u16 = self.memory[self.pc] as u16 + self.memory[self.pc + 1] as u16;
        self.pc += 2;
        instruction
    }

    pub fn decode(&mut self, instruction: u16) -> u16{
        let first_nibble = instruction.();

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


