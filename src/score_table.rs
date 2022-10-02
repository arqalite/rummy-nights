use dioxus::core::UiEvent;
use dioxus::events::FormData;
use dioxus::fermi::{use_atom_state, Atom};
use dioxus::prelude::*;
use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};

use crate::data::{GameStatus, Screen, BORDER_COLORS, CARET_COLORS, COLUMN_NUMBERS, TITLE_COLORS};
use crate::STATE;

static FINAL_SCORE: i32 = 1000;
static GAME_CONTINUES: Atom<bool> = |_| true;
static SHOW_END_ONCE: Atom<bool> = |_| true;

fn get_game_status(cx: Scope) -> GameStatus {
    let state = use_atom_state(&cx, STATE);
    let mut has_reached_max = false;
    let mut no_of_winners = 0;

    for player in &state.players {
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
        for player in &state.players {
            if player.score.values().sum::<i32>() > max {
                max = player.score.values().sum::<i32>();
            }
        }
        for player in &state.players {
            if player.score.values().sum::<i32>() >= max {
                if no_of_winners > 0 {
                    winner_name.push_str(" and ");
                }

                winner_name.push_str(&player.name);

                no_of_winners += 1;
            }
        }
    }

    if has_reached_max && are_columns_equal && no_of_winners == 1 {
        GameStatus::Finished
    } else {
        GameStatus::Ongoing
    }
}

pub fn score_table(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);

    match LocalStorage::set("state", state.get()) {
        Ok(_) => (),
        Err(_) => {
            log!("Failed to save data.");
        }
    };

    let game_continues = use_atom_state(&cx, GAME_CONTINUES);

    let columns = if state.players.len() >= 2 {
        COLUMN_NUMBERS[state.players.len() - 2]
    } else {
        "grid-cols-4"
    };

    let show_end_once = use_atom_state(&cx, SHOW_END_ONCE);

    let game_status = get_game_status(cx);

    match game_status {
        GameStatus::Finished => {
            game_continues.set(false);
            if **show_end_once {
                state.with_mut(|state| {
                    state.screen = Screen::Winner;
                });
                show_end_once.set(false);
            }
        }
        GameStatus::Ongoing | GameStatus::NotStarted => {
            game_continues.set(true);
        }
    };

    let return_to_menu = |_| {
        state.with_mut(|state| {
            state.screen = Screen::Menu;
        });
    };

    let return_to_select = |_| {
        state.with_mut(|state| {
            state.screen = Screen::PlayerSelect;
        });
    };

    cx.render(rsx! (
        div {
            class: "flex flex-col relative mx-auto h-screen w-screen overflow-hidden px-8",
            div {
                class: "z-0 absolute h-screen w-screen",
                div {
                    class: "w-[500px] h-[500px] bottom-[-250px] right-[-250px] absolute rounded-full z-0",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                }
            }
            div {
                class: "z-10 h-16 grid grid-cols-3",
                game_continues.then(|| rsx!(
                    button {
                        class: "mx-auto h-16 col-start-1 relative left-[-50%]",
                        onclick: return_to_select,
                        img {
                            class: "h-8 w-8",
                            src: "img/back.svg",
                        }
                    }
                )),
                button {
                    class: "mx-auto h-16 col-start-3 relative right-[-50%]",
                    onclick: return_to_menu,
                    img {
                        class: "h-8 w-8",
                        src: "img/exit.svg",
                    }
                }
            },
            div {
                class: "w-full rounded-full flex self-center mx-auto mb-8",
                //background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                p {
                    class: "mx-auto self-center font-semibold text-lg text-black border-b-2 border-emerald-300",
                    "Good luck and have fun!"
                }
            },
            div{
                //Main table
                class: "z-10 grid {columns} mx-auto gap-x-4 pt-2",

                state.players.iter().map(|player| {
                    let sum = player.score.values().sum::<i32>().to_string();
                    let background = TITLE_COLORS[player.id-1];
                    let border = BORDER_COLORS[player.id-1];

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
                                            class: "rounded text-md text-center border-b-4 mb-2 h-8 {border}",
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
            }
        }
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
                for player in &mut mut_state.players {
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
    let caret = CARET_COLORS[id - 1];
    let border = BORDER_COLORS[id - 1];

    cx.render(rsx!(
        form {
            onsubmit: onsubmit,
            prevent_default: "onsubmit",
            input {
                class: "{caret} {border} text-sm appearance-none font-light bg-transparent h-8 w-full mb-2 text-center rounded focus:border-b-[8px] border-b-4",
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
