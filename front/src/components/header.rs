// prelude includes all macros
use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx!(
        header {
            class: "sticky top-0 z-10 text-gray-400 bg-gray-800 body-font shadow-md",
            div { class: "container mx-auto flex flex-wrap p-0 flex-col md:flex-row justify-between items-center",
                a {
                    class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
                    img {
                        class: "bg-transparent p-2 animate-jump",
                        alt: "ferris",
                        src: "ferris.png",
                        "loading": "lazy"
                    }
                    span { class: "ml-3 text-2x1", "Rusty films" }
                }
            }
        }
    ))
}
