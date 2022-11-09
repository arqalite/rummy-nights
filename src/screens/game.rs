use dioxus::events::FormEvent;
use dioxus::fermi::{use_atom_ref, use_atom_state};
use dioxus::prelude::*;
use dioxus::web::use_eval;
use std::cmp::Ordering;
use std::ops::Not;

use crate::data::tailwind_classes;
use crate::prelude::*;

static TILE_BONUS_TOGGLE: Atom<bool> = |_| false;

pub fn screen(cx: Scope) -> Element {
    log!("Rendering game screen.");

    cx.render(rsx! {
        self::score_table()
    })
}

fn score_table(cx: Scope) -> Element {
    log!("Rendering score table.");

    let state = use_atom_ref(&cx, STATE);
    let tile_bonus_toggle = use_atom_state(&cx, TILE_BONUS_TOGGLE);

    let (banner_text, border_color) = match &state.read().game_status {
        GameStatus::Finished(winner) => {
            (format!("{} won!", winner), String::from("border-red-600"))
        }
        _ => {
            if **tile_bonus_toggle {
                (
                    String::from("Who gets the bonus?"),
                    String::from("border-cyan-500"),
                )
            } else {
                (
                    String::from("Good luck and have fun!"),
                    String::from("border-green-500"),
                )
            }
        }
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
                class: "z-10 flex justify-evenly gap-x-4 pt-2 overflow-visible mx-auto w-full sm:max-w-lg",

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
    let state = use_atom_ref(&cx, STATE);
    let tile_bonus_toggle = use_atom_state(&cx, TILE_BONUS_TOGGLE);

    log!("Rendering tile bonus menu.");

    let hidden = if state.read().game_status == GameStatus::Ongoing {
        ""
    } else {
        "hidden"
    };

    let grayscale = if !state.read().tile_bonus_granted {
        ""
    } else {
        "grayscale"
    };

    let shadow = if **tile_bonus_toggle {
        "inset 0 2px 4px 0 rgb(0 0 0 / 0.25)"
    } else {
        "0 1px 3px 0 rgb(0 0 0 / 0.25), 0 1px 2px -1px rgb(0 0 0 / 0.25)"
    };

    let tile_bonus = move |_| {
        if **tile_bonus_toggle {
            tile_bonus_toggle.set(false)
        } else if !state.read().tile_bonus_granted
            && state.read().game_status == GameStatus::Ongoing
        {
            tile_bonus_toggle.set(true)
        };
    };

    cx.render(rsx!(
        div {
            class: "z-20 absolute bottom-2 left-2 {hidden}",
            button {
                class: "flex flex-row gap-2 h-14 w-max p-2 border border-slate-100 rounded-full {grayscale}",
                onclick: tile_bonus,
                box_shadow: "{shadow}",
                img {
                    class: "h-10 w-10 self-center",
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
    log!("Rendering player column.");

    let mut game_count = 0;
    let state = use_atom_ref(&cx, STATE);
    let border = tailwind_classes::BORDER_COLORS[player.id - 1];

    let tile_bonus_toggle = use_atom_state(&cx, TILE_BONUS_TOGGLE);

    let (player_name_button_style, player_background, player_text_color, tabindex) =
        if **tile_bonus_toggle {
            (
                "pointer-events-auto",
                "bg-white border border-black",
                "text-black",
                "0",
            )
        } else {
            (
                "pointer-events-none",
                tailwind_classes::BG_COLORS[player.id - 1],
                "text-white",
                "-1",
            )
        };

    cx.render(rsx!(
        div{
            class: "w-full",
            //Column for each player
            button {
                // Name - first cell
                class: "relative rounded-full h-8 {player_background} py-1 {player_name_button_style} w-full",
                tabindex: "{tabindex}",
                onclick: move |_| {
                    if !state.read().tile_bonus_granted {
                        state.write().grant_bonus(player.id);
                        state.write().new_round_started = false;
                        tile_bonus_toggle.set(false);
                        state.write().save_game();
                    }
                },
                ((((state.read().round + state.read().players.len() + 1) - player.id) % state.read().players.len() == 0) && state.read().game_status == GameStatus::Ongoing).then(|| rsx!(
                    img {
                        class: "h-6 w-6 absolute -top-4 -right-2",
                        src: "img/pushpin.svg"
                    }
                )),
                p {
                    class: "text-center my-auto {player_text_color} font-semibold",
                    "{player.name}"
                }
            }
            div {
                //Scores - dynamic
                player.score.values().map(|score| {
                    let score_text = score.to_string();

                    let bonus_visibility = if player.bonus.contains_key(&game_count) {
                        String::from("")
                    } else {
                        String::from("hidden")
                    };

                    game_count += 1;

                    rsx!(
                        div {
                            class: "relative rounded border-b-4 h-9 mt-2 {border}",
                            p {
                                class: "text-lg text-center",
                                "{score_text}"
                            }
                            img {
                                class: "absolute h-4 w-4 top-1/2 right-0 -translate-y-1/2 {bonus_visibility}",
                                src: "img/bonus.svg",

                            }
                        }
                    )
                })
            }
            self::score_input {
                id: player.id
            },
            div {
                //Total box
                class: "rounded border-b-[7px] {border} h-9 mt-2",
                p {
                    class: "text-center text-lg font-semibold",
                    "{player.sum}"
                }
            }
        }
    ))
}

#[inline_props]
fn score_input(cx: Scope, id: usize) -> Element {
    log!("Rendering score input.");

    let id = *id;
    let state = use_atom_ref(&cx, STATE);
    let execute = use_eval(&cx);

    let onsubmit = move |evt: FormEvent| {
        let score = evt.values.get("score").unwrap();

        if let Ok(number) = score.parse::<i32>() {
            for player in &mut state.write().players {
                if id == player.id {
                    player.score.insert(player.score.len(), number);
                    player.sum =
                        player.score.values().sum::<i32>() + player.bonus.values().sum::<i32>();
                }
            }
        }

        execute(format!("document.getElementById('{}').value = '';", id));
        state.write().check_round();
        state.write().check_game_status();
        state.write().save_game();

        match id.cmp(&state.read().players.len()) {
            Ordering::Greater => (),
            Ordering::Less => {
                execute(format!("document.getElementById('{}').focus();", id + 1));
            }
            Ordering::Equal => {
                execute("document.getElementById('1').focus();".to_string());
            }
        }
    };

    let caret = tailwind_classes::CARET_COLORS[id - 1];
    let border = tailwind_classes::BORDER_COLORS[id - 1];

    if state.read().game_status == GameStatus::Ongoing {
        cx.render(rsx!(
            form {
                onsubmit: onsubmit,
                prevent_default: "onsubmit",
                input {
                    name: "score",
                    class: "{caret} {border} text-lg appearance-none font-light bg-transparent h-9 mt-2 w-full text-center rounded focus:border-b-[8px] border-b-4",
                    id: "{id}",
                    style: "-moz-appearance:textfield",
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

    let button_position = if state.read().game_status == GameStatus::Ongoing {
        "col-start-3 justify-self-end"
    } else {
        "col-start-1 justify-self-start"
    };

    cx.render(rsx!(
        div {
            class: "z-10 h-16 grid grid-cols-3 mx-auto w-full sm:max-w-lg",
            (state.read().game_status == GameStatus::Ongoing).then(|| rsx!(
                button {
                    class: "col-start-1 justify-self-start",
                    onclick: |_| state.write().screen = Screen::PlayerSelect,
                    img {
                        class: "h-8 w-8",
                        src: "img/back.svg",
                    }
                }
            )),
            button {
                class: "{button_position}",
                onclick: |_| state.write().screen = Screen::Menu,
                img {
                    class: "h-8 w-8",
                    src: "img/home.svg",
                }
            }
            (state.read().game_status == GameStatus::Ongoing).not().then(|| rsx!(
                button {
                    class: "col-start-3 justify-self-end",
                    onclick: |_| state.write().screen = Screen::Winner,
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
                class: "w-[100vw] h-[100vw] bottom-[-50vw] right-[-50vw] absolute rounded-full z-0",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
            }
        }
    ))
}
