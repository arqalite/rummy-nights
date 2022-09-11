use dioxus::core::UiEvent;
use dioxus::events::FormData;
use dioxus::fermi::use_atom_state;
use dioxus::prelude::*;

use crate::css;
use crate::GameStatus;
use crate::STATE;

static FINAL_SCORE: i32 = 1000;
static GAME_CONTINUES: Atom<bool> = |_| true;

fn get_game_status(cx: Scope) -> GameStatus {
    let state = use_atom_state(&cx, STATE);
    let mut has_reached_max = false;
    let mut no_of_winners = 0;

    for player in state.players.iter() {
        if player.score.values().sum::<i32>() >= FINAL_SCORE {
            has_reached_max = true;
        }
    }

    let mut are_columns_equal = true;

    if has_reached_max {
        for i in 0..state.players.len() - 1 {
            if state.players[i].score.len() != state.players[i + 1].score.len() {
                are_columns_equal = false;
                break;
            }
        }
    }

    let mut winner_name = String::new();
    let mut max = 0;

    if are_columns_equal {
        for player in state.players.iter() {
            if player.score.values().sum::<i32>() > max {
                max = player.score.values().sum::<i32>();
            }
        }
        for player in state.players.iter() {
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
        GameStatus::Finished(winner_name)
    } else {
        GameStatus::Ongoing
    }
}

pub fn show_winner(cx: Scope) -> Element {
    let game_status = get_game_status(cx);

    match game_status {
        GameStatus::Finished(name) => cx.render(rsx! (
            div {
                class: "mt-5",
                p {
                    class: "text-center",
                    "{name} won!"
                }
            }
        )),
        GameStatus::Ongoing | GameStatus::NotStarted => None,
    }
}

pub fn score_table(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);
    let game_continues = use_atom_state(&cx, GAME_CONTINUES);
    let columns = css::COLUMN_NUMBERS[state.players.len() - 2];

    let game_status = get_game_status(cx);

    match game_status {
        GameStatus::Finished(_) => {
            game_continues.set(false);
        }
        GameStatus::Ongoing | GameStatus::NotStarted => {
            game_continues.set(true);
        }
    };

    cx.render(rsx! (
        div {
            class: "h-16 grid grid-cols-3",
            button {
                class: "mx-auto h-16 relative left-[-30%]",
                //onclick:
                img {
                    class: "h-8 w-8",
                    src: "img/back.svg",
                }
            }
            p {
                ""
            }
            button {
                class: "mx-auto h-16 relative right-[-30%]",
                //onclick:
                img {
                    class: "h-8 w-8",
                    src: "img/exit.svg",
                }
            }
        },
        div{
            //Main table
            class: "grid {columns} mx-auto gap-x-5 pt-2",

            state.players.iter().map(|player| {
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
    ))
}

#[derive(Props, PartialEq, Eq)]
pub struct ScoreInputProps {
    id: usize,
}

pub fn score_input(cx: Scope<ScoreInputProps>) -> Element {
    let id = cx.props.id;
    let state = use_atom_state(&cx, STATE);
    let buffer = use_state(&cx, String::new);

    let onsubmit = move |_| {
        if let Ok(number) = buffer.parse::<i32>() {
            state.with_mut(|mut_state| {
                for player in mut_state.players.iter_mut() {
                    if id == player.id {
                        player.score.insert(player.score.len(), number);
                    }
                }
            });
        }
        buffer.set(String::new());
    };

    let oninput = move |evt: UiEvent<FormData>| {
        buffer.set(evt.value.clone());
    };
    let caret = css::CARET_COLORS[id - 1];
    let border = css::BORDER_COLORS[id - 1];

    cx.render(rsx!(
        form {
            onsubmit: onsubmit,
            prevent_default: "onsubmit",
            input {
                class: "{caret} {border} text-sm appearance-none font-light bg-transparent h-8 w-full mb-2 text-center rounded focus:border-b-4 border-b-2",
                style: "-moz-appearance:textfield",
                placeholder: "Insert score",
                value: "{buffer}",
                onsubmit: onsubmit,
                prevent_default: "onsubmit",
                oninput: oninput,
                outline: "none",
                r#type: "number",
            }
        }
    ))
}
