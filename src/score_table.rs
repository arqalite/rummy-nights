use dioxus::core::UiEvent;
use dioxus::events::{FormData, KeyboardData};
use dioxus::fermi::use_atom_state;
use dioxus::prelude::*;

use crate::css;
use crate::PLAYERS;

static FINAL_SCORE: i32 = 1000;
static GAME_CONTINUES: Atom<bool> = |_| true;

#[derive(PartialEq)]
enum GameStatus {
    Ongoing,
    Winner(String),
}

fn get_game_status(cx: Scope) -> GameStatus {
    let players = use_atom_state(&cx, PLAYERS);
    let mut has_reached_max = false;
    let mut no_of_winners = 0;

    for player in players.iter() {
        if player.score.values().sum::<i32>() >= FINAL_SCORE {
            has_reached_max = true;
        }
    }

    let mut are_columns_equal = true;

    if has_reached_max {
        for i in 0..players.len() - 1 {
            if players[i].score.len() != players[i + 1].score.len() {
                are_columns_equal = false;
                break;
            }
        }
    }

    let mut winner_name = String::new();
    let mut max = 0;

    if are_columns_equal {
        for player in players.iter() {
            if player.score.values().sum::<i32>() > max {
                max = player.score.values().sum::<i32>();
            }
        }
        for player in players.iter() {
            if player.score.values().sum::<i32>() >= max {
                if no_of_winners > 0 {
                    winner_name.push_str(" and ");
                }

                winner_name.push_str(&player.name);

                no_of_winners += 1;
            }
        }
    }

    if has_reached_max && are_columns_equal {
        GameStatus::Winner(winner_name)
    } else {
        GameStatus::Ongoing
    }
}

pub fn show_winner(cx: Scope) -> Element {
    let game_status = get_game_status(cx);

    match game_status {
        GameStatus::Winner(name) => {
            cx.render(rsx! (
                div {
                    class: "mt-5",
                    p {
                        class: "text-center",
                        "{name} won!"
                    }
                }
            ))
        }
        GameStatus::Ongoing => {
            None
        }
    }
}

pub fn score_table(cx: Scope) -> Element {
    let state = use_atom_state(&cx, PLAYERS);
    let game_continues = use_atom_state(&cx, GAME_CONTINUES);
    let columns = css::COLUMN_NUMBERS[state.len() - 2];

    let game_status = get_game_status(cx);

    match game_status {
        GameStatus::Winner(_) => {
            game_continues.set(false);
        }
        GameStatus::Ongoing => {
            game_continues.set(true);
        }
    };

    cx.render(rsx! (
        div{
            //Main table
            class: "grid {columns} mx-auto px-5 max-w-md mt-16 gap-x-5",

            state.iter().map(|player| {
                let sum = player.score.values().sum::<i32>().to_string();
                let background = css::TITLE_COLORS[player.id-1];
                let border = css::BORDER_COLORS[player.id-1];

                rsx!(
                    div{
                        //Column for each player
                        class: "",
                        div {
                            // Name - first cell
                            class: "rounded-full h-8 {background} py-1 mb-2 shadow",
                            p {
                                class: "text-center my-auto text-white font-semibold",
                                "{player.name}"
                            }
                        }
                        div {
                            //Scores - dynamic
                            player.score.values().map(|score| {
                                let score_text = score.to_string();
                                rsx!(
                                    p {
                                        class: "rounded text-sm text-center border-b-2 mb-2 h-8 {border}",
                                        "{score_text}"
                                    }
                                )
                            })
                        }
                        div {
                            //Input box
                            game_continues.then( ||
                                rsx!(
                                crate::score_table::score_input{
                                    id: player.id
                                })
                            )
                        }
                        div {
                            //Total box
                            class: "rounded text-sm border-b-[7px] {border} h-8",
                            p {
                                class: "text-center text-lg font-semibold",
                                "{sum}"
                            }
                        }
                    }
                )
            })
        },
        crate::score_table::show_winner(),
            div {
                    class: "hidden absolute w-96 h-56 -bottom-[2%] -right-[30%]",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    border_radius: "111px",
                    transform: "rotate(-50deg)",
            } //pill thing
    ))
}

#[derive(Props, PartialEq, Eq)]
pub struct ScoreInputProps {
    id: usize,
}

pub fn score_input(cx: Scope<ScoreInputProps>) -> Element {
    let id = cx.props.id;
    let buffer = use_state(&cx, String::new);

    let onkeypress = move |evt: UiEvent<KeyboardData>| {
        if evt.key.as_str() == "Enter" {
            if let Ok(number) = buffer.parse::<i32>() {
                let state = use_atom_state(&cx, PLAYERS);

                state.with_mut(|mut_state| {
                    for player in mut_state.iter_mut() {
                        if id == player.id {
                            player.score.insert(player.score.len(), number);
                        }
                    }
                });
            }
            buffer.set(String::new());
        }
    };
    let oninput = move |evt: UiEvent<FormData>| {
        buffer.set(evt.value.clone());
    };
    let caret = css::CARET_COLORS[id - 1];
    let border = css::BORDER_COLORS[id-1];

    cx.render(rsx!(
        input {
            class: "{caret} {border} text-sm appearance-none font-light bg-transparent h-8 w-full mb-2 text-center rounded focus:border-b-4 border-b-2",
            placeholder: "Insert score",
            value: "{buffer}",
            onkeypress: onkeypress,
            oninput: oninput,
            outline: "none",
        }
    ))
}

