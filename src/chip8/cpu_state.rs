pub struct CpuState {
    //pub memory: [u8; 4096],
    pub memory: [u8; 65_536],
    pub pc: usize,
    pub i: u16,
    pub stack: Vec<u16>,
    pub registers: [u8; 16], // named V0 through VF , VF - is a carry flag
    pub rpl_flags: [u8; 8],
    pub delay_timer: u8,
    pub sound_timer: u8,
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
            registers: [0; 16],
            rpl_flags: [0; 8],
            alt_8XY6_8XYE: false,
            alt_BNNN: false,
            alt_FX55_FX65: false,
            alt_8XY123: false,
            alt_IFX1E: false,
            alt_allow_scrolling: false,
        }
    }
}