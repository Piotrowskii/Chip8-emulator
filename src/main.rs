pub mod emulator;
mod chip_8;
mod parameters;
mod square_wave;

use crate::emulator::Emulator;

pub fn main() -> Result<(), String> {
    let mut emulator = Emulator::new();
    emulator.run();

    Ok(())
}