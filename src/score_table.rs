use dioxus::core::UiEvent;
use dioxus::events::*;
use dioxus::fermi::use_atom_state;
use dioxus::prelude::*;

use crate::statics;
use crate::PLAYERS;

static FINAL_SCORE: i32 = 1000;

fn is_game_ongoing(cx: Scope) -> bool {
    let state = use_atom_state(&cx, PLAYERS);
    let mut game_ongoing = true;
    let mut has_reached_max = false;

    for player in state.iter() {
        if player.score.iter().sum::<i32>() >= FINAL_SCORE {
            has_reached_max = true;
        }
    }

    if has_reached_max {
        let mut min = 9999;
        let mut max = 0;

        for player in state.iter() {
            if player.score.len() > max {
                max = player.score.len();
            }
            if player.score.len() < min {
                min = player.score.len();
            }
        }

        if min == max {
            game_ongoing = false;
        }
    }

    game_ongoing
}

fn get_winner(cx: Scope) -> String {
    let state = use_atom_state(&cx, PLAYERS);
    let mut winner_name = String::new();
    let mut max = 0;

    for player in state.iter() {
        if player.score.iter().sum::<i32>() > max {
            max = player.score.iter().sum::<i32>();
            winner_name = player.name.to_string();
        }
    }

    winner_name
}

pub fn score_table(cx: Scope) -> Element {
    let state = use_atom_state(&cx, PLAYERS);
    let columns = statics::COLUMN_NUMBERS[state.len() - 2];

    let game_continues = is_game_ongoing(cx);
    let show_winner = !game_continues;
    let winner = if game_continues {
        String::new()
    } else {
        get_winner(cx)
    };

    cx.render(rsx! (
        div{
            //Main table
            class: "grid {columns} mx-auto px-5 max-w-md mt-16 gap-x-5",

            state.iter().map(|player| {
                let sum = player.score.iter().sum::<i32>().to_string();
                let background = statics::TITLE_COLORS[player.id-1];
                let border = statics::BORDER_COLORS[player.id-1];

                rsx!(
                    div{
                        //Column for each player
                        class: "",
                        div {
                            // Name - first cell
                            class: "rounded-full h-8 {background} py-1 mb-2 shadow",
                            p {
                                class: "text-center my-auto",
                                "{player.name}"
                            }
                        }
                        div {
                            //Scores - dynamic
                            player.score.iter().map(|score| {
                                let score_text = score.to_string();
                                rsx!(
                                    p {
                                        class: "rounded text-center border-b-2 mb-2 h-8 {border}",
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
                            class: "rounded border-y-4 {border} h-8",
                            p {
                                class: "text-center font-semibold",
                                "{sum}"
                            }
                        }
                    }
                )
            })
        },
        show_winner.then(|| 
            rsx! (
                div {
                    class: "mt-5",
                    p {
                        class: "text-center",
                        "{winner} wins!"
                    }
                }
            )
        )
    ))
}

#[derive(Props, PartialEq, Eq)]
pub struct ScoreInputProps {
    id: usize,
}

pub fn score_input(cx: Scope<ScoreInputProps>) -> Element {
    let id = cx.props.id;
    let buffer = use_state(&cx, String::new);
    let onfocusout = move |_| {
        if let Ok(number) = buffer.parse::<i32>() {
            let state = use_atom_state(&cx, PLAYERS);

            state.with_mut(|mut_state| {
                for player in mut_state.iter_mut() {
                    if id == player.id {
                        player.score.push(number);
                    }
                }
            });
        }
        buffer.set(String::new());
    };

    let onkeypress = move |evt: UiEvent<KeyboardData>| {
        if evt.key.as_str() == "Enter" {
            if let Ok(number) = buffer.parse::<i32>() {
                let state = use_atom_state(&cx, PLAYERS);

                state.with_mut(|mut_state| {
                    for player in mut_state.iter_mut() {
                        if id == player.id {
                            player.score.push(number);
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
    let caret = statics::CARET_COLORS[id - 1];
    let border = statics::FOCUS_OUTLINE_COLORS[id - 1];

    cx.render(rsx!(
        input {
            class: "{caret} appearance-none bg-transparent h-8 w-full mb-2 text-center rounded-full focus:outline-1 {border}",
            placeholder: "Insert score",
            value: "{buffer}",
            onkeypress: onkeypress,
            oninput: oninput,
            onfocusout: onfocusout,
        }
    ))
}
