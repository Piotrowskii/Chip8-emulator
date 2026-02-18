use dioxus::prelude::*;
use crate::helpers::chip8_wrapper::Chip8Web;
use crate::helpers::game::Game;
use crate::SHOW_KEYBOARD;

#[component]
pub fn Instructions(game: Game) -> Element{
    rsx![
        div{
            class: "flex flex-col items-center mt-4",
            p{
                class: "text-3xl mb-2",
                "Controls"
            }
            div{
                class: "text-center w-full flex flex-col gap-3 justify-center",
                for control_group in game.instructions{
                    div{
                        class: "flex flex-row flex-wrap",
                        p{
                            class: "text-xl flex flex-row flex-wrap gap-2 whitespace-nowrap",
                            "{control_group.name}: ",
                            for (key, action) in control_group.controls{
                                div{
                                    class: "kbd whitespace-nowrap ",
                                    span{
                                        class: "text-primary text-xl",
                                        "{key.to_keyboard_str()}"
                                    }
                                    span{
                                        class: "text-xl ml-2",
                                        "- {action}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    ]
}