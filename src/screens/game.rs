use dioxus::core::UiEvent;
use dioxus::events::FormData;
use dioxus::fermi::{use_atom_ref, use_atom_state};
use dioxus::prelude::*;
use dioxus::web::use_eval;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use std::cmp::Ordering;
use std::ops::Not;

use crate::data::tailwind_classes;
use crate::prelude::*;

static FINAL_SCORE: i32 = 1000;
static GAME_CONTINUES: Atom<bool> = |_| true;
static SHOW_END_ONCE: Atom<bool> = |_| true;
static TILE_BONUS_TOGGLE: Atom<bool> = |_| false;

// Check if the conditions are met for ending the game.
// (i.e. final score is reached, all players have all the scores inputted, and there is no draw)
fn get_game_status(cx: Scope) -> GameStatus {
    let state = use_atom_ref(&cx, STATE);
    let mut game_status = GameStatus::Ongoing;

    // Pull the final scores and number of games played by each player.
    let (total_scores, games_played): (Vec<i32>, Vec<usize>) = state
        .read()
        .players
        .iter()
        .map(|player| (player.score.values().sum::<i32>(), player.score.len()))
        .unzip();

    let max = *(total_scores.iter().max().unwrap()); //the highest score achieved

    if max >= FINAL_SCORE {
        //Count how many players have the highest score to check for a draw.
        let no_of_winners = &state
            .read()
            .players
            .iter()
            .filter(|player| player.score.values().sum::<i32>() >= max)
            .count();

        if (games_played.iter().min().unwrap() == games_played.iter().max().unwrap())
            && *no_of_winners == 1
        {
            game_status = GameStatus::Finished;
        }
    }

    game_status
}

pub fn screen(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let game_continues = use_atom_state(&cx, GAME_CONTINUES);
    let show_end_once = use_atom_state(&cx, SHOW_END_ONCE);

    //Save game to storage.
    LocalStorage::set("state", state.read().clone()).unwrap();
    SessionStorage::set("session", true).unwrap();

    let game_status = get_game_status(cx);

    match game_status {
        GameStatus::Finished => {
            game_continues.set(false);
            if **show_end_once {
                state.write().screen = Screen::Winner;
                state.write().game_status = GameStatus::Finished;
                show_end_once.set(false);
            }
        }
        GameStatus::Ongoing | GameStatus::NotStarted => {
            game_continues.set(true);
        }
    };

    cx.render(rsx! {
        self::score_table()
    })
}

fn score_table(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let tile_bonus_toggle = use_atom_state(&cx, TILE_BONUS_TOGGLE);

    let (banner_text, border_color) = if **tile_bonus_toggle {
        (String::from("Who gets the bonus?"), String::from("border-cyan-300"))
    } else {
        (String::from("Good luck and have fun!"), String::from("border-emerald-300"))
    };

    cx.render(rsx! (
        div {
            class: "flex flex-col grow h-screen w-screen relative overflow-hidden px-[5%]",
            nav_bar(),
            div {
                class: "mb-4 w-max mx-auto",
                span {
                    class: "font-semibold text-lg border-b-2 {border_color}",
                    "{banner_text}",
                }
            }
            div{
                //Main table
                class: "z-10 flex justify-evenly gap-x-4 pt-2 overflow-auto mx-auto w-full sm:max-w-lg",

                state.read().players.iter().map(|player|
                    player_column(cx, player.clone())
                )
            }
            game_menu(),
            decorative_spheres(),
        }
    ))
}

fn game_menu(cx: Scope) -> Element {
    let toggle = use_atom_state(&cx, TILE_BONUS_TOGGLE);

    cx.render(rsx!(
        div {
            class: "z-20 absolute bottom-4 left-4 w-2/5",
            button {
                class: "flex flex-row gap-4 h-10",
                onclick: |_| toggle.set(toggle.not()),
                img {
                    class: "h-10 w-10",
                    src: "img/bonus.svg"
                }
                span {
                    class: "font-semibold text-lg self-center",
                    "Tile bonus"
                }
            }
        }
    ))
}

fn player_column(cx: Scope, player: Player) -> Element {
    let sum = player.score.values().sum::<i32>().to_string();
    let background = tailwind_classes::TITLE_COLORS[player.id - 1];
    let border = tailwind_classes::BORDER_COLORS[player.id - 1];

    cx.render(rsx!(
        div{
            class: "w-full",
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
            crate::screens::game::score_input {
                id: player.id
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
    ))
}

#[inline_props]
fn score_input(cx: Scope, id: usize) -> Element {
    let game_continues = use_atom_state(&cx, GAME_CONTINUES);
    let state = use_atom_ref(&cx, STATE);
    let buffer = use_state(&cx, String::new);
    let execute = use_eval(&cx);

    let onsubmit = move |_| {
        if let Ok(number) = buffer.parse::<i32>() {
            for player in &mut state.write().players {
                if *id == player.id {
                    player.score.insert(player.score.len(), number);
                }
            }
        }
        buffer.set(String::new());

        match id.cmp(&state.read().players.len()) {
            Ordering::Greater => (),
            Ordering::Less => {
                execute(
                    "document.getElementById('".to_string() + &(id + 1).to_string() + "').focus();",
                );
            }
            Ordering::Equal => {
                execute("document.getElementById('1').focus();".to_string());
            }
        }
    };

    let caret = tailwind_classes::CARET_COLORS[id - 1];
    let border = tailwind_classes::BORDER_COLORS[id - 1];

    if **game_continues {
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
                    oninput: move |evt: UiEvent<FormData>| {
                        buffer.set(evt.value.clone());
                    },
                    outline: "none",
                    r#type: "number",
                }
            }
        ))
    } else {
        None
    }
}

fn nav_bar(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let game_continues = use_atom_state(&cx, GAME_CONTINUES);

    let button_position = if **game_continues {
        "col-start-3 justify-self-end"
    } else {
        "col-start-1 justify-self-start"
    };

    cx.render(rsx!(
        div {
            class: "z-10 h-16 grid grid-cols-3 mx-auto w-full sm:max-w-lg",
            game_continues.then(|| rsx!(
                button {
                    class: "col-start-1 justify-self-start",
                    onclick: |_| {
                        state.write().screen = Screen::PlayerSelect;
                    },
                    img {
                        class: "h-8 w-8",
                        src: "img/back.svg",
                    }
                }
            )),
            button {
                class: "{button_position}",
                onclick: |_| {
                    state.write().screen = Screen::Menu;
                },
                img {
                    class: "h-8 w-8",
                    src: "img/home.svg",
                }
            }
            game_continues.not().then(|| rsx!(
                button {
                    class: "col-start-3 justify-self-end",
                    onclick: |_| {
                        state.write().screen = Screen::Winner;
                    },
                    img {
                        class: "h-8 w-8 scale-x-[-1]",
                        src: "img/back.svg",
                    }
                }
            ))
        }
    ))
}

fn decorative_spheres(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "z-0 absolute h-screen w-screen",
            div {
                class: "w-[500px] h-[500px] bottom-[-250px] right-[-250px] absolute rounded-full z-0",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
            }
        }
    ))
}
