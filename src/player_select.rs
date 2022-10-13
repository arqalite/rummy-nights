//! The player selection screen.

use dioxus::core::UiEvent;
use dioxus::events::FormData;
use dioxus::fermi::use_atom_state;
use dioxus::prelude::*;

use crate::data::{GameStatus, Screen, TITLE_COLORS, add_player, remove_player};
use crate::STATE;

pub fn player_select(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);

    let onclick = |_| {
        if state.players.len() >= 2 {
            state.with_mut(|state| {
                state.game_status = GameStatus::Ongoing;
                state.screen = Screen::Game;
            });
        };
    };

    let return_to_menu = |_| {
        state.with_mut(|state| {
            state.screen = Screen::Menu;
        });
    };

    cx.render(rsx!(
        div {
            class: "flex flex-col grow h-screen w-screen relative overflow-hidden px-[5%]",
            div {
                class: "z-0 absolute h-screen w-screen",
                div {
                    class: "w-[300px] h-[300px] bottom-[-150px] left-[-150px] absolute rounded-full z-0",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                }
            }    
            div {
                class: "h-16 grid grid-cols-3 z-10 mx-auto w-full sm:max-w-lg",
                button {
                    class: "col-start-1 justify-self-start",
                    onclick: return_to_menu,
                    img {
                        class: "h-8 w-8",
                        src: "img/back.svg",
                    }
                }
                button {
                    class: "col-start-3 justify-self-end",
                    //onclick:
                    img {
                        class: "h-8 w-8",
                        src: "img/save.svg",
                    }
                }
            },
            //Player select
            div {
                class: "z-10 flex flex-col grow relative mx-auto w-full sm:max-w-lg",
                div {
                    class: "mb-6 w-max mx-auto",
                    span {
                        class: "font-semibold text-lg border-b-2 border-emerald-300",
                        "Add up to 4 players"
                    }
                }
                //Player list
                div {
                    class: "flex flex-col gap-6",
                    state.players.iter().map(|player| {
                        let background = TITLE_COLORS[player.id-1];
                        let delete_button = move |_| {
                            remove_player(cx, player.id)
                        };

                        rsx!(
                            div {
                                class: "flex justify-evenly h-16 rounded-full bg-slate-200 pr-2",
                                div {
                                    class: "flex justify-center	content-center h-8 w-3/5 self-center rounded-full {background}",
                                    p {
                                        class: "flex self-center text-white font-semibold",
                                        "{player.name}"
                                    }
                                }
                                button {
                                    onclick: delete_button,
                                    img {
                                        class: "h-7",
                                        src: "img/remove.svg",
                                    }
                                }
                                button {
                                    //onclick:
                                    img {
                                        class: "h-7",
                                        src: "img/menu.svg",
                                    }
                                }
                            }
                        )
                    }),

                    player_input(),
                }
                //Start button
                button {
                    class: "z-10 flex absolute self-end w-max gap-2 border-b-[6px] border-emerald-300 right-0 bottom-32",
                    onclick: onclick,
                    span {
                        class: "flex self-center text-xl font-bold w-max",
                        "Start game"
                    }
                    img {
                        class: "h-12 w-12",
                        src: "img/arrow.svg"
                    }
                }
            },
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

    let oninput = |evt: UiEvent<FormData>| {
        buffer.set(evt.value.clone());
    };

    if state.players.len() <= 3 {
        cx.render(rsx!(
            div {
                class: "flex flex-row justify-evenly h-16 rounded-full bg-slate-200 pr-2",
                form {
                    class: "w-3/5 self-center",
                    onsubmit: onsubmit,
                    prevent_default: "onsubmit",
                    input {
                        class: "rounded-full w-full h-8 ring-1 ring-grey text-center",
                        placeholder: "Insert player name",
                        value: "{buffer}",
                        oninput: oninput,
                        onsubmit: onsubmit,
                        prevent_default: "onsubmit",
                    }
                },
                button {
                    onclick: onclick,
                    img {
                        class: "h-7",
                        src: "img/add-player.svg",
                    },
                }
                button {
                    //onclick:
                    img {
                        class: "h-7",
                        src: "img/menu.svg",
                    }
                }
            }
        ))
    } else {
        None
    }
}


