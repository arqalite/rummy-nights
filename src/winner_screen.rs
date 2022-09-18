
use dioxus::prelude::*;
use dioxus::fermi::use_atom_state;

use crate::STATE;
use crate::Screen;
use crate::css;

pub fn winner_screen(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);

    cx.render(rsx!(
        state.players.iter().map(|player| {
            let background = css::TITLE_COLORS[player.id-1];

            rsx! (
            div {
                class: "grid grid-cols-2",
                div {
                    class: "ml-4 my-auto h-8 col-span-1 rounded-full {background}",
                    p {
                        class: "text-center mx-auto pt-0.5 text-white font-semibold",
                        "{player.name}"
                    }
                }
                div {

                }
            }
            )
        })
    ))
}