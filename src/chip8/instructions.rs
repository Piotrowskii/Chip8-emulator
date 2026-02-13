use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::chip8::chip_8::{Display, Mode};
use crate::chip8::cpu_state::CpuState;
use crate::emulator::parameters::{DISPLAY_HEIGHT, DISPLAY_SIZE, DISPLAY_WIDTH, FONT_MEMORY_START};

#[derive(Debug)]
pub enum Instruction {
    I0000,
    I00BN{n: u8},
    I00CN{n: u8},
    I00DN{n: u8},
    I00E0,
    I00EE,
    I00FB,
    I00FC,
    I00FD,
    I00FE,
    I00FF,
    I1NNN{nnn: u16},
    I2NNN{nnn: u16},
    I3XNN{x: u8, nn: u8},
    I4XNN{x: u8, nn: u8},
    I5XY0{x: u8, y: u8},
    I5XY2{x: u8, y: u8},
    I5XY3{x: u8, y: u8},
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
    IDXY0{x: u8, y: u8},
    IEX9E{x: u8},
    IEXA1{x: u8},
    IF000,
    IFN01{n: u8},
    IF002,
    IFX07{x: u8},
    IFX15{x: u8},
    IFX18{x: u8},
    IFX1E{x: u8},
    IFX0A{x: u8},
    IFX29{x: u8},
    IFX30{x: u8},
    IFX33{x: u8},
    IFX3A{x: u8},
    IFX55{x: u8},
    IFX65{x: u8},
    IFX75{x: u8},
    IFX85{x: u8},
}


