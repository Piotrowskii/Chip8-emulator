use std::path::PathBuf;
use std::string::ToString;
use std::sync::{Arc, Mutex};
// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;
use web_sys::EventListener;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsCast;
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
const GLOBAL_CSS: Asset = asset!("/assets/global.css");
const JERSEY10_FONT: Asset = asset!("/assets/fonts/Jersey10-Regular.ttf");
static KEYBOARD_EVENTS: GlobalSignal<Option<(String, bool)>> = GlobalSignal::new(|| None);
static SHOW_KEYBOARD: GlobalSignal<bool> = GlobalSignal::new(|| false);

fn main() {
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    let mut keyboard_events = KEYBOARD_EVENTS.signal().clone();

    use_effect(move || {
        let window = web_sys::window().unwrap();

        let closure_keydown = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            keyboard_events.set(Some((event.key(), true)));
        }) as Box<dyn FnMut(_)>);

        let closure_keyup = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            keyboard_events.set(Some((event.key(), false)));
        }) as Box<dyn FnMut(_)>);

        window.add_event_listener_with_callback("keydown", closure_keydown.as_ref().unchecked_ref()).unwrap();
        window.add_event_listener_with_callback("keyup", closure_keyup.as_ref().unchecked_ref()).unwrap();

        closure_keydown.forget();
        closure_keyup.forget();
    });


    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: GLOBAL_CSS }
        document::Link { rel: "preload", href: JERSEY10_FONT, as: "font" , type: "font/ttf", crossorigin: "anonymous"}
        document::Style {
            r#"
            @font-face {{
                font-family: 'Jersey 10';
                src: url("{JERSEY10_FONT}") format('truetype');
                font-weight: normal;
                font-style: normal;
            }}
            "#
        }
        div{
            id: "test",
            class: "mt-5 select-none md:select-auto",
            Router::<Route> {}
        }
    }
}

