#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)] // Happy Clippy, happy life
#![allow(clippy::used_underscore_binding)] // Clippy doesn't understand Dioxus's Props macro tho

use dioxus::prelude::*;
use std::collections::BTreeMap;
use dioxus::core::UiEvent;
use dioxus::events::{FormData};
use dioxus::fermi::use_atom_state;

// mod game_menu;
mod score_table;
mod css;

static PLAYERS: Atom<Vec<Player>> = |_| Vec::new();

#[derive(PartialEq, Clone)]
struct Player {
    id: usize,
    name: String,
    score: BTreeMap<usize, i32>,
}

fn app(cx: Scope) -> Element {
    let has_game_started = use_state(&cx, || false);
    let state = use_atom_state(&cx, PLAYERS);
    
    let onclick = |_| {
        if state.len() >= 2 {
            has_game_started.set(true);
        };
    };

    if **has_game_started {
        cx.render(rsx! (
            score_table::score_table(),
        ))
    } else {
        cx.render(rsx!(
            div {
                class: "",
                input_screen()
                button {
                    class: "mx-auto w-full text-center",
                    onclick: onclick,
                    "Start"
                }
            }
        ))
    }
    
}

fn input_screen(cx: Scope) -> Element {
    let state = use_atom_state(&cx, PLAYERS);

    cx.render(rsx! (
            p {
                class: "text-center",
                "Add players:"
            }
            div {
                class: "mx-auto px-5 max-w-md mt-4 gap-x-5",
                state.iter().map(|player| {
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
    let state = use_atom_state(&cx, PLAYERS);

    let onsubmit = move |_| {
        state.with_mut(|players| {
            if players.len() < 4 {
                players.push(
                    Player {
                        id: players.len() + 1,
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
    
    if state.len() <= 3 {
        cx.render(
            rsx!(
                form {
                    onsubmit: onsubmit,
                    prevent_default: "onsubmit",
                    input {
                        class: "rounded-full mx-auto h-8 py-1 mb-2 w-full shadow ring-1 ring-black text-center my-auto",
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

fn main() {
    dioxus::web::launch(app);
}
