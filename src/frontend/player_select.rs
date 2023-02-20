use dioxus::events::FormEvent;
use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;
use dioxus::web::use_eval;
use gloo_storage::{SessionStorage, Storage};

use crate::prelude::*;

pub fn screen(cx: Scope) -> Element {
    log!("Rendering player select.");

    let state = use_atom_ref(&cx, STATE);

    let add_players = get_text(state.read().settings.language, "add_players").unwrap();

    cx.render(rsx!(
        top_bar()
        span {
            class: "font-semibold text-lg border-b-2 border-emerald-300 w-max mx-auto mb-8",
            "{add_players}"
        }
        player_list()
        start_game_button()
    ))
}

fn player_list(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    log!("Rendering player list.");

    cx.render(rsx!(
        div {
            class: "flex flex-col gap-6 px-8",
            state.read().game.players.iter().map(|player| {
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
                            div {
                                class: "h-10",
                                assets::remove()
                            }
                        }
                        div {
                            class: "flex flex-col justify-center self-center h-12 w-8",
                            button {
                                class: "place-self-center",
                                onclick: move |_| state.write().game.move_up(id),
                                div {
                                    class: "h-8",
                                    assets::up_icon()
                                },
                            }
                            button {
                                class: "place-self-center",
                                onclick: move |_| state.write().game.move_down(id),
                                div {
                                    class: "h-8 rotate-180",
                                    assets::up_icon()
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
                                    onclick: move |_| {
                                        state.write().game.change_player_color(id, color_id);
                                        hide_color_bar.set(!hide_color_bar);
                                    }
                                }
                            )
                        })
                    }
                )
            }),
            player_input(),
        }
    ))
}

fn player_input(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
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
                assets::add_button(),
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

fn start_game_button(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let start_button_label = get_text(state.read().settings.language, "start_game_button").unwrap();

    if state.read().game.players.len() < 2 {
        return None;
    };

    log!("Rendering begin game button.");
    cx.render(rsx!(
        button {
            class: "z-10 flex absolute self-end w-max gap-2 border-b-[6px] border-emerald-300 right-8 bottom-[30vw]",
            onclick: |_| state.write().start_game(),
            span {
                class: "text-xl font-bold leading-[3rem]",
                "{start_button_label}"
            }
            div {
                class: "h-12",
                assets::arrow_right()
            }
        }
    ))
}

fn top_bar(cx: Scope) -> Element {
    log!("Rendering top bar.");

    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        div {
            class: "h-16 grid grid-cols-3 z-10 mx-auto w-full sm:max-w-lg px-8",
            button {
                class: "col-start-1 justify-self-start",
                onclick: |_| {
                    state.write().screen = Screen::Menu;
                    state.write().checked_storage = false;
                    SessionStorage::delete("session");
                },
                div {
                    class: "h-10 scale-x-[-1]",
                    assets::back()
                }
            }
            button {
                class: "col-start-3 justify-self-end",
                onclick: |_| state.write().screen = Screen::Templates,
                div {
                    class: "h-10",
                    assets::save_icon()
                }
            }
        }
    ))
}
