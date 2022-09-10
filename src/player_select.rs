use dioxus::prelude::*;
use dioxus::fermi::*;
use dioxus::events::FormData;
use dioxus::core::UiEvent;
use std::collections::BTreeMap;

use crate::css;
use crate::STATE;
use crate::Player;

pub fn input_screen(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);

    cx.render(rsx! (
            p {
                class: "text-center",
                "Add players:"
            }
            div {
                class: "mx-auto px-5 max-w-md mt-4 gap-x-5",
                state.players.iter().map(|player| {
                    let background = css::TITLE_COLORS[player.id-1];
                    rsx!(
                        div {
                            // Name - first cell
                            class: "rounded-full h-8 {background} py-1 mb-2 shadow",
                            p {
                                class: "text-center my-auto text-white font-semibold",
                                "{player.name}"
                            }
                        }
                    ) 
                }),
                player_input(),
            }
            
    ))
}

fn player_input(cx: Scope) -> Element {
    let buffer = use_state(&cx, String::new);
    let state = use_atom_state(&cx, STATE);

    let onsubmit = move |_| {
        state.with_mut(|state| {
            if state.players.len() < 4 {
                state.players.push(
                    Player {
                        id: state.players.len() + 1,
                        name: buffer.to_string(),
                        score: BTreeMap::new(),
                    }
                );
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
                        class: "rounded-full mx-auto h-8 py-1 mb-2 w-full shadow ring-1 ring-grey text-center my-auto",
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