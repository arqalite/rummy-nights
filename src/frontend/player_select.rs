use dioxus::events::FormEvent;
use dioxus::prelude::*;
use dioxus_web::use_eval;
use gloo_storage::{SessionStorage, Storage};

use crate::prelude::*;

pub fn screen(cx: Scope) -> Element {
    log!("Rendering player select.");

    let state = fermi::use_atom_ref(cx, STATE);
    let add_players = get_text(state.read().settings.language, "add_players").unwrap();

    cx.render(rsx!(
        TopBar {}
        div {
            class: "flex flex-col grow pb-8",
            div {
                class: "flex flex-col grow",
                span {
                    class: "font-semibold text-lg border-b-2 border-emerald-300 w-max mx-auto mb-4",
                    "{add_players}"
                }
                PlayerList {}
            }
            BeginGameButton {}
        }
    ))
}

fn PlayerList(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    let insert_player = get_text(state.read().settings.language, "insert_player").unwrap();

    log!("Rendering player list.");

    cx.render(rsx!(
        div {
            class: "flex flex-col px-8 grow gap-2",
            state.read().game.players.iter().map(|player| {
                let background_color = BG_COLORS[player.color_index];
                let id = player.id;

                let show_player_edit = use_state(&cx, || false);
                let hide_color_bar = use_state(&cx, || true);

                let hidden = if **hide_color_bar {
                    "hidden"
                } else {
                    ""
                };

                let mut color_id = 0;

                let buffer = use_state(&cx, || player.name.clone());

                let onsubmit = move |evt: FormEvent| {
                    let name = evt.values.get("player-name").unwrap().to_string();
                    if !name.is_empty() {
                        state.write().game.edit_player_name(id - 1, name);
                        show_player_edit.set(!show_player_edit);
                    };
                    hide_color_bar.set(true);
                };

                let oninput = move |evt: FormEvent| {
                    buffer.set(evt.value.clone())
                };

                log!("Rendering player.");
                rsx!(
                    (!show_player_edit).then(|| rsx!(
                        div {
                            class: "flex justify-evenly h-14 rounded-full bg-slate-200",
                            button {
                                class: "flex justify-center h-8 w-3/5 self-center rounded-full {background_color}",
                                onclick: move |_| {
                                    show_player_edit.set(!show_player_edit)
                                },
                                p {
                                    class: "flex self-center text-white font-semibold",
                                    "{player.name}"
                                }
                            }
                            button {
                                onclick: move |_| state.write().game.remove_player(id),
                                div {
                                    class: "h-10",
                                    assets::RemoveIcon {}
                                }
                            }
                            div {
                                class: "flex flex-col justify-center self-center h-12 w-8",
                                button {
                                    class: "place-self-center",
                                    onclick: move |_| state.write().game.move_up(id),
                                    div {
                                        class: "h-8",
                                        assets::up_icon {}
                                    },
                                }
                                button {
                                    class: "place-self-center",
                                    onclick: move |_| state.write().game.move_down(id),
                                    div {
                                        class: "h-8 rotate-180",
                                        assets::up_icon {}
                                    },
                                }
                            }
                        }
                    )),
                    show_player_edit.then(|| rsx!(
                        form {
                            id: "player_name_input",
                            class: "flex flex-row w-full justify-evenly items-center h-14 rounded-full bg-slate-200",
                            prevent_default: "onsubmit",
                            onsubmit: onsubmit,
                            input {
                                name: "player-name",
                                class: "rounded-full w-3/5 h-8 ring-1 ring-grey text-center self-center",
                                placeholder: "{insert_player}",
                                oninput: oninput,
                                value: "{buffer}"
                            }
                            input {
                                name: "template_id",
                                r#type: "hidden",
                                value: "{id}",
                            }
                            button {
                                r#type: "submit",
                                class: "h-10",
                                assets::okay_button {},
                            }
                            button {
                                class: "flex flex-col justify-center h-16 w-8",
                                prevent_default: "onclick",
                                onclick: move |_| hide_color_bar.set(!hide_color_bar),
                                div {
                                    class: "h-6 w-6 rounded-full {background_color} place-self-center"
                                }
                            }
                        }
                        div {
                            class: "{hidden} flex flex-row w-full justify-evenly h-10 rounded-full bg-slate-200",
                            BG_COLORS.iter().map(|color| {
                                color_id += 1;
                                rsx!(
                                    button {
                                        id: "{color_id}",
                                        class: "h-6 w-6 rounded-full {color} place-self-center",
                                        onclick: move |_| {
                                            state.write().game.change_player_color(id, color_id);
                                        }
                                    }
                                )
                            })
                        }
                    ))
                )
            }),
            PlayerInput {},
        }
    ))
}

fn PlayerInput(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    let hide_color_bar = use_state(&cx, || true);
    let color_index = use_state(&cx, || 0);
    let selected_color = BG_COLORS[**color_index];
    let mut color_id = 0;

    if state.read().game.players.len() >= 4 {
        return None;
    }

    let insert_player = get_text(state.read().settings.language, "insert_player").unwrap();

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
            class: "flex flex-row w-full justify-evenly items-center h-16 rounded-full bg-slate-200",
            prevent_default: "onsubmit",
            onsubmit: onsubmit,
            input {
                name: "player-name",
                class: "rounded-full w-3/5 h-8 ring-1 ring-grey text-center self-center",
                placeholder: "{insert_player}",
            }
            button {
                r#type: "submit",
                class: "h-10",
                assets::AddIcon {},
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

fn BeginGameButton(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    let start_button_label = get_text(state.read().settings.language, "start_game_button").unwrap();

    if state.read().game.players.len() < 2 {
        return None;
    };

    log!("Rendering begin game button.");
    cx.render(rsx!(
        button {
            class: "z-10 flex self-center w-max gap-2 border-b-[6px] border-emerald-300",
            onclick: move |_| state.write().start_game(),
            span {
                class: "text-xl font-bold leading-[3rem]",
                "{start_button_label}"
            }
            div {
                class: "h-12",
                assets::arrow_right {}
            }
        }
    ))
}

fn TopBar(cx: Scope) -> Element {
    log!("Rendering top bar.");

    let state = fermi::use_atom_ref(cx, STATE);

    cx.render(rsx!(
        div {
            class: "h-16 grid grid-cols-3 z-10 mx-auto w-full sm:max-w-lg px-8",
            button {
                class: "col-start-1 justify-self-start",
                onclick: move |_| {
                    state.write().screen = Screen::Menu;
                    state.write().checked_storage = false;
                    SessionStorage::delete("session");
                },
                div {
                    class: "h-10 scale-x-[-1]",
                    assets::BackIcon {}
                }
            }
            button {
                class: "col-start-3 justify-self-end",
                onclick: move |_| state.write().screen = Screen::Templates,
                div {
                    class: "h-10",
                    assets::SaveIcon {}
                }
            }
        }
    ))
}
