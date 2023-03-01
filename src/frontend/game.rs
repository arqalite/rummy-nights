use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_web::use_eval;
use std::cmp::Ordering;

pub fn GameScreen(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    log!(format!("game status is {:?}", state.read().game.status));
    log!("Rendering game screen.");

    render!(
        NavBar {},
        Banner {},
        PlayerTable {},
        (state.read().settings.use_tile_bonus && state.read().game.status == GameStatus::Ongoing)
            .then(|| rsx!(TileBonusButton {})),
    )
}

fn PlayerTable(cx: Scope) -> Element {
    log!("Rendering player table.");
    let state = fermi::use_atom_ref(cx, STATE);

    render!(
        div {
            //Main table
            class: "z-10 flex justify-evenly gap-x-4 h-[65%] px-8",
            state.read().game.players.iter().map(|player| {
                let player_id = player.id;
                rsx!(
                    div {
                        class: "flex flex-col gap-2 w-full",
                        NameButton {
                            name: player.name.clone(),
                            player_id: player_id,
                            color_index: player.color_index
                        }
                        (!player.score.is_empty()).then(|| rsx!(
                            ScoreTable {
                                player: player.clone()                            }
                        ))
                        div {
                            class: "flex flex-col gap-2 w-full",
                            (state.read().game.status == GameStatus::Ongoing).then(|| rsx!(
                                ScoreInput {
                                    id: player_id,
                                    on_score_input: move |evt: FormEvent| {

                                        if state.write().add_score(evt, player_id) {
                                            let focus_id = match player_id.cmp(&state.read().game.players.len()) {
                                                Ordering::Greater => 5,
                                                Ordering::Equal => 1,
                                                Ordering::Less => player_id + 1,
                                            };
                                            use_eval(cx)(format!(
                                                "document.getElementById('{player_id}').value = '';"
                                            ));
                                            use_eval(cx)(format!("document.getElementById('{focus_id}').focus();"));
                                        }
                                    },
                                    color_index: player.color_index
                                },
                            ))
                            ScoreTotal {
                                color_index: player.color_index,
                                sum: player.sum
                            }
                        }

                    }
                )
            })
        }
    )
}

#[inline_props]
fn NameButton(cx: Scope, name: String, player_id: usize, color_index: usize) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    let (player_name_button_style, player_background, player_text_color, tabindex) =
        if state.read().game.tile_bonus_button_active {
            (
                "pointer-events-auto",
                "bg-white outline outline-1 outline-black",
                "text-black",
                "0",
            )
        } else {
            (
                "pointer-events-none",
                BG_COLORS[*color_index],
                "text-white",
                "-1",
            )
        };

    render!(
        button {
            // Name - first cell
            class: "relative rounded-full h-8 {player_background} {player_name_button_style} w-full",
            tabindex: "{tabindex}",
            onclick: move |_| state.write().grant_bonus(*player_id),
            (state.read().get_dealer() == *player_id).then(|| rsx!(
                DealerPin {}
            ))
            p {
                class: "text-center my-auto {player_text_color} font-semibold",
                "{name}"
            }
        }
    )
}

#[inline_props]
fn ScoreTable(cx: Scope, player: Player) -> Element {
    let mut game_count = 0;
    let mut score_id = 0;

    let player_id = player.id;

    render!(
        div {
            class: "flex flex-col gap-2 w-full overflow-auto scroll-smooth",
            id: "score_{player_id}",
            style: "scrollbar-width: none;",
            player.score.values().map(|score| {
                game_count += 1;
                score_id += 1;

                rsx!(
                    ScoreItem {
                        id: score_id,
                        player_id: player_id,
                        score: *score,
                        color_index: player.color_index,
                        has_bonus: player.bonus.contains_key(&game_count),
                    }
                )
            })
        }
    )
}

#[inline_props]
fn ScoreItem(
    cx: Scope,
    id: i32,
    player_id: usize,
    score: i32,
    color_index: usize,
    has_bonus: bool,
) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    let border = BORDER_COLORS[*color_index];
    let enable_score_editing = state.read().settings.enable_score_editing;

    let bonus_visibility = if *has_bonus { "" } else { "hidden" };

    render!(
        div {
            class: "flex flex-row justify-center relative rounded border-b-4 h-10 {border}",
            (enable_score_editing).then(|| rsx!(
                form {
                    onsubmit: move |evt| state.write().edit_score(evt),
                    prevent_default: "onsubmit",
                    input {
                        name: "score",
                        onsubmit: move |evt| state.write().edit_score(evt),
                        class: "text-lg appearance-none leading-6 font-light bg-transparent h-10 w-full text-center",
                        style: "-moz-appearance:textfield",
                        value: "{score}",
                        outline: "none",
                        r#type: "number",
                    }
                    input {
                        name: "score_id",
                        r#type: "hidden",
                        value: "{id}",
                    }
                    input {
                        name: "player_id",
                        r#type: "hidden",
                        value: "{player_id}",
                    }
                }
            )),
            (!enable_score_editing).then(|| rsx!(
                p {
                    class: "text-lg text-center self-center leading-6",
                    "{score}"
                }
            ))
            div {
                class: "absolute right-0 self-center h-4 {bonus_visibility} rounded-full",
                assets::BonusIcon {}
            }
        }
    )
}

