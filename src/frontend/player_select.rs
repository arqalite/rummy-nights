use crate::prelude::*;
use dioxus::prelude::*;

pub fn PlayerSelectScreen() -> Element {
    log!("Rendering player select.");

    rsx!(
        TopBar {}
        div {
            class: "flex flex-col grow pb-8",
            div {
                class: "flex flex-col grow",
                span {
                    class: "font-semibold text-lg border-b-2 border-emerald-300 w-max mx-auto mb-4",
                    {get_text("add_players")}
                }
                PlayerSelectTable {}
            },
            if STATE.read().game.players.len() >= 2 {
                BeginGameButton {}
            }
        }
    )
}

#[component]
fn PlayerItem(player: Player) -> Element {
    let mut show_player_edit = use_signal(|| false);
    let mut hide_color_bar = use_signal(|| true);
    let mut buffer = use_signal(|| player.name.clone());

    let background_color = BG_COLORS[player.color_index];
    let id = player.id;

    log!("Rendering player.");
    rsx!(
        if !show_player_edit() {
            div {
                class: "flex justify-evenly h-14 rounded-full bg-slate-200",
                button {
                    class: "flex justify-center h-8 w-3/5 self-center rounded-full {background_color}",
                    onclick: move |_| show_player_edit.set(!show_player_edit()),
                    p {
                        class: "flex self-center text-white font-semibold",
                        "{player.name}"
                    }
                }
                button {
                    onclick: move |_| STATE.write().game.remove_player(id),
                    div {
                        class: "h-10",
                        assets::RemoveIcon {}
                    }
                }
                div {
                    class: "flex flex-col justify-center self-center h-12 w-8",
                    button {
                        class: "place-self-center",
                        onclick: move |_| STATE.write().game.move_up(id),
                        div {
                            class: "h-8",
                            assets::UpIcon {}
                        },
                    }
                    button {
                        class: "place-self-center",
                        onclick: move |_| STATE.write().game.move_down(id),
                        div {
                            class: "h-8 rotate-180",
                            assets::UpIcon {}
                        },
                    }
                }
            }
        }
        if show_player_edit() {
            form {
                id: "player_name_input",
                class: "flex flex-row w-full justify-evenly items-center h-14 rounded-full bg-slate-200",
                onsubmit: move |evt| {
                    let name = evt.values().get("player-name").unwrap().join("");
                    if !name.is_empty() {
                        STATE.write().edit_player_name(evt, id);
                        show_player_edit.set(!show_player_edit());
                        hide_color_bar.set(true);
                    }
                    document::eval("document.getElementById('player_name_input').reset();");
                },
                input {
                    name: "player-name",
                    class: "rounded-full w-3/5 h-8 ring-1 ring-grey text-center self-center",
                    placeholder: get_text( "insert_player"),
                    oninput: move |evt: FormEvent| buffer.set(evt.value().clone()),
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
                    onclick: move |evt| {
                        evt.prevent_default();
                        hide_color_bar.set(!hide_color_bar());
                    },
                    div {
                        class: "h-6 w-6 rounded-full {background_color} place-self-center"
                    }
                }
            }
            div {
                class: "flex flex-row w-full justify-evenly h-10 rounded-full bg-slate-200",
                hidden: hide_color_bar(),
                for (index, color) in BG_COLORS.iter().enumerate() {
                    button {
                        id: "{index}",
                        class: "h-6 w-6 rounded-full {color} place-self-center",
                        onclick: move |_| STATE.write().game.change_player_color(id, index)
                    }
                }
            }
        }
    )
}

fn PlayerSelectTable() -> Element {
    log!("Rendering player list.");

    rsx!(
        div {
            class: "flex flex-col px-8 grow gap-2",
            for player in STATE.read().game.players.iter() {
                PlayerItem {
                    player: player.clone(),
                }
            }
            if STATE.read().game.players.len() < 4 {
                PlayerInput {}
            }
        }
    )
}

#[component]
fn ColorBar(mut color_signal: Signal<usize>) -> Element {
    rsx!(
        div {
            class: "flex flex-row w-full justify-evenly h-10 mt-2 rounded-full bg-slate-200",
            for (color_id, color) in BG_COLORS.iter().enumerate() {
                button {
                    id: "{color_id}",
                    class: "h-6 w-6 rounded-full {color} place-self-center",
                    onclick: move |_| color_signal.set(color_id),
                }
            }
        }
    )
}

fn PlayerInput() -> Element {
    let mut hide_color_bar = use_signal(|| true);
    let color = use_signal(|| 0);
    let selected_color = BG_COLORS[color()];

    log!("Rendering player input.");

    rsx!(
        div {
            form {
                id: "name_input",
                class: "flex flex-row w-full justify-evenly items-center h-16 rounded-full bg-slate-200",
                onsubmit: move |evt| {
                    let name = evt.values().get("player-name").unwrap().join("");

                    if !name.is_empty() {
                        STATE.write().add_player(name, color());
                    }
                    //Execute some JS on the spot - weird ergonomics but it works
                    document::eval("document.getElementById('name_input').reset();");
                },
                input {
                    name: "player-name",
                    class: "rounded-full w-3/5 h-8 ring-1 ring-grey text-center self-center",
                    placeholder: get_text( "insert_player"),
                }
                button {
                    r#type: "submit",
                    class: "h-10",
                    assets::AddIcon {},
                }
                button {
                    class: "flex flex-col justify-center h-16 w-8",
                    onclick: move |_| hide_color_bar.set(!hide_color_bar()),
                    div {
                        class: "h-6 w-6 rounded-full {selected_color} place-self-center"
                    }
                }
            }
            if !hide_color_bar() {
                ColorBar { 
                    color_signal: color
                }
            }
        }
    )
}

fn BeginGameButton() -> Element {
    log!("Rendering begin game button.");
    rsx!(
        button {
            class: "z-10 flex self-center w-max gap-2 border-b-[6px] border-emerald-300",
            onclick: move |_| STATE.write().start_game(),
            span {
                class: "text-xl font-bold leading-[3rem]",
                {get_text( "start_game_button")}
            }
            div {
                class: "h-12",
                assets::RightArrowIcon {}
            }
        }
    )
}

fn TopBar() -> Element {
    log!("Rendering top bar.");

    rsx!(
        div {
            class: "h-16 grid grid-cols-3 z-10 mx-auto w-full sm:max-w-lg px-8",
            button {
                class: "col-start-1 justify-self-start",
                onclick: move |_| STATE.write().clear_and_go_to_menu(),
                div {
                    class: "h-10 scale-x-[-1]",
                    assets::BackIcon {}
                }
            }
            button {
                class: "col-start-3 justify-self-end",
                onclick: move |_| STATE.write().go_to_screen(Screen::Templates),
                div {
                    class: "h-10",
                    assets::SaveIcon {}
                }
            }
        }
    )
}
