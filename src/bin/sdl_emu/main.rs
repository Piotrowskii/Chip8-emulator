mod emulator;
mod pattern_wave;
mod square_wave;
mod audio_manager;

use rfd::FileDialog;
use crate::emulator::Emulator;

pub fn main() -> Result<(), String> {
    let files = FileDialog::new()
        .add_filter("rom", &["ch8", "schip", "xo8"])
        .set_directory("/")
        .pick_file();

    if let Some(file) = files {
        let mut emulator = Emulator::new(file);
        emulator.run();
    }

    Ok(())
}