impl Instruction {
    pub fn execute(&self, cpu: &mut CpuState, display: &mut Display, keys: &[bool; 16], hires_mode: &AtomicBool, is_running: &AtomicBool) {
        //println!("{}", self);

        match *self {
            Instruction::I00BN {n}  | Instruction::I00DN {n} => {
                let n = if hires_mode.load(Ordering::Relaxed) {n as usize} else {(n * 2) as usize};

                display.execute_scroll(scroll_up, n);

                fn scroll_up(display: &mut [bool;DISPLAY_SIZE], n: usize){
                    for row in 0..DISPLAY_HEIGHT - n {
                        for col in 0..DISPLAY_WIDTH {
                            display[row * DISPLAY_WIDTH + col] = display[(row + n) * DISPLAY_WIDTH + col];
                        }
                    }

                    for row in (DISPLAY_HEIGHT - n..DISPLAY_HEIGHT).rev() {
                        for col in 0..DISPLAY_WIDTH {
                            display[row * DISPLAY_WIDTH + col] = false;
                        }
                    }
                }
            }
            Instruction::I00CN {n} => {
                let n = if hires_mode.load(Ordering::Relaxed) {n as usize} else {(n * 2) as usize};

                display.execute_scroll(scroll_down, n);
                fn scroll_down(display: &mut [bool;DISPLAY_SIZE], n: usize){
                    for row in (0..(DISPLAY_HEIGHT - n)).rev() {
                        for col in 0..DISPLAY_WIDTH {
                            display[(row + n) * DISPLAY_WIDTH + col] = display[row * DISPLAY_WIDTH + col];
                        }
                    }

                    for row in 0..n {
                        for col in 0..DISPLAY_WIDTH {
                            display[row * DISPLAY_WIDTH + col] = false;
                        }
                    }
                }
            }
            Instruction::I00E0 => {
                for plane in display.get_selected_planes(){
                    plane.fill(false);
                }
            },
            Instruction::I00EE => {
                if let Some(from_stack) = cpu.stack.pop() {
                    cpu.pc = from_stack as usize;
                } else {
                    println!("Stack underflow");
                }
            }
            Instruction::I00FB => {
                let n = if hires_mode.load(Ordering::Relaxed) {4} else {8};

                display.execute_scroll(scroll_right, n);

                fn scroll_right(display: &mut [bool;DISPLAY_SIZE], n: usize) {
                    for col in (0..DISPLAY_WIDTH - n).rev() {
                        for row in 0..DISPLAY_HEIGHT {
                            display[(col + n) + (DISPLAY_WIDTH * row)] = display[(col) + (DISPLAY_WIDTH * row)];
                        }
                    }

                    for col in 0..n{
                        for row in 0..DISPLAY_HEIGHT {
                            display[row * DISPLAY_WIDTH + col] = false;
                        }
                    }
                }
            },
            Instruction::I00FC => {
                let n = if hires_mode.load(Ordering::Relaxed) {4} else {8};

                display.execute_scroll(scroll_right, n);

                fn scroll_right(display: &mut [bool;DISPLAY_SIZE], n: usize) {
                    for col in (0 + n)..DISPLAY_WIDTH {
                        for row in 0..DISPLAY_HEIGHT {
                            display[(col - n) + (DISPLAY_WIDTH * row)] = display[(col) + (DISPLAY_WIDTH * row)];
                        }
                    }

                    for col in DISPLAY_WIDTH - n..DISPLAY_WIDTH {
                        for row in 0..DISPLAY_HEIGHT {
                            display[row * DISPLAY_WIDTH + col] = false;
                        }
                    }
                }
            },
            Instruction::I00FD | Instruction::I0000 => { is_running.store(false, Ordering::Relaxed)},
            Instruction::I00FE => { hires_mode.store(false, Ordering::Relaxed)}
            Instruction::I00FF => { hires_mode.store(true, Ordering::Relaxed)}
            Instruction::I1NNN {nnn} => { cpu.pc = nnn as usize; },
            Instruction::I2NNN {nnn} => {
                cpu.stack.push(cpu.pc as u16);
                cpu.pc = nnn as usize;
            },
            Instruction::I3XNN {x, nn} => {
                if cpu.registers[x as usize] == nn { cpu.skip_instruction() }
            }
            Instruction::I4XNN {x, nn} => {
                if cpu.registers[x as usize] != nn { cpu.skip_instruction() }
            }
            Instruction::I5XY0 {x, y} => {
                if cpu.registers[x as usize] == cpu.registers[y as usize] { cpu.skip_instruction() }
            }
            Instruction::I5XY2 {x, y} => {
                if x <= y {
                    for (idx,reg) in (x..=y).enumerate(){
                        cpu.memory[(cpu.i as usize) + idx] = cpu.registers[reg as usize]
                    }
                }else{
                    for (idx,reg) in (y..=x).rev().enumerate(){
                        cpu.memory[(cpu.i as usize) + idx] = cpu.registers[reg as usize]
                    }
                }
            }
            Instruction::I5XY3 {x, y} => {
                if x <= y {
                    for (idx,reg) in (x..=y).enumerate(){
                        cpu.registers[reg as usize] = cpu.memory[(cpu.i as usize) + idx]
                    }
                }else{
                    for (idx,reg) in (y..=x).rev().enumerate(){
                        cpu.registers[reg as usize] = cpu.memory[(cpu.i as usize) + idx]
                    }
                }
            }
            Instruction::I6XNN {x, nn} => { cpu.registers[x as usize] = nn }
            Instruction::I7XNN {x, nn} => {
                let (result, borrow) = cpu.registers[x as usize].overflowing_add(nn);

                cpu.registers[x as usize] = result;
            }
            Instruction::I8XY0 {x,y} => {
                cpu.registers[x as usize] = cpu.registers[y as usize];
            }
            Instruction::I8XY1 {x,y} => {
                cpu.registers[x as usize] = cpu.registers[x as usize] | cpu.registers[y as usize];
                if cpu.alt_8XY123 {cpu.registers[0xF] = 0};
            }
            Instruction::I8XY2 {x,y} => {
                cpu.registers[x as usize] = cpu.registers[x as usize] & cpu.registers[y as usize];
                if cpu.alt_8XY123 {cpu.registers[0xF] = 0};
            }
            Instruction::I8XY3 {x,y} => {
                cpu.registers[x as usize] = cpu.registers[x as usize] ^ cpu.registers[y as usize];
                if cpu.alt_8XY123 {cpu.registers[0xF] = 0};
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
                if !cpu.alt_8XY6_8XYE { cpu.registers[x as usize] = cpu.registers[y as usize] };

                let bit = (cpu.registers[x as usize] >> 0) & 1;
                cpu.registers[x as usize] >>= 1;
                cpu.registers[0xF] = bit;
            }
            Instruction::I8XY7 {x,y} => {
                let (result, borrow) = cpu.registers[y as usize].overflowing_sub(cpu.registers[x as usize]);

                cpu.registers[x as usize] = result;
                cpu.registers[0xF] = if borrow { 0 } else { 1 };
            }
            Instruction::I8XYE {x,y} => {
                if !cpu.alt_8XY6_8XYE { cpu.registers[x as usize] = cpu.registers[y as usize] };

                let bit = (cpu.registers[x as usize] & 0b_10000000) >> 7;
                cpu.registers[x as usize] <<= 1;
                cpu.registers[0xF] = bit;
            }
            Instruction::I9XY0 {x,y} => {
                if cpu.registers[x as usize] != cpu.registers[y as usize] { cpu.skip_instruction() }
            }
            Instruction::IANNN {nnn} => { cpu.i = nnn }
            Instruction::IBNNN {x, nnn} => {
                let value = if !cpu.alt_BNNN { nnn + cpu.registers[0x0] as u16 } else { nnn + cpu.registers[x as usize] as u16 };
                cpu.pc = value as usize;
            }
            Instruction::ICXNN {x,nn} => {
                let rand: u8 = rand::random_range(0..=255) & nn;
                cpu.registers[x as usize] = rand;
            }
            Instruction::IDXY0 {x,y} => {
                let scale: u8 = match hires_mode.load(Ordering::Relaxed){
                    true => 1,
                    false => 2,
                };
                let x = (cpu.registers[x as usize] as usize * (scale as usize)) % DISPLAY_WIDTH;
                let y = (cpu.registers[y as usize] as usize * (scale as usize)) % DISPLAY_HEIGHT;
                cpu.registers[0xF] = 0;


                let mut i = 0;
                for plane in display.get_selected_planes(){
                    let mut bytes: u16 = 0;
                    for row in 0..16u8*scale {
                        if row % scale == 0 {
                            bytes = ((cpu.memory[(cpu.i as usize + ((row/scale) * 2) as usize) + (i*32)] as u16) << 8) | (cpu.memory[(cpu.i as usize + (((row/scale) * 2)+ 1) as usize) + (i*32)] as u16);
                        }

                        for col in 0..16u8*scale {
                            let is_on = if (bytes >> (15 - (col/scale))) & 1 == 1 { true } else { false };
                            draw_pixel((x + col as usize ),(y + row as usize), is_on, cpu, plane);
                        }
                    }
                    i += 1;
                }
            }
            Instruction::IDXYN {x,y,n} => {
                let scale: u8 = match hires_mode.load(Ordering::Relaxed){
                    true => 1,
                    false => 2,
                };
                let x = (cpu.registers[x as usize] as usize * (scale as usize)) % DISPLAY_WIDTH;
                let y = (cpu.registers[y as usize] as usize * (scale as usize)) % DISPLAY_HEIGHT;
                cpu.registers[0xF] = 0;

                let mut i = 0;
                for plane in display.get_selected_planes(){
                    let mut byte: u8 = 0;
                    for row in 0..n*scale {
                        if row % scale == 0 {
                            byte = cpu.memory[(cpu.i as usize + (row/scale) as usize) + (i*(n as usize))];
                        }

                        for col in 0..8u8*scale {
                            let is_on = if (byte >> (7 - (col/scale))) & 1 == 1 { true } else { false };

                            draw_pixel((x + col as usize ),(y + row as usize), is_on, cpu, plane);
                        }
                    }
                    i += 1;
                }
            }
            Instruction::IEX9E {x} => {
                if keys[(cpu.registers[x as usize] as usize) & 0x0F] {
                    cpu.skip_instruction();
                }
            }
            Instruction::IEXA1 {x} => {
                if !keys[(cpu.registers[x as usize] as usize) & 0x0F] {
                    cpu.skip_instruction();
                }
            }
            Instruction::IF000 => {
                let nnnn: u16 = ((cpu.memory[cpu.pc] as u16) << 8) | (cpu.memory[cpu.pc+1] as u16);
                cpu.i = nnnn;

                cpu.pc += 2;
            }
            Instruction::IFN01 {n} => {
                display.selected_plane = n;
            }
            Instruction::IF002 => {
                for i in 0..16 {
                    cpu.sound_pattern_buffer[i] = cpu.memory[(cpu.i as usize) + i];
                }
            }
            Instruction::IFX07 {x} => { cpu.registers[x as usize] = cpu.delay_timer }
            Instruction::IFX0A {x} => {
                let pressed_key = keys.iter().position(|&pressed| pressed);

                if let Some(awaiting_key) = cpu.awaiting_key{
                    if keys[awaiting_key] == false {
                        cpu.registers[x as usize] = awaiting_key as u8;
                        cpu.awaiting_key = None;
                        cpu.pc +=2;
                    }
                }
                else if let Some(pressed_key) = pressed_key{
                    cpu.awaiting_key = Some(pressed_key);
                }

                cpu.pc -=2;
            }
            Instruction::IFX15 {x} => { cpu.delay_timer = cpu.registers[x as usize] }
            Instruction::IFX18 {x} => { cpu.sound_timer = cpu.registers[x as usize] }
            Instruction::IFX1E {x} => {
                let value = cpu.i.overflowing_add(cpu.registers[x as usize] as u16);

                cpu.i = value.0;
                if cpu.alt_IFX1E { cpu.registers[0xF] = if value.1 { 1 } else { 0 }};
            }
            Instruction::IFX29 {x} => {
                cpu.i = FONT_MEMORY_START as u16 + (cpu.registers[x as usize] as u16 * 5);
            }
            Instruction::IFX30 {x} => {
                cpu.i = 0xA0 + (cpu.registers[x as usize] as u16 * 10);
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
            Instruction::IFX3A {x} => {
                cpu.pitch_register = cpu.registers[x as usize];
            }
            Instruction::IFX55 {x} => {
                for i in 0..=x as usize {
                    cpu.memory[cpu.i as usize + i] = cpu.registers[i];
                }

                if cpu.alt_FX55_FX65{
                    cpu.i += x as u16 + 1;
                }
            }
            Instruction::IFX65 {x} => {
                for i in 0..=x as usize {
                    cpu.registers[i] = cpu.memory[cpu.i as usize + i];
                }

                if cpu.alt_FX55_FX65{
                    cpu.i += x as u16 + 1;
                }
            }
            Instruction::IFX75{x} => {
                for i in 0..=x as usize {
                    cpu.rpl_flags[i] = cpu.registers[i];
                }
            }
            Instruction::IFX85{x} => {
                for i in 0..=x as usize {
                    cpu.registers[i] = cpu.rpl_flags[i];
                }
            }
        }

        fn draw_pixel(x: usize, y: usize, is_on:bool, cpu: &mut CpuState, display: &mut [bool; DISPLAY_SIZE]) {
            //Clipping Logic
            if !cpu.alt_allow_scrolling{
                if y >= DISPLAY_HEIGHT { return; }
                if x >= DISPLAY_WIDTH { return; }
            }

            //Clipping Logic
            let position = &mut display[(y % DISPLAY_HEIGHT) * DISPLAY_WIDTH + (x) % DISPLAY_WIDTH];

            if is_on {
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

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::I0000 => {write!(f, "0000: Exit program")}
            Instruction::I00BN {n} => {write!(f, "00BN: Scroll up {} pixels", n)}
            Instruction::I00CN {n} => {write!(f, "00CN: Scroll down {} pixels", n)}
            Instruction::I00DN {n} => {write!(f, "00DN: Scroll up {} pixels", n)}
            Instruction::I00E0 => {write!(f, "00E0: Clear screen")}
            Instruction::I00EE => {write!(f, "00EE: PC = stack.pop")}
            Instruction::I00FB => {write!(f, "00FB: Scroll right 4 pixels")}
            Instruction::I00FC => {write!(f, "00FC: Scroll left 4 pixels")}
            Instruction::I00FD => {write!(f, "00FD: Exit interpreter")}
            Instruction::I00FE => {write!(f, "00FE: Disable hi-res mode")}
            Instruction::I00FF => {write!(f, "00FF: Enable hi-res mode")}
            Instruction::I1NNN {nnn} => {write!(f, "1NNN: PC = {}", nnn)}
            Instruction::I2NNN {nnn} => {write!(f, "2NNN: Push pc to stack, PC = {}", nnn)}
            Instruction::I3XNN {x,nn} => {write!(f, "3XNN: PC += 2 if V{} == {}",x ,nn)}
            Instruction::I4XNN {x,nn} => {write!(f, "4XNN: PC += 2 if V{} != {}",x ,nn)}
            Instruction::I5XY0 {x,y} => {write!(f, "5XY0: PC += 2 if V{} == V{}",x ,y)}
            Instruction::I5XY2 {x,y} => {write!(f, "5XY2: memory[..] = V{}..V{}",x ,y)}
            Instruction::I5XY3 {x,y} => {write!(f, "5XY3: V{}..V{} = memory[..]",x ,y)}
            Instruction::I6XNN {x,nn} => {write!(f, "6XNN: V{} = {}", x, nn)}
            Instruction::I7XNN {x,nn} => {write!(f, "7XNN: V{} += {}", x, nn)}
            Instruction::I8XY0 {x,y} => {write!(f, "8XY0: V{} = V{}",x ,y)}
            Instruction::I8XY1 {x,y} => {write!(f, "8XY1: V{} = V{} | V{}",x, x ,y)}
            Instruction::I8XY2 {x,y} => {write!(f, "8XY2: V{} = V{} & V{}",x, x ,y)}
            Instruction::I8XY3 {x,y} => {write!(f, "8XY3: V{} = V{} ^ V{}",x, x ,y)}
            Instruction::I8XY4 {x,y} => {write!(f, "8XY4: V{} = V{} + V{}",x, x ,y)}
            Instruction::I8XY5 {x,y} => {write!(f, "8XY5: V{} = V{} - V{}",x, x ,y)}
            Instruction::I8XY6 {x,y} => {write!(f, "8XY6: V{} >> 1",x)}
            Instruction::I8XY7 {x,y} => {write!(f, "8XY7: V{} = V{} | V{}", x, y ,x)}
            Instruction::I8XYE {x,y} => {write!(f, "8XYE: V{} << 1",x)}
            Instruction::I9XY0 {x,y} => {write!(f, "9XY0: PC += 2 if V{} != V{}",x ,y)}
            Instruction::IANNN {nnn} => {write!(f, "ANNN: I = {}",nnn)}
            Instruction::IBNNN {x,nnn} => {write!(f,"BNNN: PC = ({} + V0 OR {} + V{})",nnn,nnn,x)}
            Instruction::ICXNN {x,nn} => {write!(f, "CXNN: V{} = Rnd & {}",x,nn)}
            Instruction::IDXYN {x,y,n} => {write!(f, "DXYN: Display x: {} y:{} n:{}",x,y,n)}
            Instruction::IDXY0 {x,y} => {write!(f, "DXY0: Display x: {} y:{}",x,y)}
            Instruction::IEX9E {x} => {write!(f, "EX9E: PC += 2 if key == V{}",x)}
            Instruction::IEXA1 {x} => {write!(f, "EXA1: PC += 2 if key != V{}",x)}
            Instruction::IF000 => {write!(f, "IF000: i = nnnn")}
            Instruction::IFN01 {n} => {write!(f, "IFN01: Changing plane to {}",n)}
            Instruction::IF002 => {write!(f, "IF002: audio_buffer[1..16] = memory[i..i+15]")}
            Instruction::IFX07 {x} => {write!(f, "FX07: V{} = delay timer",x)}
            Instruction::IFX15 {x} => {write!(f, "FX15: Delay timer = V{}",x)}
            Instruction::IFX18 {x} => {write!(f, "FX18: Sound timer = V{}",x)}
            Instruction::IFX1E {x} => {write!(f, "FX1E: I += V{}",x)}
            Instruction::IFX0A {x} => {write!(f, "FX0A: Wait for key, V{} = key",x)}
            Instruction::IFX29 {x} => {write!(f, "FX29: I = font character V{}",x)}
            Instruction::IFX30 {x} => {write!(f, "FX30: I = big font character V{}", x)}
            Instruction::IFX33 {x} => {write!(f, "FX33: mem[i..i+2] = 1,2,3 (np. V{} = 123)",x)}
            Instruction::IFX3A {x} => {write!(f, "FX3A: pitch_register = V{}", x)}
            Instruction::IFX55 {x} => {write!(f, "FX55: mem[i..i+x] = V0..={}  ",x)}
            Instruction::IFX65 {x} => {write!(f, "FX65:  V0..={} = mem[i..i+x] ",x)}
            Instruction::IFX75 {x} => {write!(f, "FX75:  rpl[i..i+x] = V0..={} ",x)}
            Instruction::IFX85 {x} => {write!(f, "FX85:  V0..={} = pl[i..i+x]",x)}
        }
    }
}