use crate::backend::prelude::STATE;
use crate::prelude::*;
use dioxus::prelude::*;
use std::cmp::Ordering;

pub fn GameScreen() -> Element {
    log!(format!("game status is {:?}", STATE.read().game.status));
    log!("Rendering game screen.");

    rsx!(
        NavBar {},
        Banner {},
        PlayerTable {},
        div {
            class: "z-20 absolute bottom-4 left-4 flex flex-col gap-2",
            DoubleGameButton {},
            if STATE.read().settings.use_tile_bonus && STATE.read().game.status == GameStatus::Ongoing {
                TileBonusButton {}
            }
        }

    )
}

fn PlayerTable() -> Element {
    log!("Rendering player table.");

    let players = STATE.read().game.players.clone();

    rsx!(
        div {
            //Main table
            class: "z-10 flex justify-evenly gap-x-4 h-max max-h-[50%] px-8",
            for player in players.clone().iter() {
                div {
                    class: "flex flex-col gap-2 w-full",
                    NameButton {
                        name: player.name.clone(),
                        player_id: player.id,
                        color_index: player.color_index
                    }
                    if !player.score.is_empty() {
                        ScoreTable {
                            player: player.clone()
                        }
                    }
                    div {
                        class: "flex flex-col gap-2 w-full",
                        if STATE.read().game.status == GameStatus::Ongoing {
                            ScoreInput {
                                id: player.id,
                                color_index: player.color_index
                            },
                        }
                        ScoreTotal {
                            color_index: player.color_index,
                            sum: player.sum
                        }
                    }

                }
            }
        },
        if STATE.read().game.double_game_button_active {
            div {
                class: "px-8 mt-4",
                NameButton {
                    name: String::from(get_text( "everyone")),
                    player_id: 0,
                    color_index: 3,
                }
            }
        }
    )
}

#[component]
fn NameButton(name: String, player_id: usize, color_index: usize) -> Element {
    let is_tile_bonus_active = STATE.read().game.tile_bonus_button_active;
    let is_double_game_button_active = STATE.read().game.double_game_button_active;

    let (player_name_button_style, player_background, player_text_color, tabindex) =
        if is_tile_bonus_active || is_double_game_button_active {
            (
                "pointer-events-auto",
                "bg-white outline outline-1 outline-black",
                "text-black",
                "0",
            )
        } else {
            (
                "pointer-events-none",
                BG_COLORS[color_index],
                "text-white",
                "-1",
            )
        };

    rsx!(
        button {
            // Name - first cell
            class: "relative rounded-full h-8 {player_background} {player_name_button_style} w-full",
            tabindex: "{tabindex}",
            onclick: move |_| {
                if is_tile_bonus_active {
                    STATE.write().grant_bonus(player_id);
                } else if is_double_game_button_active {
                    if player_id == 0 {
                        STATE.write().double_game_total();
                    } else {
                        STATE.write().double_game_for_player(player_id);
                    }
                };

            },
            if STATE.read().get_dealer() == player_id {
                DealerPin {}
            }
            p {
                class: "text-center my-auto {player_text_color} font-semibold",
                "{name}"
            }
        }
    )
}

#[component]
fn ScoreTable(player: Player) -> Element {
    rsx!(
        div {
            class: "flex flex-col gap-2 w-full overflow-auto scroll-smooth",
            id: "score_{player.id}",
            style: "scrollbar-width: none;",
            for (index, score) in player.score.values().enumerate() {
                ScoreItem {
                    id: index.try_into().unwrap_or(0),
                    player_id: player.id,
                    score: *score,
                    color_index: player.color_index,
                    has_bonus: player.bonus.contains_key(&index),
                    has_double: player.doubles.contains_key(&index),
                }
            }
        }
    )
}

#[component]
fn ScoreItem(
    id: i32,
    player_id: usize,
    score: i32,
    color_index: usize,
    has_bonus: bool,
    has_double: bool,
) -> Element {
    let border = BORDER_COLORS[color_index];
    let enable_score_editing = STATE.read().settings.enable_score_editing;

    let bonus_visibility = if has_bonus { "" } else { "hidden" };
    let double_visibility = if has_double { "" } else { "hidden" };

    rsx!(
        div {
            class: "flex flex-row justify-center relative rounded border-b-4 h-10 {border}",
            if enable_score_editing {
                form {
                    onsubmit: move |evt| STATE.write().edit_score(evt),

                    input {
                        name: "score",
                        onsubmit: move |evt| STATE.write().edit_score(evt),
                        class: "text-lg appearance-none leading-6 font-light bg-transparent h-10 w-full text-center",
                        style: "-moz-appearance:textfield",
                        value: "{score}",
                        outline: "none",
                        r#type: "tel",
                    }
                    input {
                        name: "score_id",
                        r#type: "hidden",
                        value: "{id}",
                    }
                    input {
                        name: "player.id",
                        r#type: "hidden",
                        value: "{player_id}",
                    }
                }
            } else {
                p {
                    class: "text-lg text-center self-center leading-6",
                    "{score}"
                }
            }
            div {
                class: "absolute left-0 self-center h-4 {bonus_visibility} rounded-full",
                assets::BonusIcon {}
            }
            div {
                class: "absolute right-0 self-center {double_visibility} font-bold text-green-600 rounded-full text-sm",
                "x2"
            }
        }
    )
}

