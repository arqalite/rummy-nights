use crate::prelude::*;
use dioxus::prelude::*;

#[inline_props]
pub fn PlayerSelectScreen<'a>(
    cx: Scope,
    lang_code: usize,
    game: Game,
    on_click_begin: EventHandler<'a, MouseEvent>,
    on_click_back: EventHandler<'a, MouseEvent>,
    on_click_template: EventHandler<'a, MouseEvent>,
    on_add_player: EventHandler<'a, (FormEvent, usize)>,
    on_edit_player: EventHandler<'a, (FormEvent, usize)>,
    on_remove_player: EventHandler<'a, (MouseEvent, usize)>,
    on_move_up: EventHandler<'a, (MouseEvent, usize)>,
    on_move_down: EventHandler<'a, (MouseEvent, usize)>,
    on_color_change: EventHandler<'a, (MouseEvent, usize, usize)>,
) -> Element {
    log!("Rendering player select.");

    render!(
        TopBar {
            on_click_back: |evt| on_click_back.call(evt),
            on_click_template: |evt| on_click_template.call(evt)
        }
        div {
            class: "flex flex-col grow pb-8",
            div {
                class: "flex flex-col grow",
                span {
                    class: "font-semibold text-lg border-b-2 border-emerald-300 w-max mx-auto mb-4",
                    get_text(*lang_code, "add_players")
                }
                PlayerSelectTable {
                    game: game.clone(),
                    lang_code: *lang_code,
                    on_add_player: |evt| on_add_player.call(evt),
                    on_edit_player: |evt| on_edit_player.call(evt),
                    on_remove_player: |evt| on_remove_player.call(evt),
                    on_move_up: |evt| on_move_up.call(evt),
                    on_move_down: |evt| on_move_down.call(evt),
                    on_color_change: |evt| on_color_change.call(evt)
                }
            },
            (game.players.len() >= 2).then(|| rsx!(
                BeginGameButton {
                    lang_code: *lang_code,
                    on_click_begin: |evt| on_click_begin.call(evt),
                }
            ))
        }
    )
}

#[inline_props]
fn PlayerSelectTable<'a>(
    cx: Scope,
    game: Game,
    lang_code: usize,
    on_add_player: EventHandler<'a, (FormEvent, usize)>,
    on_edit_player: EventHandler<'a, (FormEvent, usize)>,
    on_remove_player: EventHandler<'a, (MouseEvent, usize)>,
    on_move_up: EventHandler<'a, (MouseEvent, usize)>,
    on_move_down: EventHandler<'a, (MouseEvent, usize)>,
    on_color_change: EventHandler<'a, (MouseEvent, usize, usize)>,
) -> Element {
    log!("Rendering player list.");

    render!(
        div {
            class: "flex flex-col px-8 grow gap-2",
            game.players.iter().map(|player| {
                let background_color = BG_COLORS[player.color_index];
                let id = player.id;

                let show_player_edit = use_state(cx, || false);
                let hide_color_bar = use_state(cx, || true);

                let hidden = if **hide_color_bar {
                    "hidden"
                } else {
                    ""
                };

                let mut color_id = 0;

                let buffer = use_state(cx, || player.name.clone());

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
                                onclick: move |evt| on_remove_player.call((evt, id)),
                                div {
                                    class: "h-10",
                                    assets::RemoveIcon {}
                                }
                            }
                            div {
                                class: "flex flex-col justify-center self-center h-12 w-8",
                                button {
                                    class: "place-self-center",
                                    onclick: move |evt| on_move_up.call((evt, id)),
                                    div {
                                        class: "h-8",
                                        assets::UpIcon {}
                                    },
                                }
                                button {
                                    class: "place-self-center",
                                    onclick: move |evt| on_move_down.call((evt, id)),
                                    div {
                                        class: "h-8 rotate-180",
                                        assets::UpIcon {}
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
                            onsubmit: move |evt| {
                                let name = evt.values.get("player-name").unwrap().to_string();
                                if !name.is_empty() {
                                    on_edit_player.call((evt, id));
                                    show_player_edit.set(!show_player_edit);
                                    hide_color_bar.set(true);
                                }
                            },
                            input {
                                name: "player-name",
                                class: "rounded-full w-3/5 h-8 ring-1 ring-grey text-center self-center",
                                placeholder: get_text(*lang_code, "insert_player"),
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
                                assets::OkayIcon {},
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
                                        onclick: move |evt| on_color_change.call((evt, id, color_id))
                                    }
                                )
                            })
                        }
                    ))
                )
            }),
            (game.players.len() < 4).then(|| rsx!(
                PlayerInput {
                    lang_code: *lang_code,
                    on_add_player: |evt| on_add_player.call(evt)
                },
            ))
        }
    )
}

#[inline_props]
fn PlayerInput<'a>(
    cx: Scope,
    lang_code: usize,
    on_add_player: EventHandler<'a, (FormEvent, usize)>,
) -> Element {
    let hide_color_bar = use_state(cx, || true);
    let color_index = use_state(cx, || 0);
    let selected_color = BG_COLORS[**color_index];
    let mut color_id = 0;

    let hidden = if **hide_color_bar { "hidden" } else { "" };

    log!("Rendering player input.");

    render!(
        div {
            form {
                id: "name_input",
                class: "flex flex-row w-full justify-evenly items-center h-16 rounded-full bg-slate-200",
                prevent_default: "onsubmit",
                onsubmit: |evt| on_add_player.call((evt, **color_index)),
                input {
                    name: "player-name",
                    class: "rounded-full w-3/5 h-8 ring-1 ring-grey text-center self-center",
                    placeholder: get_text(*lang_code, "insert_player"),
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
    )
}

#[inline_props]
fn BeginGameButton<'a>(
    cx: Scope,
    lang_code: usize,
    on_click_begin: EventHandler<'a, MouseEvent>,
) -> Element {
    log!("Rendering begin game button.");
    render!(
        button {
            class: "z-10 flex self-center w-max gap-2 border-b-[6px] border-emerald-300",
            onclick: |evt| on_click_begin.call(evt),
            span {
                class: "text-xl font-bold leading-[3rem]",
                get_text(*lang_code, "start_game_button")
            }
            div {
                class: "h-12",
                assets::RightArrowIcon {}
            }
        }
    )
}

#[inline_props]
fn TopBar<'a>(
    cx: Scope,
    on_click_back: EventHandler<'a, MouseEvent>,
    on_click_template: EventHandler<'a, MouseEvent>,
) -> Element {
    log!("Rendering top bar.");

    render!(
        div {
            class: "h-16 grid grid-cols-3 z-10 mx-auto w-full sm:max-w-lg px-8",
            button {
                class: "col-start-1 justify-self-start",
                onclick: |evt| on_click_back.call(evt),
                div {
                    class: "h-10 scale-x-[-1]",
                    assets::BackIcon {}
                }
            }
            button {
                class: "col-start-3 justify-self-end",
                onclick: |evt| on_click_template.call(evt),
                div {
                    class: "h-10",
                    assets::SaveIcon {}
                }
            }
        }
    )
}
