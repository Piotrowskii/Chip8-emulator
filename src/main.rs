pub mod emulator;
mod chip_8;
mod parameters;
mod square_wave;

use rfd::FileDialog;
use crate::chip_8::Chip8;
use crate::emulator::Emulator;

pub fn main() -> Result<(), String> {
    let files = FileDialog::new()
        .add_filter("rom", &["ch8", "schip"])
        .set_directory("/")
        .pick_file();

    if let Some(file) = files {
        let chip8 = Chip8::get_new_and_start(file);
        let mut emulator = Emulator::new(chip8);
        emulator.run();
    }

    Ok(())
}