use dioxus::core::UiEvent;
use dioxus::events::{FormData};
use dioxus::fermi::{use_atom_state, Atom};
use dioxus::prelude::*;
use dioxus::web::use_eval;
use gloo_storage::{LocalStorage, SessionStorage, Storage};

use crate::data::{GameStatus, Screen, BORDER_COLORS, CARET_COLORS, TITLE_COLORS};
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
        Err(_) => ()
    };

    match SessionStorage::set("session", true) {
        Ok(_) => (),
        Err(_) => ()
    }

    let game_continues = use_atom_state(&cx, GAME_CONTINUES);

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
            class: "flex flex-col grow h-screen w-screen relative overflow-hidden px-[5%]",
            div {
                class: "z-0 absolute h-screen w-screen",
                div {
                    class: "w-[500px] h-[500px] bottom-[-250px] right-[-250px] absolute rounded-full z-0",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                }
            }
            div {
                class: "z-10 h-16 grid grid-cols-3 mx-auto w-full sm:max-w-lg",
                game_continues.then(|| rsx!(
                    button {
                        class: "col-start-1 justify-self-start",
                        onclick: return_to_select,
                        img {
                            class: "h-8 w-8",
                            src: "img/back.svg",
                        }
                    }
                )),
                button {
                    class: "col-start-3 justify-self-end",
                    onclick: return_to_menu,
                    img {
                        class: "h-8 w-8",
                        src: "img/exit.svg",
                    }
                }
            },
            div {
                class: "mb-4 w-max mx-auto",
                span {
                    class: "font-semibold text-lg border-b-2 border-emerald-300",
                    "Good luck and have fun!",
                }
            }
            div{
                //Main table
                class: "z-10 flex justify-evenly gap-x-4 pt-2 overflow-auto mx-auto w-full sm:max-w-lg",

                state.players.iter().map(|player| {
                    let sum = player.score.values().sum::<i32>().to_string();
                    let background = TITLE_COLORS[player.id-1];
                    let border = BORDER_COLORS[player.id-1];

                    rsx!(
                        div{
                            //Column for each player
                            div {
                                // Name - first cell
                                class: "rounded-full h-8 {background} py-1",
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
                                            class: "rounded text-lg text-center border-b-4 h-9 mt-2 {border}",
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
                                class: "rounded border-b-[7px] {border} h-9 mt-2",
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
    let execute = use_eval(&cx);

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

        if id < state.players.len() {
            let new_id = id + 1;

            execute("document.getElementById('".to_string() + &new_id.to_string() + "').focus();")
        } else if id == state.players.len() {
            execute("document.getElementById('1').focus();".to_string())
        }
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
                class: "{caret} {border} text-lg appearance-none font-light bg-transparent h-9 mt-2 w-full text-center rounded focus:border-b-[8px] border-b-4",
                id: "{id}",
                style: "-moz-appearance:textfield",
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
