use std::sync::{Arc, Mutex};
use dioxus::prelude::*;
use chip8_lib::display::Display;
use chip8_lib::parameters::*;

#[component]
pub fn EmuDisplay(display: Display) -> Element{
    rsx! {
        svg {
            width: "100%",
            view_box: "0 0 128 64",
            for y in 0..DISPLAY_HEIGHT {
                for x in 0..DISPLAY_WIDTH {
                    rect {
                        x: "{x}",
                        y: "{y}",
                        width: "1",
                        height: "1",
                        fill: get_pixel_color(&display, (y * DISPLAY_WIDTH) + x)
                    }
                }
            }
        }
    }
}

fn get_pixel_color(display: &Display, idx: usize) -> String{
    if display.plane_1[idx] && display.plane_2[idx]{
        "#EEEEFF".to_string()
    }
    else if display.plane_2[idx]{
        "#456543".to_string()
    }
    else if display.plane_1[idx]{
        "#554422".to_string()
    }
    else{
        "#87CEEB".to_string()
    }
}