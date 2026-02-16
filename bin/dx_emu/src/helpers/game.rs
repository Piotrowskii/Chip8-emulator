use chip8_lib::chip_8::Mode;

#[derive(Copy, Clone, PartialEq)]
pub struct Colors{
    pub plane1: &'static str,
    pub plane2: &'static str,
    pub mixed: &'static str,
    pub none: &'static str,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Game{
    pub name: &'static str,
    pub bytes: &'static [u8],
    pub mode: Mode,
    pub colors: Colors,
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
            }
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
            }
        }
    }
}