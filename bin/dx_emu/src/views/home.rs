use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use chip8_lib::chip_8::KeyPad;
use chip8_lib::display::Display;
use crate::components::{EmuDisplay, Emulator};
use crate::helpers::chip8_wrapper::Chip8Web;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx!{
        div{
            class: "lg:w-3/5 2xl:w-2/5 p-5 mx-auto flex flex-col justify-center bg-base-200 card shadow-md",
            Emulator{}
        }
    }
}