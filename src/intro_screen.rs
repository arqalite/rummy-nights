//! The intro screen.
//! It doesn't do much besides showing a Start button and looking nice, 
//! so the users don't get thrown directly into player select.

use dioxus::prelude::*;
use dioxus::fermi::use_atom_state;

use crate::STATE;
use crate::Screen;

pub fn intro(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);

    let go_to_player_select = |_| {
        state.with_mut(|mut_state| {
            mut_state.screen = Screen::PlayerSelect;
        })
    };

    cx.render(rsx!(
        div {
            class: "h-16 grid grid-cols-3",
            button {
                class: "mx-auto h-16 col-start-3 relative right-[-30%]",
                //onclick:
                img {
                    class: "h-8 w-8",
                    src: "img/user.svg",
                }
            }
        }
        div {
            //As the name and image suggests, this will fly away from the app soon.
            img {
                class: "",
                src: "img/fly-away.svg",
            }    
            p {
                class: "font-semibold text-center",
                "Welcome to Rummy Nights!"
            }
        }
        button {
            class: "w-full h-32 mx-auto pt-8",
            onclick: go_to_player_select,
            img {
                class: "h-24 w-24 inline align-middle",
                src: "img/new.svg", 
            }
            p {
                class: "font-bold text-center ml-4 text-lg inline drop-shadow-2xl",
                "New game"
            }
        }
    ))
}