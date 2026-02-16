use dioxus::core::Element;
use dioxus::core_macro::rsx;
use dioxus::hooks::{use_future, use_signal};
use gloo_timers::future::TimeoutFuture;
use chip8_lib::display::Display;
use dioxus::prelude::*;
use chip8_lib::chip_8::Mode::Chip8;
use crate::components::EmuDisplay;
use crate::helpers::chip8_wrapper::Chip8Web;
use crate::helpers::game::Game;

#[component]
pub fn Emulator() -> Element{
    let mut display_signal = use_signal(|| Display::new());
    let mut in_focus = use_signal(|| true);
    let mut selected_game = use_signal(|| Game::br8kout());
    let mut active_game: Signal<Option<Game>> = use_signal(|| None);
    let mut chip8: Signal<Option<Chip8Web>> = use_signal(|| None);

    rsx! {
        div{
            tabindex: "0",
            autofocus: true,
            class: "focus:outline-hidden",
            onkeydown: move |event| {
                let key = event.key();
                handle_key_press(&mut chip8, key, true);
            },
            onkeyup: move |event| {
                let key = event.key();
                handle_key_press(&mut chip8, key, false);
            },
            onfocusout: move |event| {
                if let Some(chip8) = chip8.write().as_mut(){
                    chip8.clear_keys();
                    chip8.pause();
                }
                in_focus.set(false);
            },
            onfocusin: move |event| {
                if let Some(chip8) = chip8.write().as_mut(){
                    chip8.resume();
                }
                in_focus.set(true);
            },
            div {
                class: "flex justify-center",
                EmuDisplay {
                    display: display_signal(),
                    game: active_game(),
                    focused: in_focus()
                }
            }
            div{
                class: "flex flex-row justify-center items-end gap-4",
                select{
                    class: "select select-primary text-xl",
                    for game in Game::available_games(){
                        option{
                            class: "text-xl",
                            onclick: move |_| selected_game.set(game),
                            "{game.name}",
                        }
                    }
                }
                button{
                    onclick: move |_| start_emu(&mut chip8, &selected_game.peek(), &mut display_signal, &mut active_game, in_focus()),
                    class: "btn btn-primary mt-5 text-xl",
                    "Start emulator"
                }
            }
        }
    }
}

pub fn start_emu(chip8_signal: &mut Signal<Option<Chip8Web>>, game: &Game, display_signal: &mut Signal<Display>, selected_game: &mut Signal<Option<Game>>, focused: bool) {
    if let Some(chip8) = chip8_signal.write().as_mut(){
        chip8.stop();
        display_signal.set(Display::new());
    }

    let mut new_chip8 = Chip8Web::new(game.mode);
    if !focused{
        new_chip8.pause();
    }
    new_chip8.start(&game, display_signal);
    chip8_signal.set(Some(new_chip8));
    selected_game.set(Some(*game));
}

pub fn handle_key_press(chip8: &mut Signal<Option<Chip8Web>>, key: Key, pressed: bool){
    if let Some(chip8) = chip8.write().as_mut() {
        chip8.handle_key_press(key, pressed);
    }
}