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
            class: "h-36 grid grid-cols-3 overflow-hidden",
            div {
                class: "w-60 h-60 relative top-[-100px] left-[-100px] rounded-full",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
            }
            p {
                class: "-rotate-45 text-white text-2xl font-semibold w-8 absolute top-10 left-12",
                "Rummy Nights"
            }
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
                class: "h-96 w-96 mx-auto",
                src: "img/fly-away.svg",
            }
        }
        button {
            class: "w-full h-32 mx-auto pt-8",
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