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
            class: "flex flex-col relative mx-auto h-screen w-screen overflow-hidden px-8",
            div {
                class: "z-0 absolute h-screen w-screen",
                div {
                    class: "w-[300px] h-[300px] bottom-[-150px] left-[-150px] absolute rounded-full z-0",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                }
            }    
            div {
                class: "h-16 grid grid-cols-3 z-10",
                button {
                    class: "mx-auto h-16 relative left-[-50%]",
                    onclick: return_to_menu,
                    img {
                        class: "h-8 w-8",
                        src: "img/back.svg",
                    }
                }
                button {
                    class: "mx-auto h-16 relative col-start-3 right-[-50%]",
                    //onclick:
                    img {
                        class: "h-8 w-8",
                        src: "img/save.svg",
                    }
                }
            },
            //Player select
            div {
                class: "z-10",
                div {
                    class: "w-full rounded-full flex self-center mx-auto mb-8",
                    //background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    p {
                        class: "mx-auto self-center font-semibold text-lg text-black border-b-2 border-emerald-300",
                        "Add up to 4 players"
                    }
                },
                //Player list
                state.players.iter().map(|player| {
                    let background = TITLE_COLORS[player.id-1];
                    let delete_button = move |_| {
                        remove_player(cx, player.id)
                    };

                    rsx!(
                        div {
                            class: "grid grid-cols-3 gap-6 items-center h-20 rounded-full bg-slate-200 mb-6 space-x-2",
                            div {
                                class: "flex justify-center items-center ml-4 w-full h-12 col-span-2 rounded-full {background}",
                                p {
                                    class: "text-white font-semibold",
                                    "{player.name}"
                                }
                            }
                            div {
                                class: "col-span-1 my-auto space-x-4",
                                button {
                                    class: "mx-auto h-16",
                                    onclick: delete_button,
                                    img {
                                        class: "h-8 w-8",
                                        src: "img/remove.svg",
                                    }
                                }
                                button {
                                    class: "mx-auto h-16",
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
            },
            //Start button
            div {
                class: "z-10 w-48 h-18 mt-16 border-b-[6px] border-emerald-300 ml-auto mr-8",
                button {
                    class: "w-full text-center my-2",
                    onclick: onclick,
                    p {
                        class: "inline-block pr-2 text-xl font-bold align-middle",
                        "Start game"
                    }
                    img {
                        class: "h-12 w-12 inline-block align-middle",
                        src: "img/arrow.svg"
                    }
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

    let oninput = |evt: UiEvent<FormData>| {
        buffer.set(evt.value.clone());
    };

    if state.players.len() <= 3 {
        cx.render(rsx!(
            div {
                class: "grid grid-cols-3 items-center h-20 rounded-full bg-slate-200",
                form {
                    class: "col-span-2 w-full",
                    onsubmit: onsubmit,
                    prevent_default: "onsubmit",
                    input {
                        class: "ml-4 rounded-full w-full mx-auto h-12 ring-1 ring-grey text-center",
                        placeholder: "Insert player name",
                        value: "{buffer}",
                        oninput: oninput,
                        onsubmit: onsubmit,
                        prevent_default: "onsubmit",
                    }
                },
                button {
                    class: "col-span-1 mx-auto text-center",
                    onclick: onclick,
                    img {
                        class: "h-8 w-8",
                        src: "img/add-player.svg",
                    },
                }
            }
        ))
    } else {
        None
    }
}


