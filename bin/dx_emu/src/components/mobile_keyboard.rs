use dioxus::prelude::*;
use chip8_lib::keypad::KeyPad;
use crate::helpers::chip8_wrapper::Chip8Web;
use crate::helpers::game::Game;
use crate::KEYBOARD_EVENTS;

#[component]
pub fn MobileKeyboard(game: Game) -> Element {
    let all_keys = KeyPad::all();
    let mut keyboard_events = KEYBOARD_EVENTS.signal().clone();


    //TODO: Wrap Instructions into KeyGROUP
    rsx!{
        div{
            class: "grid grid-cols-4 gap-4 fixed bottom-0 left-0 w-full mb-8 card bg-base-200 p-2 lg:hidden",
            for key in all_keys{
                div{
                    class: "flex flex-row justify-center",
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
                }
            }
        }
    }
}