#[inline_props]
fn ScoreTotal(cx: Scope, color_index: usize, sum: i32) -> Element {
    let border = BORDER_COLORS[*color_index];

    render!(
        div {
            //Total box
            class: "rounded border-b-[7px] {border} h-10",
            p {
                class: "text-center text-lg font-semibold",
                "{sum}"
            }
        }
    )
}

#[inline_props]
fn ScoreInput<'a>(
    cx: Scope,
    id: usize,
    color_index: usize,
    on_score_input: EventHandler<'a, FormEvent>,
) -> Element {
    let caret = CARET_COLORS[*color_index];
    let border = BORDER_COLORS[*color_index];

    log!("Rendering score input.");
    render!(
        form {
            onsubmit: |evt| on_score_input.call(evt),
            prevent_default: "onsubmit",
            input {
                name: "score",
                class: "{caret} {border} text-lg appearance-none font-light bg-transparent h-10 w-full text-center rounded focus:border-b-[8px] border-b-4",
                id: "{id}",
                style: "-moz-appearance:textfield",
                outline: "none",
                r#type: "number",
            }
        }
    )
}

fn TileBonusButton(cx: Scope) -> Element {
    log!("Rendering tile bonus menu.");
    let state = fermi::use_atom_ref(cx, STATE);

    let grayscale = if state.read().game.tile_bonus_granted {
        "grayscale"
    } else {
        ""
    };

    render!(
        div {
            class: "z-20 absolute bottom-4 left-4",
            button {
                class: "flex flex-row gap-2 h-14 w-max p-2 border border-slate-100 rounded-full {grayscale}",
                onclick: move |_| state.write().toggle_tile_bonus(),
                box_shadow: if state.read().game.tile_bonus_button_active {
                    "inset 0 2px 4px 0 rgb(0 0 0 / 0.25)"
                } else {
                    "0 1px 3px 0 rgb(0 0 0 / 0.25), 0 1px 2px -1px rgb(0 0 0 / 0.25)"
                },
                div {
                    class: "h-10 w-10 self-center rounded-full",
                    assets::BonusIcon {},
                }
                span {
                    class: "font-semibold text-lg self-center pr-2",
                    get_text(cx, "tile_bonus")
                }
            }
        }
    )
}

fn NavBar(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    let game_status = state.read().game.status;

    let button_position = if game_status == GameStatus::Ongoing {
        "col-start-3 justify-self-end"
    } else {
        "col-start-1 justify-self-start"
    };

    log!("Render nav bar.");
    render!(
        div {
            class: "z-10 h-16 grid grid-cols-3 sm:max-w-lg px-8",
            (game_status == GameStatus::Ongoing).then(|| rsx!(
                button {
                    class: "col-start-1 justify-self-start",
                    onclick: move |_| state.write().go_to_screen(Screen::PlayerSelect),
                    div {
                        class: "h-10 scale-x-[-1]",
                        assets::BackIcon {}
                    }
                }
            )),
            button {
                class: "{button_position}",
                onclick: move |_| state.write().go_to_screen(Screen::Menu),
                div {
                    class: "h-10",
                    assets::HomeIcon {},
                }
            }
            (game_status != GameStatus::Ongoing).then(|| rsx!(
                button {
                    class: "col-start-3 justify-self-end",
                    onclick: move |_| state.write().go_to_screen(Screen::EndGame),
                    div {
                        class: "h-10",
                        assets::BackIcon {}
                    }
                }
            ))
        }
    )
}

fn Banner(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);

    let (banner_text, banner_color) = if state.read().game.status == GameStatus::Finished {
        (
            format!(
                "{} {}",
                state.read().game.get_winner(),
                get_text(cx, "banner_win")
            ),
            String::from("border-green-600"),
        )
    } else if state.read().game.tile_bonus_button_active {
        (
            get_text(cx, "banner_bonus").to_string(),
            String::from("border-cyan-500"),
        )
    } else if state.read().game.warn_incorrect_score {
        (
            get_text(cx, "banner_wrong_score").to_string(),
            String::from("border-red-500"),
        )
    } else {
        (
            get_text(cx, "banner_play").to_string(),
            String::from("border-violet-500"),
        )
    };

    log!("Render banner.");
    render!(
        span {
            class: "mb-8 w-max mx-auto font-semibold text-lg border-b-2 {banner_color}",
            "{banner_text}",
        }
    )
}

fn DealerPin(cx: Scope) -> Element {
    log!("Render dealer pin.");
    render!(
        div {
            class: "h-7 absolute -top-4 -right-4 scale-x-[-1]",
            assets::DealerIcon {}
        }
    )
}
