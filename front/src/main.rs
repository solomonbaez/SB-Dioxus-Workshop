#![allow(non_snake_case)]
use dioxus::prelude::*;

mod components;
mod models;
use components::{FilmModal, Footer, Header};

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "relative z-0 bg-gray-500 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header {}
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
                FilmModal {
                    on_create_or_update: move |_| {},
                    on_cancel: move |_| {},
                }
            }
            Footer {}
        }
    })
}
