use crate::chip8::instructions::Instruction;
#[derive(Debug)]
pub struct DecodedInstruction{
    pub opcode: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: u16
}

impl DecodedInstruction {
    pub fn to_instruction(&self) -> Option<Instruction> {
        /*I00FB,
        I00FC,*/
        match *self{
            //DecodedInstruction {opcode: 0x0, x: 0x0, y: 0x0, n: 0x0, ..} => Some(Instruction::I0000),
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xB, n, ..} => Some(Instruction::I00BN {n}),
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xD, n, ..} => Some(Instruction::I00DN {n}),
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xC, n, ..} => Some(Instruction::I00CN {n}),
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xE, n: 0x0, ..} => Some(Instruction::I00E0),
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xE, n: 0xE, ..} => Some(Instruction::I00EE),
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xF, n: 0xB, ..} => Some(Instruction::I00FB),
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xF, n: 0xC, ..} => Some(Instruction::I00FC),
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xF, n: 0xD, ..} => Some(Instruction::I00FD),
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xF, n: 0xE, ..} => Some(Instruction::I00FE),
            DecodedInstruction {opcode: 0x0, x: 0x0, y: 0xF, n: 0xF, ..} => Some(Instruction::I00FF),
            DecodedInstruction {opcode: 0x1, nnn, ..} => Some(Instruction::I1NNN {nnn}),
            DecodedInstruction {opcode: 0x2, nnn, ..} => Some(Instruction::I2NNN {nnn}),
            DecodedInstruction {opcode: 0x3, x, nn, ..} => Some(Instruction::I3XNN {x,nn}),
            DecodedInstruction {opcode: 0x4, x, nn, ..} => Some(Instruction::I4XNN {x,nn}),
            DecodedInstruction {opcode: 0x5, n: 0x0, x,y, ..} => Some(Instruction::I5XY0 {x,y}),
            DecodedInstruction {opcode: 0x5, n: 0x2, x,y, ..} => Some(Instruction::I5XY2 {x,y}),
            DecodedInstruction {opcode: 0x5, n: 0x3, x,y, ..} => Some(Instruction::I5XY3 {x,y}),
            DecodedInstruction {opcode: 0x6, x,nn, ..} => Some(Instruction::I6XNN {x,nn}),
            DecodedInstruction {opcode: 0x7, x,nn,..} => Some(Instruction::I7XNN{x,nn}),
            DecodedInstruction {opcode: 0x8, n: 0x0, x, y, .. } => Some(Instruction::I8XY0 {x,y}),
            DecodedInstruction {opcode: 0x8, n: 0x1, x, y, .. } => Some(Instruction::I8XY1 {x,y}),
            DecodedInstruction {opcode: 0x8, n: 0x2, x, y, .. } => Some(Instruction::I8XY2 {x,y}),
            DecodedInstruction {opcode: 0x8, n: 0x3, x, y, .. } => Some(Instruction::I8XY3 {x,y}),
            DecodedInstruction {opcode: 0x8, n: 0x4, x, y, .. } => Some(Instruction::I8XY4 {x,y}),
            DecodedInstruction {opcode: 0x8, n: 0x5, x, y, .. } => Some(Instruction::I8XY5 {x,y}),
            DecodedInstruction {opcode: 0x8, n: 0x6, x, y, .. } => Some(Instruction::I8XY6 {x,y}),
            DecodedInstruction {opcode: 0x8, n: 0x7, x, y, .. } => Some(Instruction::I8XY7 {x,y}),
            DecodedInstruction {opcode: 0x8, n: 0xE, x, y, .. } => Some(Instruction::I8XYE {x,y}),
            DecodedInstruction {opcode: 0x9, n: 0x0, x, y, ..} => Some(Instruction::I9XY0 {x,y}),
            DecodedInstruction {opcode: 0xA, nnn, ..} => Some(Instruction::IANNN {nnn}),
            DecodedInstruction {opcode: 0xB, x, nnn, ..} => Some(Instruction::IBNNN {x, nnn}),
            DecodedInstruction {opcode: 0xC, x, nn, ..} => Some(Instruction::ICXNN {x,nn}),
            DecodedInstruction {opcode: 0xD, n: 0x0, x, y, ..} => Some(Instruction::IDXY0 {x,y}),
            DecodedInstruction {opcode: 0xD, x, y, n, ..} => Some(Instruction::IDXYN {x,y,n}),
            DecodedInstruction {opcode: 0xE, y: 0x9, n: 0xE, x, ..} => Some(Instruction::IEX9E {x}),
            DecodedInstruction {opcode: 0xE, y: 0xA, n: 0x1, x, ..} => Some(Instruction::IEXA1 {x}),
            DecodedInstruction {opcode: 0xF, x: 0x0, y: 0x0, n: 0x0, ..} => Some(Instruction::IF000),
            DecodedInstruction {opcode: 0xF, y: 0x0, n: 0x1, x, ..} => Some(Instruction::IFN01 {n:x}),
            DecodedInstruction {opcode: 0xF, x: 0x0, y: 0x0, n: 0x2, ..} => Some(Instruction::IF002),
            DecodedInstruction {opcode: 0xF, y: 0x0, n: 0x7, x, ..} => Some(Instruction::IFX07 {x}),
            DecodedInstruction {opcode: 0xF, y: 0x0, n: 0xA, x, ..} => Some(Instruction::IFX0A {x}),
            DecodedInstruction {opcode: 0xF, y: 0x1, n: 0x5, x, ..} => Some(Instruction::IFX15 {x}),
            DecodedInstruction {opcode: 0xF, y: 0x1, n: 0x8, x, ..} => Some(Instruction::IFX18 {x}),
            DecodedInstruction {opcode: 0xF, y: 0x1, n: 0xE, x, ..} => Some(Instruction::IFX1E {x}),
            DecodedInstruction {opcode: 0xF, y: 0x2, n: 0x9, x, ..} => Some(Instruction::IFX29 {x}),
            DecodedInstruction {opcode: 0xF, y: 0x3, n: 0x0, x, ..} => Some(Instruction::IFX30 {x}),
            DecodedInstruction {opcode: 0xF, y: 0x3, n: 0x3, x, ..} => Some(Instruction::IFX33 {x}),
            DecodedInstruction {opcode: 0xF, y: 0x3, n: 0xA, x, ..} => Some(Instruction::IFX3A {x}),
            DecodedInstruction {opcode: 0xF, y: 0x5, n: 0x5, x, ..} => Some(Instruction::IFX55 {x}),
            DecodedInstruction {opcode: 0xF, y: 0x6, n: 0x5, x, ..} => Some(Instruction::IFX65 {x}),
            DecodedInstruction {opcode: 0xF, y: 0x7, n: 0x5, x, ..} => Some(Instruction::IFX75 {x}),
            DecodedInstruction {opcode: 0xF, y: 0x8, n: 0x5, x, ..} => Some(Instruction::IFX85 {x}),
            _ => {
                println!("{:?} Instruction not found", self);
                None
            }
        }
    }
}

