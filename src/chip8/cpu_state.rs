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