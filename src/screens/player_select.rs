//! The player selection screen.

use dioxus::core::UiEvent;
use dioxus::events::FormData;
use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;

use crate::data::tailwind_classes;
use crate::prelude::*;

pub fn screen(cx: Scope) -> Element {
    cx.render(rsx!(
        div { //Screen container
            class: "flex flex-col grow h-screen w-screen relative overflow-hidden px-[5%]",
            decorative_spheres()
            top_bar()

            div {
                class: "z-10 flex flex-col grow relative mx-auto w-full sm:max-w-lg",
                div {
                    class: "mb-6 w-max mx-auto",
                    span {
                        class: "font-semibold text-lg border-b-2 border-emerald-300",
                        "Add up to 4 players"
                    }
                }
                player_list()
                start_game_button()
            }
        }
    ))
}

fn start_game_button(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        button {
            class: "z-10 flex absolute self-end w-max gap-2 border-b-[6px] border-emerald-300 right-0 bottom-32",
            onclick: |_| {
                if state.read().players.len() >= 2 {
                    state.write().game_status = GameStatus::Ongoing;
                    state.write().screen = Screen::Game;
                };
            },
            span {
                class: "flex self-center text-xl font-bold w-max",
                "Start game"
            }
            img {
                class: "h-12 w-12",
                src: "img/arrow.svg"
            }
        }
    ))
}

fn player_list(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        div {
            class: "flex flex-col gap-6",
            state.read().players.iter().map(|player| {
                let background = tailwind_classes::TITLE_COLORS[player.id-1];
                let player_id = player.id;


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
                            onclick: move |_| state.write().remove_player(player_id),
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
    ))
}

fn player_input(cx: Scope) -> Element {
    let buffer = use_state(&cx, String::new);
    let state = use_atom_ref(&cx, STATE);

    let onsubmit = move |_| {
        if buffer.len() > 0 {
            state.write().add_player(buffer.to_string());
            buffer.set(String::new());
        }
    };

    let oninput = |evt: UiEvent<FormData>| {
        buffer.set(evt.value.clone());
    };

    let onclick = move |_| {
        if buffer.len() > 0 {
            state.write().add_player(buffer.to_string());
            buffer.set(String::new());
        }
    };

    if state.read().players.len() <= 3 {
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
        ))
    } else {
        None
    }
}

fn top_bar(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        div {
            class: "h-16 grid grid-cols-3 z-10 mx-auto w-full sm:max-w-lg",
            button {
                class: "col-start-1 justify-self-start",
                onclick: |_| {state.write().screen = Screen::Menu}, //return to main menu
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
        }
    ))
}

fn decorative_spheres(cx: Scope) -> Element {
    cx.render(rsx!(
        div { //Decorative circles
            class: "z-0 absolute h-screen w-screen",
            div {
                class: "w-[300px] h-[300px] bottom-[-150px] left-[-150px] absolute rounded-full z-0",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
            }
        }
    ))
}
