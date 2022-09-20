//! The intro screen.
//! It doesn't do much besides showing a Start button and looking nice, 
//! so the users don't get thrown directly into player select.

use dioxus::prelude::*;
use dioxus::fermi::use_atom_state;

use crate::STATE;
use crate::data::Screen;

pub fn intro(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);

    let go_to_player_select = |_| {
        state.with_mut(|mut_state| {
            mut_state.screen = Screen::PlayerSelect;
        });
    };

    cx.render(rsx!(
        div {
            class: "grid grid-cols-6",
            div {
                class: "w-[200px] h-[200px] relative top-[-75px] left-[-75px] rounded-full",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
            },
            p {
                class: "-rotate-45 text-white text-2xl text-justify font-semibold relative w-6 top-6",
                "Rummy Nights"
            }
            button {
                class: "mx-auto h-16 col-start-6",
                //onclick:
                img {
                    class: "",
                    src: "img/user.svg",
                }
            }
        }
        div {
            img {
                class: "mx-auto my-16",
                src: "img/intro.gif",
            }
            // p {
            //     class: "mx-auto font-semibold text-2xl text-center pt-8",
            //     "Welcome to Rummy Nights!"
            // }
        }
        button {
            class: "w-full mx-auto mt-32",
            onclick: go_to_player_select,
            p {
                class: "font-bold text-center mr-8 text-lg inline",
                "Start Game"
            }
            img {
                class: "h-20 w-20 inline align-middle",
                src: "img/new.svg", 
            }
        }
    ))
}