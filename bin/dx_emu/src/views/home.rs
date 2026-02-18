use dioxus::prelude::*;
use crate::components::{Emulator, MobileKeyboard};
use crate::SHOW_KEYBOARD;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx!{
        div{
            class: "lg:w-3/5 2xl:w-2/5 p-2 lg:p-5 mx-auto flex flex-col justify-center lg:bg-base-200 lg:card lg:shadow-md",
            p{
                class: "text-5xl md:text-8xl text-center my-4 underline",
                "XO-Chip Emulator"
            }
            Emulator{}
        }
    }
}