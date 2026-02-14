mod emulator;
mod sound;
mod file_picker;

use rfd::FileDialog;
use crate::emulator::Emulator;
use crate::file_picker::pick_file;

pub fn main() -> Result<(), String> {
    let file = file_picker::pick_file();

    if let Some(file) = file {
        let mut emulator = Emulator::new(file);
        emulator.run();
    }

    Ok(())
}
