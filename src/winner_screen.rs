
use dioxus::prelude::*;
use dioxus::fermi::use_atom_state;

use crate::STATE;
use crate::Screen;
use crate::css;

pub fn winner_screen(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);

    cx.render(rsx!(
        div {
            class: "mx-auto",
            img {
                src: "img/trophy.svg",
                class: "h-12 w-12 mx-auto"
            }
            p {
                class: "text-center font-bold text-2xl",
                "THE WINNER IS"
            }
        },
        state.players.iter().map(|player| {
            let background = css::TITLE_COLORS[player.id-1];
            let score = player.score.values().sum::<i32>();

            rsx! (
            div {
                class: "flex flex-row justify-evenly py-4",
                div {
                    class: "",
                    p {
                        class: "",
                        "{score}"
                    }
                }
                div {
                    class: "h-8 basis-1/4 {background} rounded-full",
                    p {
                        class: "text-center",
                        "{player.name}"
                    }
                }
            }
            )
        })
    ))
}