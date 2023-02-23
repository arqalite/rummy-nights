use dioxus::events::FormEvent;
use dioxus::prelude::*;
use dioxus_web::use_eval;
use std::cmp::Ordering;
use std::ops::Not;

use crate::prelude::*;

pub fn screen(cx: Scope) -> Element {
    log!("Rendering game screen.");

    cx.render(rsx! (
        NavBar {},
        Banner {},
        PlayerTable {}
        GameMenu {},
    ))
}

fn PlayerTable(cx: Scope) -> Element {
    log!("Rendering player table.");

    let state = fermi::use_atom_ref(cx, STATE);
    let mut game_count = 0;

    let edit_score = move |evt: FormEvent| {
        log!(format!("This has {:?}", evt.values));
        if let Ok(score) = evt.values.get("score").unwrap().parse::<i32>() {
            if let Ok(score_id) = evt.values.get("score_id").unwrap().parse::<usize>() {
                if let Ok(player_id) = evt.values.get("player_id").unwrap().parse::<usize>() {
                    state.write().edit_score(player_id, score_id, score);
                }
            }
        };
    };

    cx.render(rsx!(
        div {
            //Main table
            class: "z-10 flex justify-evenly gap-x-4 h-[65%] px-8",
            state.read().game.players.iter().map(|player| {
                log!("Rendering player column.");
                let player_id = player.id;
                let border = BORDER_COLORS[player.color_index];
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
                            BG_COLORS[player.color_index],
                            "text-white",
                            "-1",
                        )
                    };
                let mut score_id = 0;

                rsx!(
                    div {
                        class: "flex flex-col gap-2 w-full",
                        button {
                            // Name - first cell
                            class: "relative rounded-full h-8 {player_background} {player_name_button_style} w-full",
                            tabindex: "{tabindex}",
                            onclick: move |_| {
                                if !state.read().game.tile_bonus_granted && state.read().settings.use_tile_bonus {
                                    state.write().game.grant_bonus(player_id);
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
                        (!player.score.is_empty()).then(|| rsx!(
                            div {
                                class: "flex flex-col gap-2 w-full overflow-auto scroll-smooth",
                                id: "score_{player_id}",
                                style: "scrollbar-width: none;",
                                //Scores - dynamic
                                player.score.values().map(|score| {
                                    let bonus_visibility = if player.bonus.contains_key(&game_count) {
                                        String::from("")
                                    } else {
                                        String::from("hidden")
                                    };

                                    game_count += 1;
                                    score_id += 1;

                                    rsx!(
                                        div {
                                            class: "flex flex-row justify-center relative rounded border-b-4 h-10 {border}",
                                            (state.read().settings.enable_score_editing).then(|| rsx!(
                                                form {
                                                    onsubmit: edit_score,
                                                    prevent_default: "onsubmit",
                                                    input {
                                                        name: "score",
                                                        onsubmit: edit_score,
                                                        class: "text-lg appearance-none leading-6 font-light bg-transparent h-10 w-full text-center",
                                                        style: "-moz-appearance:textfield",
                                                        value: "{score}",
                                                        outline: "none",
                                                        r#type: "number",
                                                    }
                                                    input {
                                                        name: "score_id",
                                                        r#type: "hidden",
                                                        value: "{score_id}",
                                                    }
                                                    input {
                                                        name: "player_id",
                                                        r#type: "hidden",
                                                        value: "{player_id}",
                                                    }
                                                }
                                            )),
                                            (!state.read().settings.enable_score_editing).then(|| rsx!(
                                                p {
                                                    class: "text-lg text-center self-center leading-6",
                                                    "{score}"
                                                }
                                            ))
                                            div {
                                                class: "absolute right-0 self-center h-4 {bonus_visibility} rounded-full",
                                                assets::bonus {}
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
    let state = fermi::use_atom_ref(cx, STATE);
    let mut color_index = 0;

    if state.read().game.status != GameStatus::Ongoing {
        return None;
    };

    for player in &state.read().game.players {
        if player.id == *id {
            color_index = player.color_index;
        }
    }

    let id = *id;
    let caret = CARET_COLORS[color_index];
    let border = BORDER_COLORS[color_index];

    let onsubmit = move |evt: FormEvent| {
        if let Ok(score) = evt.values.get("score").unwrap().parse::<i32>() {
            state.write().add_score(id, score);
        };

        let focus_id = match id.cmp(&state.read().game.players.len()) {
            Ordering::Greater => 5,
            Ordering::Equal => 1,
            Ordering::Less => id + 1,
        };
        use_eval(&cx)(format!("document.getElementById('{id}').value = '';"));
        use_eval(&cx)(format!("document.getElementById('{focus_id}').focus();"));
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

fn GameMenu(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);

    if !state.read().settings.use_tile_bonus {
        return None;
    };

    let tile_bonus_text = get_text(state.read().settings.language, "tile_bonus").unwrap();

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
            class: "z-20 absolute bottom-4 left-4 {hidden}",
            button {
                class: "flex flex-row gap-2 h-14 w-max p-2 border border-slate-100 rounded-full {grayscale}",
                onclick: tile_bonus,
                box_shadow: "{shadow}",
                div {
                    class: "h-10 w-10 self-center rounded-full",
                    assets::bonus {},
                }
                span {
                    class: "font-semibold text-lg self-center pr-2",
                    "{tile_bonus_text}"
                }
            }
        }
    ))
}

fn NavBar(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);

    let button_position = if state.read().game.status == GameStatus::Ongoing {
        "col-start-3 justify-self-end"
    } else {
        "col-start-1 justify-self-start"
    };

    log!("Render nav bar.");
    cx.render(rsx!(
        div {
            class: "z-10 h-16 grid grid-cols-3 sm:max-w-lg px-8",
            (state.read().game.status == GameStatus::Ongoing).then(|| rsx!(
                button {
                    class: "col-start-1 justify-self-start",
                    onclick: move |_| state.write().screen = Screen::PlayerSelect,
                    div {
                        class: "h-10 scale-x-[-1]",
                        assets::BackIcon {}
                    }
                }
            )),
            button {
                class: "{button_position}",
                onclick: move |_| state.write().screen = Screen::Menu,
                div {
                    class: "h-10",
                    assets::home {},
                }
            }
            (state.read().game.status == GameStatus::Ongoing).not().then(|| rsx!(
                button {
                    class: "col-start-3 justify-self-end",
                    onclick: move |_| state.write().screen = Screen::EndGame,
                    div {
                        class: "h-10",
                        assets::BackIcon {}
                    }
                }
            ))
        }
    ))
}

fn Banner(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);

    let banner_win = get_text(state.read().settings.language, "banner_win").unwrap();
    let banner_bonus = get_text(state.read().settings.language, "banner_bonus").unwrap();
    let banner_play = get_text(state.read().settings.language, "banner_play").unwrap();

    let (banner_text, banner_color) = match &state.read().game.status {
        GameStatus::Finished => (
            format!("{} {}!", state.read().game.get_winner(), banner_win),
            String::from("border-red-600"),
        ),
        _ => {
            if state.read().game.tile_bonus_toggle {
                (format!("{banner_bonus}?"), String::from("border-cyan-500"))
            } else {
                (format!("{banner_play}!"), String::from("border-green-500"))
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
    let state = fermi::use_atom_ref(cx, STATE);

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
    cx.render(rsx!(div {
        class: "h-7 absolute -top-4 -right-4 scale-x-[-1]",
        assets::pushpin {}
    },))
}
