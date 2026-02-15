use std::path::PathBuf;
use std::sync::{Arc, Mutex};
// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;
use chip8_lib::chip_8::{Chip8, Mode};
use views::{Home};
use crate::helpers::chip8_wrapper::Chip8Web;

/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;
mod helpers;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {}
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const T8NKS: &[u8] = include_bytes!("../assets/roms/t8nks.ch8");
const BREAKOUT: &[u8] = include_bytes!("../assets/roms/br8kout.ch8");
static CHIP8: GlobalSignal<Chip8Web> = Signal::global(|| Chip8Web::new(Mode::XoChip));
fn main() {
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        
        div{
            Router::<Route> {}
        }

    }
}
