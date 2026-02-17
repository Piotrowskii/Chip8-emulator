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
                class: "text-center w-full flex flex-row flex-wrap gap-3 justify-center",
                for instruction in game.instructions{
                    p{
                        class: "text-xl flex flex-row gap-2",
                        span{
                            class: "kbd text-xl",
                            {instruction.0.to_keyboard_str()}
                        }
                        " - {instruction.1}"
                    }
                }
            }
        }
    ]
}