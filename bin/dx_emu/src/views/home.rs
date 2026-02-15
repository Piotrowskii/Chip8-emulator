use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use chip8_lib::chip_8::KeyPad;
use chip8_lib::display::Display;
use crate::{BREAKOUT, CHIP8, T8NKS};
use crate::components::EmuDisplay;
use crate::helpers::chip8_wrapper::Chip8Web;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let mut number = use_signal(|| 1);
    let is_running = use_signal(|| false);
    let mut display_signal = use_signal(|| Display::new());

    use_future(move || {
        async move {
            loop {
                if let Some(display) = CHIP8.peek().get_display_copy(){
                    display_signal.set(display);
                }

                // Use 16ms for 60fps, or 32ms for 30fps
                TimeoutFuture::new(16).await;
            }
        }
    });

    rsx! {
        div{
            class: "w-2/5 p-5 mx-auto flex flex-col justify-center",
            div {
                tabindex: "0",
                autofocus: true,
                onkeydown: move |event| {
                    let key = event.key();
                    handle_key_press(key, true);
                },
                onkeyup: move |event| {
                    let key = event.key();
                    handle_key_press(key, false);
                },
                class: "flex justify-center",
                EmuDisplay {
                    display: display_signal()
                }
            }
            button{
                onclick: move |_| start_emu(),
                class: "btn btn-primary mt-5",
                "Start emulator"
            }
            button{
                onclick: move |_| number += 1,
                class: "btn btn-primary mt-5",
                "Start emulator"
            }
            p{
                "{number}"
            }
        }
    }
}

pub fn start_emu(){
    let chip_8 = &mut CHIP8.write();
    chip_8.start(T8NKS);
}

pub fn handle_key_press(key: Key, pressed: bool){
    if let Some(key) = get_keypad(&key){
        CHIP8.write().chip8.handle_input(key, pressed);
    }
}

fn get_keypad(key: &Key) -> Option<KeyPad> {
    match key {
        Key::Character(c) => match c.to_lowercase().as_str() {
            "1" => Some(KeyPad::Num1),
            "2" => Some(KeyPad::Num2),
            "3" => Some(KeyPad::Num3),
            "4" => Some(KeyPad::C),
            "q" => Some(KeyPad::Num4),
            "w" => Some(KeyPad::Num5),
            "e" => Some(KeyPad::Num6),
            "r" => Some(KeyPad::D),
            "a" => Some(KeyPad::Num7),
            "s" => Some(KeyPad::Num8),
            "d" => Some(KeyPad::Num9),
            "f" => Some(KeyPad::E),
            "z" => Some(KeyPad::A),
            "x" => Some(KeyPad::Num0),
            "c" => Some(KeyPad::B),
            "v" => Some(KeyPad::F),
            _ => None,
        },
        _ => None,
    }
}