use std::sync::{Arc, Mutex};
use dioxus::prelude::*;
use chip8_lib::display::Display;
use chip8_lib::parameters::*;
use crate::helpers::game::{Colors, Game};

#[component]
pub fn EmuDisplay(display: Display, game: Option<Game>, focused: bool) -> Element{
    rsx! {
        div{
            class: "relative w-full",
            div{
                svg {
                class: "rounded-md",
                shape_rendering: "crispEdges",
                width: "100%",
                view_box: "0 0 128 64",
                    for y in 0..DISPLAY_HEIGHT {
                        for x in 0..DISPLAY_WIDTH {
                            rect {
                                x: "{x}",
                                y: "{y}",
                                width: "1",
                                height: "1",
                                fill: get_pixel_color(&display, (y * DISPLAY_WIDTH) + x, &game)
                            }
                        }
                    }
                }
            }
            if game.is_none(){
                div{
                    class: "absolute top-0 left-0 w-full h-full bg-base-100 opacity-90 rounded-sm text-5xl text-primary flex items-center justify-center text-center",
                    "Select game and start emulator"
                }
            }
            else if !focused{
                div{
                    class: "absolute top-0 left-0 w-full h-full bg-base-100 opacity-90 rounded-sm text-8xl text-primary flex flex-col items-center justify-center text-center",
                    p { "Game paused" }
                    p {
                        class: "text-3xl text-secondary",
                        "Focus in emulator to unpause"
                    }
                }
            }
        }
    }
}

fn get_pixel_color(display: &Display, idx: usize, game: &Option<Game>) -> String{
    if let Some(game) = game {
        if display.plane_1[idx] && display.plane_2[idx]{
            game.colors.mixed.to_string()
        }
        else if display.plane_2[idx]{
            game.colors.plane2.to_string()
        }
        else if display.plane_1[idx]{
            game.colors.plane1.to_string()
        }
        else{
            game.colors.none.to_string()
        }
    }else{
        "#000000".to_string()
    }
}