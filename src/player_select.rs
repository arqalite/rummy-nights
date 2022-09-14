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
use crate::Screen;

pub fn player_select(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);
    let players = &state.players;

    let onclick = |_| {
        if players.len() >= 2 {
            state.with_mut(|state| {
                state.game_status = GameStatus::Ongoing;
                state.screen = Screen::Game;
            })
        };
    };

    cx.render(rsx!(
        //Navbar
        div {
            class: "h-16 grid grid-cols-3",
            button {
                class: "mx-auto h-16 relative left-[-30%]",
                //onclick:
                img {
                    class: "h-8 w-8",
                    src: "img/back.svg",
                }
            }
            button {
                class: "mx-auto h-16 relative justify-self-center",
                //onclick:
                img {
                    class: "h-8 w-8",
                    src: "img/home.svg",
                }
            }
            button {
                class: "mx-auto h-16 relative right-[-30%]",
                //onclick:
                img {
                    class: "h-8 w-8",
                    src: "img/save.svg",
                }
            }
        },
            //Player select
            div {
                class: "pt-2 px-2",
                //Player list
                players.iter().map(|player| {
                    let background = css::TITLE_COLORS[player.id-1];
                    let delete_button = |_| {
                        state.with_mut(|mut_state| {
                            mut_state.players.retain(|item|{
                                item.id != player.id
                            });
                        })
                    };

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
                            div {
                                class: "col-span-1 my-auto",
                                button {
                                    class: "mx-auto h-16 px-2",
                                    onclick: delete_button,
                                    img {
                                        class: "h-8 w-8",
                                        src: "img/remove.svg",
                                    }
                                }
                                button {
                                    class: "mx-auto h-16 px-2",
                                    //onclick:
                                    img {
                                        class: "h-8 w-8",
                                        src: "img/menu.svg",
                                    }
                                }
                            }
                        }
                    )
                }),
                //Name input
                player_input(),
            }
            //Start button
            div {
                class: "w-32 h-12 mt-16 border-b-4 border-emerald-300 relative left-2/3",
                button {
                    class: "w-full text-center my-2",
                    onclick: onclick,
                    p {
                        class: "inline-block pr-2 font-bold",
                        "Start game"
                    }
                    img {
                        class: "h-8 w-8 inline-block",
                        src: "img/arrow.svg"
                    }
                }
            }
    ))
}

fn player_input(cx: Scope) -> Element {
    let buffer = use_state(&cx, String::new);
    let state = use_atom_state(&cx, STATE);

    let onsubmit = move |_| {
        if buffer.len() > 0 {
            add_player(cx, buffer.to_string());
            buffer.set(String::new());
        }
    };
    let onclick = move |_| {
        if buffer.len() > 0 {
            add_player(cx, buffer.to_string());
            buffer.set(String::new());
        }
    };

    let oninput = move |evt: UiEvent<FormData>| {
        buffer.set(evt.value.clone());
    };

    if state.players.len() <= 3 {
        cx.render(
            rsx!(
                div {
                    class: "grid grid-cols-3 h-16 rounded-full bg-slate-200 my-2",
                    div {
                        class: "ml-4 my-auto h-8 col-span-2 rounded-full",
                        form {
                            onsubmit: onsubmit,
                            prevent_default: "onsubmit",
                            input {
                                class: "rounded-full mx-auto h-8 w-full shadow ring-1 ring-grey text-center",
                                placeholder: "Insert player name",
                                value: "{buffer}",
                                oninput: oninput,
                                onsubmit: onsubmit,
                                prevent_default: "onsubmit",
                            }
                        }
                    }
                    button {
                        class: "col-span-1 mx-auto text-center h-16",
                        onclick: onclick,
                        img {
                            class: "h-8 w-8",
                            src: "img/add-player.svg",
                        },
                    }
                }
            )
        )
    } else {
        None
    }
}

fn add_player(cx: Scope, name: String) {
    let state = use_atom_state(&cx, STATE);

    let mut lowest_available_id = 0;

    for i in 1..5 {
        let slot = state.players.iter().find(|item| item.id == i);

        if slot == None {
            lowest_available_id = i;
            break;
        };
    }

    state.with_mut(|state| {
        if state.players.len() < 4 && lowest_available_id != 0 {
            state.players.push(Player {
                id: lowest_available_id,
                name: name,
                score: BTreeMap::new(),
            });
        };
    });
}
