//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod emu_display;
mod emulator;
mod mobile_keyboard;
mod instructions;
mod footer;

pub use emu_display::EmuDisplay;
pub use emulator::Emulator;
pub use mobile_keyboard::MobileKeyboard;
pub use instructions::Instructions;
pub use footer::Footer;