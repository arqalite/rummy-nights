use dioxus::events::FormEvent;
use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;
use dioxus::web::use_eval;
use std::cmp::Ordering;
use std::ops::Not;

use crate::prelude::*;

pub fn screen(cx: Scope) -> Element {
    log!("Rendering game screen.");

    cx.render(rsx! (
        nav_bar(),
        banner()
        player_table()
        game_menu(),
    ))
}

fn player_table(cx: Scope) -> Element {
    log!("Rendering player table.");

    let mut game_count = 0;
    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        div {
            //Main table
            class: "z-10 flex justify-evenly gap-x-4 h-[65%]",
            state.read().game.players.iter().map(|player| {
                log!("Rendering player column.");
                let player_id = player.id;
                let border = BORDER_COLORS[player_id - 1];
                let (player_name_button_style, player_background, player_text_color, tabindex) =
                    if state.read().game.tile_bonus_toggle {
                        (
                            "pointer-events-auto",
                            "bg-white outline outline-1 outline-black",
                            "text-black",
                            "0",
                        )
                    } else {
                        (
                            "pointer-events-none",
                            BG_COLORS[player_id - 1],
                            "text-white",
                            "-1",
                        )
                    };
                rsx!(
                    div {
                        class: "flex flex-col gap-2 w-full",
                        button {
                            // Name - first cell
                            class: "relative rounded-full h-8 {player_background} {player_name_button_style} w-full",
                            tabindex: "{tabindex}",
                            onclick: move |_| {
                                if !state.read().game.tile_bonus_granted && state.read().settings.use_tile_bonus {
                                    state.write().grant_bonus(player_id);
                                }
                            },
                            self::dealer_pin {
                                player_id: player_id
                            },
                            p {
                                class: "text-center my-auto {player_text_color} font-semibold",
                                "{player.name}"
                            }
                        }
                        (player.score.len() > 0).then(|| rsx!(
                            div {
                                class: "flex flex-col gap-2 w-full overflow-auto scroll-smooth",
                                id: "score_{player_id}",
                                style: "scrollbar-width: none;",
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
                                            class: "flex flex-row justify-center relative rounded border-b-4 h-10 {border}",
                                            p {
                                                class: "text-lg text-center self-center",
                                                "{score_text}"
                                            }
                                            img {
                                                class: "absolute right-0 self-center h-4 w-4 {bonus_visibility} rounded-full",
                                                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                                                src: "img/bonus.svg",
                                            }

                                        }
                                    )
                                })
                            }
                        ))
                        div {
                            class: "flex flex-col gap-2 w-full",
                            self::score_input {
                                id: player_id
                            },
                            div {
                                //Total box
                                class: "rounded border-b-[7px] {border} h-10",
                                p {
                                    class: "text-center text-lg font-semibold",
                                    "{player.sum}"
                                }
                            }
                        }

                    }
                )
            })
        }
    ))
}

#[inline_props]
fn score_input(cx: Scope, id: usize) -> Element {
    let state = use_atom_ref(&cx, STATE);

    if state.read().game.status != GameStatus::Ongoing {
        return None;
    };

    let id = *id;
    let caret = CARET_COLORS[id - 1];
    let border = BORDER_COLORS[id - 1];

    let onsubmit = move |evt: FormEvent| {
        if let Ok(score) = evt.values.get("score").unwrap().parse::<i32>() {
            state.write().add_score(id, score);
        };

        let focus_id = match id.cmp(&state.read().game.players.len()) {
            Ordering::Greater => 5,
            Ordering::Equal => 1,
            Ordering::Less => id + 1,
        };
        use_eval(&cx)(format!("document.getElementById('{}').value = '';", id));
        use_eval(&cx)(format!("document.getElementById('{}').focus();", focus_id));
    };

    log!("Rendering score input.");
    cx.render(rsx!(
        form {
            onsubmit: onsubmit,
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
    ))
}

fn game_menu(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    if !state.read().settings.use_tile_bonus {
        return None;
    };

    log!("Rendering tile bonus menu.");

    let hidden = if state.read().game.status == GameStatus::Ongoing {
        ""
    } else {
        "hidden"
    };

    let grayscale = if !state.read().game.tile_bonus_granted {
        ""
    } else {
        "grayscale"
    };

    let shadow = if state.read().game.tile_bonus_toggle {
        "inset 0 2px 4px 0 rgb(0 0 0 / 0.25)"
    } else {
        "0 1px 3px 0 rgb(0 0 0 / 0.25), 0 1px 2px -1px rgb(0 0 0 / 0.25)"
    };

    let tile_bonus = move |_| {
        if state.read().game.tile_bonus_toggle {
            state.write().game.tile_bonus_toggle = false;
        } else if !state.read().game.tile_bonus_granted
            && state.read().game.status == GameStatus::Ongoing
        {
            state.write().game.tile_bonus_toggle = true;
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
                    class: "h-10 w-10 self-center rounded-full",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
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

fn nav_bar(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    let button_position = if state.read().game.status == GameStatus::Ongoing {
        "col-start-3 justify-self-end"
    } else {
        "col-start-1 justify-self-start"
    };

    log!("Render nav bar.");
    cx.render(rsx!(
        div {
            class: "z-10 h-16 grid grid-cols-3 sm:max-w-lg",
            (state.read().game.status == GameStatus::Ongoing).then(|| rsx!(
                button {
                    class: "col-start-1 justify-self-start",
                    onclick: |_| state.write().screen = Screen::PlayerSelect,
                    img {
                        class: "h-10 scale-x-[-1]",
                        src: "img/back.svg",
                    }
                }
            )),
            button {
                class: "{button_position}",
                onclick: |_| state.write().screen = Screen::Menu,
                img {
                    class: "h-10",
                    src: "img/home.svg",
                }
            }
            (state.read().game.status == GameStatus::Ongoing).not().then(|| rsx!(
                button {
                    class: "col-start-3 justify-self-end",
                    onclick: |_| state.write().screen = Screen::EndGame,
                    img {
                        class: "h-10",
                        src: "img/back.svg",
                    }
                }
            ))
        }
    ))
}

fn banner(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    let (banner_text, banner_color) = match &state.read().game.status {
        GameStatus::Finished => (
            format!("{} won!", state.read().get_winner()),
            String::from("border-red-600"),
        ),
        _ => {
            if state.read().game.tile_bonus_toggle {
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

    log!("Render banner.");
    cx.render(rsx!(
        span {
            class: "mb-8 w-max mx-auto font-semibold text-lg border-b-2 {banner_color}",
            "{banner_text}",
        }
    ))
}

#[inline_props]
fn dealer_pin(cx: Scope, player_id: usize) -> Element {
    let state = use_atom_ref(&cx, STATE);

    if !state.read().settings.enable_dealer_tracking {
        return None;
    }

    if !((((state.read().game.round + state.read().game.players.len() + 1) - player_id
        + state.read().game.total_rounds)
        % state.read().game.players.len()
        == 0)
        && state.read().game.status == GameStatus::Ongoing)
    {
        return None;
    }

    log!("Render dealer pin.");
    cx.render(rsx!(img {
        class: "h-7 absolute -top-4 -right-4 scale-x-[-1]",
        src: "img/pushpin.svg"
    },))
}
