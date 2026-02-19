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
pub struct ControlsGroup{
    pub name: &'static str,
    pub controls: Vec<(KeyPad, &'static str)>
}

//authors - https://github.com/JohnEarnest/chip8Archive/blob/master/programs.json
#[derive(Clone, PartialEq)]
pub struct Author{
    pub name: &'static str,
    pub url: Option<&'static str>,
}

#[derive(Clone, PartialEq)]
pub struct Game{
    pub name: &'static str,
    pub bytes: &'static [u8],
    pub mode: Mode,
    pub colors: Colors,
    pub instructions: Vec<ControlsGroup>,
    pub author: Option<Author>,
}

impl Game{
    pub fn available_games() -> Vec<Game>{
        vec![Game::t8nks(), Game::br8kout(),  Game::chiken_scratch(), Game::octopeg(), Game::horsey_jump()]
    }
    pub fn get_all_controls(&self) -> Vec<(KeyPad, &'static str)>{
        self.instructions.iter().flat_map(|group| group.controls.iter().cloned()).collect()
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
                ControlsGroup{
                    name: "Steering",
                    controls: vec![
                        (KeyPad:: Num7, "Left"),
                        (KeyPad:: Num9, "Right"),
                    ]
                }
            ],
            author: Some(Author{
                name: "SharpenedSpoon",
                url: None
            })
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
                ControlsGroup{
                    name: "Steering",
                    controls: vec![
                        (KeyPad:: Num5, "Up"),
                        (KeyPad:: Num8, "Down"),
                        (KeyPad:: Num7, "Left"),
                        (KeyPad:: Num9, "Right"),
                    ]
                },
                ControlsGroup{
                    name: "Weapons",
                    controls: vec![
                        (KeyPad:: Num1, "Regular"),
                        (KeyPad:: Num2, "Mirv"),
                        (KeyPad:: Num3, "Nuke"),
                    ]
                },
                ControlsGroup{
                    name: "Actions",
                    controls: vec![
                        (KeyPad:: Num6, "Shoot"),
                    ]
                },
            ],
            author: None,
        }
    }

    pub fn chiken_scratch() -> Game{
        Game{
            name: "Chicken Scratch",
            bytes: include_bytes!("../../assets/roms/chickenScratch.ch8"),
            mode: Mode::XoChip,
            colors: Colors{
                plane1: "#8B4000",
                plane2: "#FFAC1C",
                mixed: "#EEEEFF",
                none: "#FAD5A5"
            },
            instructions: vec![
                ControlsGroup{
                    name: "Steering",
                    controls: vec![
                        (KeyPad:: Num5, "Up"),
                        (KeyPad:: Num8, "Down"),
                        (KeyPad:: Num7, "Left"),
                        (KeyPad:: Num9, "Right"),
                    ]
                },
                ControlsGroup{
                    name: "Actions",
                    controls: vec![
                        (KeyPad:: Num6, "Peck / Space"),
                    ]
                },
            ],
            author: Some(Author{
                name: "JohnEarnest",
                url: Some("https://github.com/JohnEarnest")
            })
        }
    }

    pub fn octopeg() -> Game{
        Game{
            name: "Octopeg",
            bytes: include_bytes!("../../assets/roms/octopeg.ch8"),
            mode: Mode::Chip8,
            colors: Colors{
                plane1: "#acd5ff",
                plane2: "#FF6600",
                mixed: "#662200",
                none: "#113152"
            },
            instructions: vec![
                ControlsGroup{
                    name: "Steering",
                    controls: vec![
                        (KeyPad:: Num5, "Up"),
                        (KeyPad:: Num8, "Down"),
                        (KeyPad:: Num7, "Left"),
                        (KeyPad:: Num9, "Right"),
                    ]
                },
                ControlsGroup{
                    name: "Actions",
                    controls: vec![
                        (KeyPad:: Num6, "Shoot"),
                    ]
                },
            ],
            author: Some(Author{
                name: "Chromatophore",
                url: Some("https://github.com/Chromatophore")
            })
        }
    }

    pub fn horsey_jump() -> Game{
        Game{
            name: "Horsey Jump",
            bytes: include_bytes!("../../assets/roms/horseyJump.ch8"),
            mode: Mode::SuperChip,
            colors: Colors{
                plane1: "#FFFFFF",
                plane2: "#000000",
                mixed: "#000000",
                none: "#000000"
            },
            instructions: vec![
                ControlsGroup{
                    name: "Actions",
                    controls: vec![
                        (KeyPad:: Num0, "Jump"),
                    ]
                },
            ],
            author: Some(Author{
                name: "LarissaR",
                url: None
            })
        }
    }
}