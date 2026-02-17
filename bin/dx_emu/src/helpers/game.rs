use dioxus::prelude::*;
use chip8_lib::chip_8::Mode;
use chip8_lib::keypad::KeyPad;

#[derive(Copy, Clone, PartialEq)]
pub struct Colors{
    pub plane1: &'static str,
    pub plane2: &'static str,
    pub mixed: &'static str,
    pub none: &'static str,
}

#[derive(Clone, PartialEq)]
pub struct Game{
    pub name: &'static str,
    pub bytes: &'static [u8],
    pub mode: Mode,
    pub colors: Colors,
    pub instructions: Vec<(KeyPad, &'static str)>
}

impl Game{
    pub fn available_games() -> Vec<Game>{
        vec![Game::br8kout(), Game::t8nks()]
    }
    
    pub fn br8kout() -> Game{
        Game{
            name: "Br8kout",
            bytes: include_bytes!("../../assets/roms/br8kout.ch8"),
            mode: Mode::Chip8,
            colors: Colors{
                plane1: "#FFFFFF",
                plane2: "#000000",
                mixed: "#000000",
                none: "#000000"
            },
            instructions: vec![
                (KeyPad:: Num7, "Left"),
                (KeyPad:: Num9, "Right"),
            ]
        }
    }
    
    pub fn t8nks() -> Game{
        Game{
            name: "T8NKS",
            bytes: include_bytes!("../../assets/roms/t8nks.ch8"),
            mode: Mode::XoChip,
            colors: Colors{
                plane1: "#554422",
                plane2: "#456543",
                mixed: "#EEEEFF",
                none: "#87CEEB"
            },
            instructions: vec![
                (KeyPad:: Num5, "Up"),
                (KeyPad:: Num8, "Down"),
                (KeyPad:: Num7, "Left"),
                (KeyPad:: Num9, "Right"),
                (KeyPad:: Num1, "Regular"),
                (KeyPad:: Num2, "Mirv"),
                (KeyPad:: Num3, "Nuke"),
                (KeyPad:: Num6, "Shoot"),
            ]
        }
    }
}