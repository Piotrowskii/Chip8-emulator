use crate::chip8::cpu_state::CpuState;
use crate::emulator::parameters::{DISPLAY_HEIGHT, DISPLAY_SIZE, DISPLAY_WIDTH};

pub enum Instruction {
    I00E0,
    I00EE,
    I1NNN{nnn: u16},
    I2NNN{nnn: u16},
    I3XNN{x: u8, nn: u8},
    I4XNN{x: u8, nn: u8},
    I5XY0{x: u8, y: u8},
    I6XNN{x: u8, nn: u8},
    I7XNN{x: u8, nn: u8},
    I8XY0{x: u8, y: u8},
    I8XY1{x: u8, y: u8},
    I8XY2{x: u8, y: u8},
    I8XY3{x: u8, y: u8},
    I8XY4{x: u8, y: u8},
    I8XY5{x: u8, y: u8},
    I8XY6{x: u8, y: u8},
    I8XY7{x: u8, y: u8},
    I8XYE{x: u8, y: u8},
    I9XY0{x: u8, y: u8},
    IANNN{nnn: u16},
    IBNNN{x: u8, nnn: u16},
    ICXNN{x: u8, nn: u8},
    IDXYN{x: u8, y: u8, n: u8},
    IEX9E{x: u8},
    IEXA1{x: u8},
    IFX07{x: u8},
    IFX15{x: u8},
    IFX18{x: u8},
    IFX1E{x: u8},
    IFX0A{x: u8},
    IFX29{x: u8},
    IFX33{x: u8},
    IFX55{x: u8},
    IFX65{x: u8},
}

