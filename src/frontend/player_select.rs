use crate::prelude::*;
use dioxus::prelude::*;
use fermi::use_atom_ref;

pub fn PlayerSelectScreen(cx: Scope) -> Element {
    log!("Rendering player select.");
    let state = fermi::use_atom_ref(cx, &STATE);

    render!(
        TopBar {}
        div {
            class: "flex flex-col grow pb-8",
            div {
                class: "flex flex-col grow",
                span {
                    class: "font-semibold text-lg border-b-2 border-emerald-300 w-max mx-auto mb-4",
                    get_text(cx, "add_players")
                }
                PlayerSelectTable {}
            },
            (state.read().game.players.len() >= 2).then(|| rsx!(
                BeginGameButton {}
            ))
        }
    )
}

#[inline_props]
fn PlayerItem(cx: Scope, player: Player) -> Element {
    let state = use_atom_ref(cx, &STATE);

    let show_player_edit = use_state(cx, || false);
    let hide_color_bar = use_state(cx, || true);
    let buffer = use_state(cx, || player.name.clone());

    let background_color = BG_COLORS[player.color_index];
    let id = player.id;
    let mut color_id = 0;

    log!("Rendering player.");
    render!(
        (!show_player_edit).then(|| rsx!(
            div {
                class: "flex justify-evenly h-14 rounded-full bg-slate-200",
                button {
                    class: "flex justify-center h-8 w-3/5 self-center rounded-full {background_color}",
                    onclick: move |_| show_player_edit.set(!show_player_edit),
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
                            assets::UpIcon {}
                        },
                    }
                    button {
                        class: "place-self-center",
                        onclick: move |_| state.write().game.move_down(id),
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
                onsubmit: move |evt| {
                    let name = evt.values.get("player-name").unwrap().join("");
                    if !name.is_empty() {
                        state.write().edit_player_name(evt, id);
                        show_player_edit.set(!show_player_edit);
                        hide_color_bar.set(true);
                    }
                    let _ = use_eval(cx)("document.getElementById('player_name_input').reset();");
                },
                input {
                    name: "player-name",
                    class: "rounded-full w-3/5 h-8 ring-1 ring-grey text-center self-center",
                    placeholder: get_text(cx, "insert_player"),
                    oninput: move |evt: FormEvent| buffer.set(evt.value.clone()),
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
                class: "flex flex-row w-full justify-evenly h-10 rounded-full bg-slate-200",
                hidden: **hide_color_bar,
                BG_COLORS.iter().map(|color| {
                    color_id += 1;
                    rsx!(
                        button {
                            id: "{color_id}",
                            class: "h-6 w-6 rounded-full {color} place-self-center",
                            onclick: move |_| state.write().game.change_player_color(id, color_id)
                        }
                    )
                })
            }
        ))
    )
}

fn PlayerSelectTable(cx: Scope) -> Element {
    log!("Rendering player list.");
    let state = fermi::use_atom_ref(cx, &STATE);

    render!(
        div {
            class: "flex flex-col px-8 grow gap-2",
            state.read().game.players.iter().map(|player| {
                rsx!(
                    PlayerItem {
                        player: player.clone(),
                    }
                )
            }),
            (state.read().game.players.len() < 4).then(|| rsx!(
                PlayerInput {}
            ))
        }
    )
}

fn PlayerInput(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, &STATE);
    let hide_color_bar = use_state(cx, || true);
    let color_index = use_state(cx, || 0);
    let selected_color = BG_COLORS[**color_index];
    let mut color_id = 0;

    log!("Rendering player input.");

    render!(
        div {
            form {
                id: "name_input",
                class: "flex flex-row w-full justify-evenly items-center h-16 rounded-full bg-slate-200",
                onsubmit: move |evt| {
                    let name = evt.values.get("player-name").unwrap().join("");

                    if !name.is_empty() {
                        state.write().add_player(name, **color_index);
                    }
                    //Execute some JS on the spot - weird ergonomics but it works
                    let _ = use_eval(cx)("document.getElementById('name_input').reset();");
                },
                input {
                    name: "player-name",
                    class: "rounded-full w-3/5 h-8 ring-1 ring-grey text-center self-center",
                    placeholder: get_text(cx, "insert_player"),
                }
                button {
                    r#type: "submit",
                    class: "h-10",
                    assets::AddIcon {},
                }
                button {
                    class: "flex flex-col justify-center h-16 w-8",
                    onclick: move |_| hide_color_bar.set(!hide_color_bar),
                    div {
                        class: "h-6 w-6 rounded-full {selected_color} place-self-center"
                    }
                }
            }
            (!hide_color_bar).then(|| rsx!(
                div {
                    class: "flex flex-row w-full justify-evenly h-10 mt-2 rounded-full bg-slate-200",
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
            ))
        }
    )
}

fn BeginGameButton(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, &STATE);

    log!("Rendering begin game button.");
    render!(
        button {
            class: "z-10 flex self-center w-max gap-2 border-b-[6px] border-emerald-300",
            onclick: move |_| state.write().start_game(),
            span {
                class: "text-xl font-bold leading-[3rem]",
                get_text(cx, "start_game_button")
            }
            div {
                class: "h-12",
                assets::RightArrowIcon {}
            }
        }
    )
}

fn TopBar(cx: Scope) -> Element {
    log!("Rendering top bar.");
    let state = fermi::use_atom_ref(cx, &STATE);

    render!(
        div {
            class: "h-16 grid grid-cols-3 z-10 mx-auto w-full sm:max-w-lg px-8",
            button {
                class: "col-start-1 justify-self-start",
                onclick: move |_| state.write().clear_and_go_to_menu(),
                div {
                    class: "h-10 scale-x-[-1]",
                    assets::BackIcon {}
                }
            }
            button {
                class: "col-start-3 justify-self-end",
                onclick: move |_| state.write().go_to_screen(Screen::Templates),
                div {
                    class: "h-10",
                    assets::SaveIcon {}
                }
            }
        }
    )
}
