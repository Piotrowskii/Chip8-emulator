use dioxus::core::Element;
use dioxus::core_macro::rsx;
use dioxus::hooks::{use_signal};
use chip8_lib::display::Display;
use dioxus::prelude::*;
use crate::components::EmuDisplay;
use crate::helpers::chip8_wrapper::Chip8Web;
use crate::helpers::game::Game;
use crate::{KEYBOARD_EVENTS, SHOW_KEYBOARD};

#[component]
pub fn Emulator() -> Element{
    let mut display_signal = use_signal(|| Display::new());
    let mut paused = use_signal(|| false);
    let mut selected_game = use_signal(|| Game::br8kout());
    let mut active_game: Signal<Option<Game>> = use_signal(|| None);
    let mut chip8: Signal<Option<Chip8Web>> = use_signal(|| None);
    let mut show_keyboard = SHOW_KEYBOARD.signal().clone();



    let keyboard_event = KEYBOARD_EVENTS.signal().clone();
    let mut chip8_signal = chip8.clone();
    use_effect(move || {
        if let Some((key, pressed)) = keyboard_event.read().as_ref(){
            handle_key_press(&mut chip8_signal, key, *pressed);
        }
    });


    rsx! {
        div{
            tabindex: "0",
            autofocus: true,
            class: "focus:outline-hidden",
            div {
                class: "flex justify-center",
                EmuDisplay {
                    display: display_signal(),
                    game: active_game(),
                    paused: paused()
                }
            }
            div{
                class: "flex flex-row justify-center items-end gap-2 md:gap-4",
                select{
                    class: "select select-primary text-xl flex-6",
                    value: "{selected_game().name}",
                    onchange: move |event| {
                        let value = event.value().clone();
                        if let Some(game) = Game::available_games().into_iter().find(|game| game.name == value){
                            selected_game.set(game);
                        }
                    },
                    for game in Game::available_games(){
                        option{
                            class: "text-xl",
                            value: "{game.name}",
                            "{game.name}",
                        }
                    }
                }
                button{
                    onclick: move |_| start_emu(&mut chip8, &selected_game.peek(), &mut display_signal, &mut active_game, &mut paused),
                    class: "btn btn-primary mt-5 text-xl font-thin flex-2",
                    "Start"
                }
                button{
                    onclick: move |_| pause_resume_emu(&mut chip8, &mut paused),
                    class: "btn btn-primary mt-5 text-xl flex-1",
                    if paused(){
                        span{
                            class: "text-3xl",
                            "▸"
                        }
                    }else{
                        span{
                            class: "text-xl",
                            "⏸︎"
                        }
                    }
                }
                button{
                    class: "btn btn-primary mt-5 text-xl flex-1 lg:hidden",
                    class: if show_keyboard(){ "btn-secondary" },
                    onclick: move |_| {
                        let current_value = *show_keyboard.peek();
                        show_keyboard.set(!current_value);
                    },
                    "⌨️"
                }

            }
        }
    }
}

pub fn start_emu(chip8_signal: &mut Signal<Option<Chip8Web>>, game: &Game, display_signal: &mut Signal<Display>, selected_game: &mut Signal<Option<Game>>, paused: &mut Signal<bool>) {
    if let Some(chip8) = chip8_signal.write().as_mut(){
        chip8.stop();
        display_signal.set(Display::new());
    }

    let mut new_chip8 = Chip8Web::new(game.mode);
    new_chip8.start(&game, display_signal);
    chip8_signal.set(Some(new_chip8));
    selected_game.set(Some(*game));

    paused.set(false);
}

pub fn pause_resume_emu(chip8_signal: &mut Signal<Option<Chip8Web>>, paused: &mut Signal<bool>) {
    if let Some(chip8) = chip8_signal.write().as_mut(){
        let is_paused = *paused.peek();

        if is_paused{
            chip8.resume();
        }else{
            chip8.pause();
        }

        paused.set(!is_paused);
    }
}

pub fn handle_key_press(chip8: &mut Signal<Option<Chip8Web>>, key: &String, pressed: bool){
    if let Some(chip8) = chip8.write().as_mut() {
        chip8.handle_key_press(key, pressed);
    }
}