impl Instruction {
    pub fn execute(&self, cpu: &mut CpuState, display: &mut [bool; 2048], keys: &[bool; 16]) {
        match *self {
            Instruction::I00E0 => { display.fill(false) },
            Instruction::I00EE => {
                if let Some(from_stack) = cpu.stack.pop() {
                    cpu.pc = from_stack as usize;
                } else {
                    println!("Stack underflow");
                }
            }
            Instruction::I1NNN {nnn} => { cpu.pc = nnn as usize; },
            Instruction::I2NNN {nnn} => {
                cpu.stack.push(cpu.pc as u16);
                cpu.pc = nnn as usize;
            },
            Instruction::I3XNN {x, nn} => {
                if cpu.registers[x as usize] == nn { cpu.pc += 2; }
            }
            Instruction::I4XNN {x, nn} => {
                if cpu.registers[x as usize] != nn { cpu.pc += 2; }
            }
            Instruction::I5XY0 {x, y} => {
                if cpu.registers[x as usize] == cpu.registers[y as usize] { cpu.pc += 2; }
            }
            Instruction::I6XNN {x, nn} => { cpu.registers[x as usize] = nn }
            Instruction::I7XNN {x, nn} => {
                let result = cpu.registers[x as usize].overflowing_add(nn);

                cpu.registers[x as usize] = result.0;
                cpu.registers[0xF] = if result.1 { 1 } else { 0 };
            }
            Instruction::I8XY0 {x,y} => {
                cpu.registers[x as usize] = cpu.registers[y as usize];
            }
            Instruction::I8XY1 {x,y} => {
                cpu.registers[x as usize] = cpu.registers[x as usize] | cpu.registers[y as usize];
            }
            Instruction::I8XY2 {x,y} => {
                cpu.registers[x as usize] = cpu.registers[x as usize] & cpu.registers[y as usize];
            }
            Instruction::I8XY3 {x,y} => {
                cpu.registers[x as usize] = cpu.registers[x as usize] ^ cpu.registers[y as usize];
            }
            Instruction::I8XY4 {x,y} => {
                let result = cpu.registers[x as usize].overflowing_add(cpu.registers[y as usize]);

                cpu.registers[x as usize] = result.0;
                cpu.registers[0xF] = if result.1 { 1 } else { 0 };
            }
            Instruction::I8XY5 {x,y} => {
                let result = cpu.registers[x as usize].overflowing_sub(cpu.registers[y as usize]);

                cpu.registers[x as usize] = result.0;
                cpu.registers[0xF] = if result.1 { 0 } else { 1 };
            }
            Instruction::I8XY6 {x,y} => {
                if !cpu.ins_8XY6_and_8XYE_alt { cpu.registers[x as usize] = cpu.registers[y as usize] };

                let bit = (cpu.registers[x as usize] >> 0) & 1;
                cpu.registers[x as usize] >>= 1;
                cpu.registers[0xF] = bit;
            }
            Instruction::I8XY7 {x,y} => {
                let result = cpu.registers[y as usize].overflowing_sub(cpu.registers[x as usize]);

                cpu.registers[x as usize] = result.0;
                cpu.registers[0xF] = if result.1 { 0 } else { 1 };
            }
            Instruction::I8XYE {x,y} => {
                if !cpu.ins_8XY6_and_8XYE_alt { cpu.registers[x as usize] = cpu.registers[y as usize] };

                let bit = (cpu.registers[x as usize] & 0b_10000000) >> 7;
                cpu.registers[x as usize] <<= 1;
                cpu.registers[0xF] = bit;
            }
            Instruction::I9XY0 {x,y} => {
                if cpu.registers[x as usize] != cpu.registers[y as usize] { cpu.pc += 2; }
            }
            Instruction::IANNN {nnn} => { cpu.i = nnn }
            Instruction::IBNNN {x, nnn} => {
                let value = if !cpu.ins_BNNN_alt { nnn } else { nnn + cpu.registers[x as usize] as u16 };
                cpu.pc = value as usize;
            }
            Instruction::ICXNN {x,nn} => {
                let rand: u8 = rand::random_range(0..=255) & nn;
                cpu.registers[x as usize] = rand;
            }
            Instruction::IDXYN {x,y,n} => {
                let x = cpu.registers[x as usize] % DISPLAY_WIDTH as u8;
                let y = cpu.registers[y as usize] % DISPLAY_HEIGHT as u8;
                cpu.registers[0xF] = 0;

                for row in 0..n {
                    let byte = cpu.memory[cpu.i as usize + row as usize];

                    for col in 0..8u8 {
                        let is_on = if (byte >> (7 - col)) & 1 == 1 { true } else { false };
                        let position = &mut display[(y as usize + row as usize) * DISPLAY_WIDTH + (x as usize + col as usize)];

                        if (is_on) {
                            if (*position) {
                                *position = !*position;
                                cpu.registers[0xF] = 1;
                            } else {
                                *position = true;
                            }
                        }
                    }
                }
            }
            Instruction::IEX9E {x} => {
                if keys[cpu.registers[x as usize] as usize] {
                    cpu.pc += 2;
                }
            }
            Instruction::IEXA1 {x} => {
                if !keys[cpu.registers[x as usize] as usize] {
                    cpu.pc += 2;
                }
            }
            Instruction::IFX07 {x} => { cpu.registers[x as usize] = cpu.delay_timer }
            Instruction::IFX0A {x} => {
                let pressed_key = keys.iter().position(|&pressed| pressed);
                if let Some(pressed_key) = pressed_key {
                    cpu.registers[x as usize] = pressed_key as u8;
                } else {
                    cpu.pc -= 2;
                }
            }
            Instruction::IFX15 {x} => { cpu.delay_timer = cpu.registers[x as usize] }
            Instruction::IFX18 {x} => { cpu.sound_timer = cpu.registers[x as usize] }
            Instruction::IFX1E {x} => {
                let value = cpu.i.overflowing_add(cpu.registers[x as usize] as u16);

                cpu.i = value.0;
                cpu.registers[0xF] = if value.1 { 1 } else { 0 };
            }
            Instruction::IFX29 {x} => {
                cpu.i = 80 + (cpu.registers[x as usize] as u16 * 5);
            }
            Instruction::IFX33 {x} => {
                let value = cpu.registers[x as usize];
                let hundred = value / 100;
                let tens = (value % 100) / 10;
                let ones = value % 10;

                cpu.memory[cpu.i as usize] = hundred;
                cpu.memory[cpu.i as usize + 1] = tens;
                cpu.memory[cpu.i as usize + 2] = ones;
            }
            Instruction::IFX55 {x} => {
                if !cpu.ins_FX55_and_FX65_alt {
                    for i in 0..=x as usize {
                        cpu.memory[cpu.i as usize + i] = cpu.registers[i];
                    }
                } else {
                    for i in 0..=x as usize {
                        cpu.memory[cpu.i as usize] = cpu.registers[i];
                        cpu.i += 1;
                    }
                }
            }
            Instruction::IFX65 {x} => {
                if !cpu.ins_FX55_and_FX65_alt {
                    for i in 0..=x as usize {
                        cpu.registers[i] = cpu.memory[cpu.i as usize + i];
                    }
                } else {
                    for i in 0..=x as usize {
                        cpu.registers[i] = cpu.memory[cpu.i as usize];
                        cpu.i += 1;
                    }
                }
            }
        }
    }

}