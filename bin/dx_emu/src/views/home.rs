use dioxus::prelude::*;
use crate::components::CanvasDisplay;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div{
            class: "w-2/5 mx-auto flex flex-col justify-center",
            CanvasDisplay{

            }
            p{
                "dsadasdasdasa"
            }
            button{
                class: "btn btn-primary",
                "dsa"
            }
        }
    }
}