#[component]
fn ScoreTotal(color_index: usize, sum: i32) -> Element {
    let border = BORDER_COLORS[color_index];

    rsx!(
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

#[component]
fn ScoreInput(id: usize, color_index: usize) -> Element {
    let caret = CARET_COLORS[color_index];
    let border = BORDER_COLORS[color_index];

    let on_score_input = move |evt: FormEvent| {
        if STATE.write().add_score(evt, id) {
            let focus_id = match id.cmp(&STATE.read().game.players.len()) {
                Ordering::Greater => 5,
                Ordering::Equal => 1,
                Ordering::Less => id + 1,
            };
            let focus_score = format!("document.getElementById('{focus_id}').focus();");
            document::eval(&format!("document.getElementById('{0}').value = '';", id));
            document::eval(&focus_score);
        }
    };

    log!("Rendering score input.");
    rsx!(
        form {
            onsubmit: on_score_input,
            input {
                name: "score",
                class: "{caret} {border} text-lg appearance-none font-light bg-transparent h-10 w-full text-center rounded focus:border-b-[8px] border-b-4",
                id: "{id}",
                style: "-moz-appearance:textfield",
                outline: "none",
                r#type: "tel",
            }
        }
    )
}

fn DoubleGameButton() -> Element {
    log!("Rendering double game menu.");

    let grayscale = if STATE.read().game.double_game_granted {
        "grayscale"
    } else {
        ""
    };

    rsx!(
        button {
            class: "flex flex-row gap-2 h-14 w-full p-2 border border-slate-100 rounded-full {grayscale}",
            onclick: move |_| STATE.write().toggle_double_game_button(),
            box_shadow: if STATE.read().game.double_game_button_active {
                "inset 0 2px 4px 0 rgb(0 0 0 / 0.25)"
            } else {
                "0 1px 3px 0 rgb(0 0 0 / 0.25), 0 1px 2px -1px rgb(0 0 0 / 0.25)"
            },
            div {
                class: "h-10 w-10 text-2xl font-bold text-green-600 self-baseline rounded-full",
                "x2"
            }
            span {
                class: "font-semibold text-lg self-center pr-2",
                {get_text("double_game")}
            }
        }
    )
}

fn TileBonusButton() -> Element {
    log!("Rendering tile bonus menu.");

    let grayscale = if STATE.read().game.tile_bonus_granted {
        "grayscale"
    } else {
        ""
    };

    rsx!(
        button {
            class: "flex flex-row gap-2 h-14 w-full p-2 border border-slate-100 rounded-full {grayscale}",
            onclick: move |_| STATE.write().toggle_tile_bonus(),
            box_shadow: if STATE.read().game.tile_bonus_button_active {
                "inset 0 2px 4px 0 rgb(0 0 0 / 0.25)"
            } else {
                "0 1px 3px 0 rgb(0 0 0 / 0.25), 0 1px 2px -1px rgb(0 0 0 / 0.25)"
            },
            div {
                class: "h-8 w-8 self-center rounded-full",
                assets::BonusIcon {},
            }
            span {
                class: "font-semibold text-lg self-center pr-2",
                {get_text("tile_bonus")}
            }
        }
    )
}

fn NavBar() -> Element {
    let game_status = STATE.read().game.status;

    let button_position = if game_status == GameStatus::Ongoing {
        "col-start-3 justify-self-end"
    } else {
        "col-start-1 justify-self-start"
    };

    log!("Render nav bar.");
    rsx!(
        div {
            class: "z-10 h-16 grid grid-cols-3 sm:max-w-lg px-8",
            if game_status == GameStatus::Ongoing {
                button {
                    class: "col-start-1 justify-self-start",
                    onclick: move |_| STATE.write().go_to_screen(Screen::PlayerSelect),
                    div {
                        class: "h-10 scale-x-[-1]",
                        assets::BackIcon {}
                    }
                }
            }
            button {
                class: "{button_position}",
                onclick: move |_| STATE.write().go_to_screen(Screen::Menu),
                div {
                    class: "h-10",
                    assets::HomeIcon {},
                }
            }
            if game_status != GameStatus::Ongoing {
                button {
                    class: "col-start-3 justify-self-end",
                    onclick: move |_| STATE.write().go_to_screen(Screen::EndGame),
                    div {
                        class: "h-10",
                        assets::BackIcon {}
                    }
                }
            }
        }
    )
}

fn Banner() -> Element {
    let (banner_text, banner_color) = if STATE.read().game.status == GameStatus::Finished {
        (
            format!(
                "{} {}",
                STATE.read().game.get_winner(),
                get_text("banner_win")
            ),
            String::from("border-green-600"),
        )
    } else if STATE.read().game.tile_bonus_button_active {
        (
            get_text("banner_bonus").to_string(),
            String::from("border-pink-500"),
        )
    } else if STATE.read().game.double_game_button_active {
        (
            get_text("banner_double").to_string(),
            String::from("border-cyan-500"),
        )
    } else if STATE.read().game.warn_incorrect_score {
        (
            get_text("banner_wrong_score").to_string(),
            String::from("border-red-500"),
        )
    } else {
        (
            get_text("banner_play").to_string(),
            String::from("border-violet-500"),
        )
    };

    log!("Render banner.");
    rsx!(
        span {
            class: "mb-8 w-max mx-auto font-semibold text-lg border-b-2 {banner_color}",
            "{banner_text}",
        }
    )
}

fn DealerPin() -> Element {
    log!("Render dealer pin.");
    rsx!(
        div {
            class: "h-7 absolute -top-4 -right-4 scale-x-[-1]",
            assets::DealerIcon {}
        }
    )
}
