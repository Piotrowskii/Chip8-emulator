use dioxus::core::Element;
use dioxus::core_macro::rsx;
use dioxus::hooks::{use_signal};
use chip8_lib::display::Display;
use dioxus::prelude::*;
use crate::components::{EmuDisplay, Footer, Instructions, MobileKeyboard};
use crate::helpers::chip8_wrapper::Chip8Web;
use crate::helpers::game::Game;
use crate::{KEYBOARD_EVENTS, SHOW_KEYBOARD};

#[component]
pub fn Emulator() -> Element{
    let mut display_signal = use_signal(|| Display::new());
    let mut paused_signal = use_signal(|| false);
    let mut selected_game_signal = use_signal(|| Game::t8nks());
    let mut active_game_signal: Signal<Option<Game>> = use_signal(|| None);
    let mut chip8_signal: Signal<Option<Chip8Web>> = use_signal(|| None);
    let mut show_keyboard_signal = SHOW_KEYBOARD.signal().clone();


    let mut start_emu = move |game: &Game|{
        if let Some(chip8) = chip8_signal.write().as_mut(){
            chip8.stop();
            display_signal.set(Display::new());
        }

        let mut new_chip8 = Chip8Web::new(game.mode);
        new_chip8.start(&game, &mut display_signal);
        chip8_signal.set(Some(new_chip8));
        active_game_signal.set(Some(game.clone()));

        paused_signal.set(false);
    };

    let mut pause_resume_emu = move ||{
        if let Some(chip8) = chip8_signal.write().as_mut(){
            let is_paused = *paused_signal.peek();

            if is_paused{
                chip8.resume();
            }else{
                chip8.pause();
            }

            paused_signal.set(!is_paused);
        }
    };

    let mut handle_key_press = move |key: &String, pressed: bool|{
        if let Some(chip8) = chip8_signal.write().as_mut() {
            chip8.handle_key_press(key, pressed);
        }
    };

    let keyboard_event = KEYBOARD_EVENTS.signal().clone();
    use_effect(move || {
        if let Some((key, pressed)) = keyboard_event.read().as_ref(){
            handle_key_press(key, *pressed);
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
                    game: active_game_signal(),
                    paused: paused_signal()
                }
            }
            div{
                class: "flex flex-row justify-center items-end gap-2 md:gap-4",
                select{
                    name: "game selection",
                    class: "select select-primary text-xl flex-6",
                    value: "{selected_game_signal().name}",
                    onchange: move |event| {
                        let value = event.value().clone();
                        if let Some(game) = Game::available_games().into_iter().find(|game| game.name == value){
                            selected_game_signal.set(game);
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
                    onclick: move |_| start_emu(&selected_game_signal()),
                    class: "btn btn-primary mt-5 text-xl font-thin flex-2",
                    "Start"
                }
                button{
                    onclick: move |_| pause_resume_emu(),
                    class: "btn btn-primary mt-5 text-xl flex-1",
                    if paused_signal(){
                        span{
                            class: "text-3xl",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                height: "24px",
                                view_box: "0 -960 960 960",
                                path {
                                    d: "M320-200v-560l440 280-440 280Z"
                                }
                            }
                        }
                    }else{
                        span{
                            class: "text-xl",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                height: "24px",
                                view_box: "0 -960 960 960",
                                path {
                                    d: "M560-200v-560h160v560H560Zm-320 0v-560h160v560H240Z"
                                }
                            }
                        }
                    }
                }
                button{
                    class: if !SHOW_KEYBOARD() {"lg:hidden"},
                    class: "btn btn-primary mt-5 text-xl flex-1",
                    class: if show_keyboard_signal(){ "btn-secondary" },
                    onclick: move |_| {
                        let current_value = *show_keyboard_signal.peek();
                        show_keyboard_signal.set(!current_value);
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        height: "24px",
                        view_box: "0 -960 960 960",
                        path {
                            d: "M160-200q-33 0-56.5-23.5T80-280v-400q0-33 23.5-56.5T160-760h640q33 0 56.5 23.5T880-680v400q0 33-23.5 56.5T800-200H160Zm0-80h640v-400H160v400Zm160-40h320v-80H320v80ZM200-440h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80ZM200-560h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80ZM160-280v-400 400Z"
                        }
                    }
                }
            }
            if let Some(game) = active_game_signal() {
                if !SHOW_KEYBOARD() {
                    Instructions{
                        game: game.clone()
                    }
                }
            }
            if SHOW_KEYBOARD(){
                MobileKeyboard {
                    game: active_game_signal()
                }
            }
            Footer{
                game: active_game_signal()
            }
        }
    }
}

