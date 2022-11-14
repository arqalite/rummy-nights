use dioxus::prelude::*;
use crate::prelude::*;
use gloo_console::log;

pub fn screen(cx: Scope) -> Element {
    log!("Rendering settings menu.");
    
    cx.render(rsx!(
        top_bar()
    ))
}

fn top_bar(cx: Scope) -> Element {
    log!("Rendering top bar.");

    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        div {
            class: "h-16 grid grid-cols-3 z-10 mx-auto w-full sm:max-w-lg",
            button {
                class: "col-start-1 justify-self-start",
                onclick: |_| {
                    state.write().screen = Screen::Menu;
                },
                img {
                    class: "h-10 scale-x-[-1]",
                    src: "img/back.svg",
                }
            }
            // button {
            //     class: "col-start-3 justify-self-end",
            //     //onclick:
            //     img {
            //         class: "h-10",
            //         src: "img/save.svg",
            //     }
            // }
        }
    ))
}