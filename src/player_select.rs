//! The player selection screen.

use dioxus::core::UiEvent;
use dioxus::events::FormData;
use dioxus::fermi::*;
use dioxus::prelude::*;
use std::collections::BTreeMap;

use crate::css;
use crate::GameStatus;
use crate::Player;
use crate::STATE;

pub fn player_select(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);
    let players = &state.players;

    let onclick = |_| {
        if players.len() >= 2 {
            state.with_mut(|state| 
                state.game_status = GameStatus::Ongoing
            )
        };
    };

    cx.render(rsx!(
        div {
            class: "mx-auto px-4 max-w-md",
            //Navbar
            div {
                class: "h-16",
            }
            //Player select
            div {
                class: "",

                //Player list
                players.iter().map(|player| {
                    let background = css::TITLE_COLORS[player.id-1];

                    rsx!(
                        div {
                            class: "grid grid-cols-3 gap-4 h-16 rounded-full bg-slate-200 my-2",
                            div {
                                class: "ml-4 my-auto h-8 col-span-2 rounded-full {background}",
                                p {
                                    class: "text-center mx-auto pt-0.5 text-white font-semibold",
                                    "{player.name}"
                                }
                            }
                        }
                    )
                })

                //Name input
                player_input(),
            }
            //Start button
            div {
                button {
                    class: "w-full text-center my-2",
                    onclick: onclick,
                    "Start"
                }        
            }
        }
    ))
}

fn player_input(cx: Scope) -> Element {
    let buffer = use_state(&cx, String::new);
    let state = use_atom_state(&cx, STATE);

    let onsubmit = move |_| {
        state.with_mut(|state| {
            if state.players.len() < 4 {
                state.players.push(Player {
                    id: state.players.len() + 1,
                    name: buffer.to_string(),
                    score: BTreeMap::new(),
                });
            };
            buffer.set(String::new());
        });
    };

    let oninput = move |evt: UiEvent<FormData>| {
        buffer.set(evt.value.clone());
    };

    if state.players.len() <= 3 {
        cx.render(
            rsx!(
                form {
                    onsubmit: onsubmit,
                    prevent_default: "onsubmit",
                    input {
                        class: "rounded-full mx-auto h-8 w-full shadow ring-1 ring-grey text-center",
                        placeholder: "Insert player name",
                        value: "{buffer}",
                        oninput: oninput,
                    }
                }
            )
        )
    } else {
        None
    }
}
