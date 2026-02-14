use crate::chip_8::Mode;
use crate::decoded_instruction::DecodedInstruction;
use crate::instructions::Instruction;

pub struct CpuState {
    pub memory: [u8; 65_536],
    pub pc: usize,
    pub i: u16,
    pub stack: Vec<u16>,
    pub registers: [u8; 16], // named V0 through VF , VF - is a carry flag
    pub rpl_flags: [u8; 8],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub sound_pattern_buffer: [u8; 16],
    pub pitch_register: u8,
    pub awaiting_key: Option<usize>,
    pub alt_8XY6_8XYE: bool, 
    pub alt_BNNN: bool,
    pub alt_FX55_FX65: bool,
    pub alt_8XY123: bool,
    pub alt_IFX1E: bool,
    pub alt_allow_scrolling: bool,
}

impl Default for CpuState {
    fn default() -> CpuState {
        CpuState{
            memory: [0; 65_536],
            pc: 0x200,
            i: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            sound_pattern_buffer: [0; 16],
            pitch_register: 64, //64 = 4000 HZ, 4000*2^((vx-64)/48)
            registers: [0; 16],
            rpl_flags: [0; 8],
            awaiting_key: None,
            alt_8XY6_8XYE: false,
            alt_BNNN: false,
            alt_FX55_FX65: false,
            alt_8XY123: false,
            alt_IFX1E: false,
            alt_allow_scrolling: false,
        }
    }
}


impl CpuState{
    pub fn set_compatibility_mode(&mut self, mode: &Mode){
        match mode {
            Mode::Chip8 => {
                self.alt_8XY123 = true;
                self.alt_FX55_FX65 = true;
            }
            Mode::SuperChip => {
                self.alt_allow_scrolling = false;
                self.alt_8XY6_8XYE = true;
                self.alt_BNNN = true;
            }
            Mode::XoChip => {
                self.alt_allow_scrolling = true;
                self.alt_FX55_FX65 = true;
            }
            Mode::Experimental => {
                self.alt_FX55_FX65 = false;
                self.alt_allow_scrolling = true;
            }
        }
    }

    pub fn fetch(&mut self) -> u16{
        let pc = self.pc;

        let instruction: u16 = ((self.memory[pc] as u16) << 8) | (self.memory[pc+1] as u16);

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

    pub fn get_current_instruction(&mut self, increment_pc: bool) -> Option<Instruction> {
        let instruction = self.fetch();
        if increment_pc{
            self.pc += 2;
        }
        Self::decode(instruction)
    }

    pub fn skip_instruction(&mut self) {
        if let Some(instruction) = self.get_current_instruction(false) {
            match instruction{
                Instruction::IF000 => self.pc += 4,
                _ => self.pc +=2
            }
        }
        else{
            println!("Error skipping instruction");
        }
    }
}