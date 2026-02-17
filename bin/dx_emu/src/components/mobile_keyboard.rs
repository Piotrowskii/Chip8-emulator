use dioxus::prelude::*;
use chip8_lib::keypad::KeyPad;
use crate::KEYBOARD_EVENTS;

#[component]
pub fn MobileKeyboard() -> Element {
    let all_keys = get_all_keys();
    let mut keyboard_events = KEYBOARD_EVENTS.signal().clone();

    rsx!{
        div{
            class: "grid grid-cols-4 gap-4 fixed bottom-0 left-0 w-full mb-8 card bg-base-200 p-2 lg:hidden",
            for (display,value) in all_keys{
                div{
                    class: "flex flex-row justify-center",
                    button{
                        class: "text-3xl select-none btn btn-primary w-full active:btn-secondary",
                        onpointerdown: move |event| {
                            event.prevent_default();
                            keyboard_events.set(Some((value.to_string(), true)));
                        },
                        onpointerleave: move |event| {
                            event.prevent_default();
                            keyboard_events.set(Some((value.to_string(), false)));
                        },
                        onpointerup: move |event| {
                            event.prevent_default();
                            keyboard_events.set(Some((value.to_string(), false)));
                        },
                        onpointercancel: move |event| {
                            event.prevent_default();
                            keyboard_events.set(Some((value.to_string(), false)));
                        },
                        "{display}"
                    }
                }
            }
        }
    }
}

pub fn get_all_keys() -> [(&'static str, &'static str); 16]{
    [
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("C", "4"),

        ("4", "q"),
        ("5", "w"),
        ("6", "e"),
        ("D", "r"),

        ("7", "a"),
        ("8", "s"),
        ("9", "d"),
        ("E", "f"),

        ("A", "z"),
        ("0", "x"),
        ("B", "c"),
        ("F", "v"),
    ]
}