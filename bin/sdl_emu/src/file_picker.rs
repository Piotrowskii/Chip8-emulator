use std::path::PathBuf;
use rfd::FileDialog;

pub fn pick_file() -> Option<PathBuf>{
    let file = FileDialog::new()
        .add_filter("rom", &["ch8", "schip", "xo8"])
        .set_directory("/")
        .pick_file();
    file
}