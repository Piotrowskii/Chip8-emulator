use dioxus::prelude::*;
use chip8_lib::keypad::KeyPad;
use crate::helpers::chip8_wrapper::Chip8Web;
use crate::helpers::game::Game;
use crate::KEYBOARD_EVENTS;

#[component]
pub fn MobileKeyboard(game: Option<Game>) -> Element {
    let all_keys = KeyPad::all();
    let mut controls: Vec<(KeyPad, &'static str)> = vec![];
    if let Some(_game) = game{
        controls = _game.get_all_controls();
    }

    let mut keyboard_events = KEYBOARD_EVENTS.signal().clone();

    let is_key_active = |key: &KeyPad| -> bool {
        return controls.iter().map(|cont| cont.0).collect::<Vec<KeyPad>>().contains(key);
    };

    let get_key_text = |key: &KeyPad| -> Option<&'static str> {
        return controls.iter().find(|control| control.0 == *key).map(|control| control.1);
    };

    rsx!{
        div{
            class: "grid grid-cols-4 gap-2 fixed bottom-0 left-0 w-full mb-8 card bg-base-200 p-2 lg:hidden items-start ",
            for key in all_keys{
                div{
                    class: if is_key_active(&key) {""} else {""},
                    class: "flex flex-col text-center justify-center",
                    button{
                        class: "text-3xl select-none btn btn-primary w-full active:btn-secondary",
                        onpointerdown: move |event| {
                            event.prevent_default();
                            keyboard_events.set(Some((key.to_keyboard_str().to_string(), true)));
                        },
                        onpointerleave: move |event| {
                            event.prevent_default();
                            keyboard_events.set(Some((key.to_keyboard_str().to_string(), false)));
                        },
                        onpointerup: move |event| {
                            event.prevent_default();
                            keyboard_events.set(Some((key.to_keyboard_str().to_string(), false)));
                        },
                        onpointercancel: move |event| {
                            event.prevent_default();
                            keyboard_events.set(Some((key.to_keyboard_str().to_string(), false)));
                        },
                        {key.to_chip8_str()}
                    }
                    p{
                        class: "text-xs text-secondary",
                        {if let Some(text) = get_key_text(&key) {text} else {"⠀"}}
                    }
                }
            }
        }
    }

}
