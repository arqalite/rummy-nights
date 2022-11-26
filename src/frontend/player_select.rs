use dioxus::events::FormEvent;
use dioxus::prelude::*;
use dioxus_web::use_eval;

use gloo_storage::{SessionStorage, Storage};

use crate::prelude::*;

pub fn Screen(cx: Scope) -> Element {
    log!("Rendering player select.");

    cx.render(rsx!(
        NavBar {},
        span {
            class: "font-semibold text-lg border-b-2 border-emerald-300 w-max mx-auto mb-8",
            "Add up to 4 players"
        },
        PlayerList {},
        StartGameButton {},
    ))
}

fn PlayerList(cx: Scope) -> Element {
    log!("test 3");
    let state = use_context::<Model>(&cx)?;
    log!("Rendering player list.");

    cx.render(rsx!(
        div {
            class: "flex flex-col gap-6",
            state.read().game.players.iter().map(|player| {
                log!("Got to state player iter.");
                let background_color = BG_COLORS[player.color_index];
                let id = player.id;

                let hide_color_bar = use_state(&cx, || true);
                let hidden = if **hide_color_bar {
                    "hidden"
                } else {
                    ""
                };
                let mut color_id = 0;

                log!("Rendering player.");
                rsx!(
                    div {
                        class: "flex justify-evenly h-16 rounded-full bg-slate-200",
                        button {
                            class: "flex justify-center h-8 w-3/5 self-center rounded-full {background_color}",
                            onclick: move |_| hide_color_bar.set(!hide_color_bar),
                            p {
                                class: "flex self-center text-white font-semibold",
                                "{player.name}"
                            }
                        }
                        button {
                            onclick: move |_| state.write().game.remove_player(id),
                            img {
                                class: "h-10",
                                src: "img/remove.svg",
                            }
                        }
                        div {
                            class: "flex flex-col justify-center self-center h-16 w-8",
                            button {
                                class: "place-self-center",
                                onclick: move |_| state.write().game.move_up(id),
                                img {
                                    class: "h-6",
                                    src: "img/up.svg"
                                },
                            }
                            button {
                                class: "place-self-center",
                                onclick: move |_| state.write().game.move_down(id),
                                img {
                                    class: "h-6	rotate-180",
                                    src: "img/up.svg"
                                },
                            }
                        }
                    },
                    div {
                        class: "{hidden} flex flex-row w-full justify-evenly h-10 mt-2 rounded-full bg-slate-200",
                        BG_COLORS.iter().map(|color| {
                            color_id += 1;
                            rsx!(
                                button {
                                    id: "{color_id}",
                                    class: "h-6 w-6 rounded-full {color} place-self-center",
                                    onclick: move |_| state.write().game.change_player_color(id, color_id),
                                }
                            )
                        })
                    }
                )
            }),
            PlayerInput {},
        }
    ))
}

fn PlayerInput(cx: Scope) -> Element {
    log!("test 2");

    let state = use_context::<Model>(&cx)?;
    let hide_color_bar = use_state(&cx, || true);
    let color_index = use_state(&cx, || 0);
    let selected_color = BG_COLORS[**color_index];
    let mut color_id = 0;

    if state.read().game.players.len() >= 4 {
        return None;
    }

    let hidden = if **hide_color_bar { "hidden" } else { "" };

    let onsubmit = move |evt: FormEvent| {
        let name = evt.values.get("player-name").unwrap().to_string();

        if !name.is_empty() {
            state.write().game.add_player(name, **color_index);

            //Execute some JS on the spot - weird ergonomics but it works
            use_eval(&cx)(String::from(
                "document.getElementById('name_input').reset();",
            ));
        };
    };

    log!("Rendering player input.");

    cx.render(rsx!(
    div {
        form {
            id: "name_input",
            class: "flex flex-row w-full justify-evenly h-16 rounded-full bg-slate-200",
            prevent_default: "onsubmit",
            onsubmit: onsubmit,
            input {
                name: "player-name",
                class: "rounded-full w-3/5 h-8 ring-1 ring-grey text-center self-center",
                placeholder: "Insert player name",
            }
            button {
                r#type: "submit",
                img {
                    class: "h-10",
                    src: "img/add.svg",
                }
            }
            button {
                class: "flex flex-col justify-center h-16 w-8",
                prevent_default: "onclick",
                onclick: move |_| hide_color_bar.set(!hide_color_bar),
                div {
                    class: "h-6 w-6 rounded-full {selected_color} place-self-center"
                }
            }
        }
        div {
            class: "{hidden} flex flex-row w-full justify-evenly h-10 mt-2 rounded-full bg-slate-200",
            BG_COLORS.iter().map(|color| {
                color_id += 1;
                rsx!(
                    button {
                        id: "{color_id}",
                        class: "h-6 w-6 rounded-full {color} place-self-center",
                        onclick: move |_| color_index.set(color_id-1),
                    }
                )
            })
        }
    }
    ))
}

fn StartGameButton(cx: Scope) -> Element {
    log!("test");
    let state = use_context::<Model>(&cx)?;

    if state.read().game.players.len() < 2 {
        return None;
    };

    log!("Rendering begin game button.");
    cx.render(rsx!(
        button {
            class: "z-10 flex absolute self-end w-max gap-2 border-b-[6px] border-emerald-300 right-8 bottom-[30vw]",
            onclick: move |_| state.write().start_game(),
            span {
                class: "flex self-center text-xl font-bold",
                "Start game"
            }
            img {
                class: "h-12 self-center",
                src: "img/arrow.svg"
            }
        }
    ))
}

fn NavBar(cx: Scope) -> Element {
    log!("Rendering top bar.");

    let state = use_context::<Model>(&cx)?;

    cx.render(rsx!(
        div {
            class: "h-16 grid grid-cols-3 z-10 mx-auto w-full sm:max-w-lg",
            button {
                class: "col-start-1 justify-self-start",
                onclick: move |_| {
                    state.write().screen = Screen::Menu;
                    state.write().checked_storage = false;
                    SessionStorage::delete("session");
                },
                img {
                    class: "h-10 scale-x-[-1]",
                    src: "img/back.svg",
                }
            }
            button {
                class: "col-start-3 justify-self-end",
                onclick: move |_| state.write().screen = Screen::Templates,
                img {
                    class: "h-10",
                    src: "img/save.svg",
                }
            }
        }
    ))
}
