use dioxus::prelude::*;
use crate::helpers::game::Game;

#[component]
pub fn Footer(game: Option<Game>) -> Element{
    rsx!{
        div{
            class: "fixed bottom-0 left-0 w-full flex flex-row items-center justify-center",
            div{
                class: "flex flex-wrap justify-center",
                div{
                    class: "whitespace-nowrap",
                    p{
                        class: "text-md md:text-xl",
                        "Site by ",
                        a{
                            href: "https://github.com/Piotrowskii",
                            class: "text-primary",
                            "Piotrowskii"
                        }
                    }
                }
                if let Some(game) = game{
                    div {
                        class: "whitespace-nowrap",
                        p {
                            class: "text-md md:text-xl",
                            ", {game.name} by "
                            if let Some(author) = game.author {
                                if let Some(url) = author.url {
                                    a {
                                        href: url,
                                        class: "text-primary",
                                        "{author.name}"
                                    }
                                } else {
                                    span {
                                        class: "text-primary",
                                        "{author.name}"
                                    }
                                }
                            } else {
                                span { "unknown" }
                            }
                        }
                    }
                }
            }
        }
    }
}