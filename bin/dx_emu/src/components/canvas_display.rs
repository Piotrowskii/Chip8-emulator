use dioxus::prelude::*;

#[component]
pub fn CanvasDisplay() -> Element{
    rsx! {
        canvas {
            width: "128",
            height: "64",
            style: "border: 1px solid black; image-rendering: pixelated;",


        }
    }
}