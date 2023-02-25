use crate::prelude::*;
use dioxus::prelude::*;

#[inline_props]
pub fn GameScreen<'a>(
    cx: Scope,
    lang_code: usize,
    game: Game,
    use_tile_bonus: bool,
    tile_bonus_button_active: bool,
    tile_bonus_granted: bool,
    enable_dealer_tracking: bool,
    enable_score_editing: bool,
    on_click_playerselect: EventHandler<'a, MouseEvent>,
    on_click_home: EventHandler<'a, MouseEvent>,
    on_click_end: EventHandler<'a, MouseEvent>,
    on_score_edit: EventHandler<'a, FormEvent>,
    on_score_input: EventHandler<'a, (FormEvent, usize, usize)>,
    on_click_player: EventHandler<'a, (MouseEvent, usize)>,
    on_click_tile_bonus: EventHandler<'a, MouseEvent>,
) -> Element {
    let banner_type = match game.status {
        GameStatus::Finished => BannerType::Win,
        _ => {
            if *tile_bonus_button_active {
                BannerType::Bonus
            } else {
                BannerType::Play
            }
        }
    };

    let mut dealer_pin_position = 0;
    for player in game.players.iter() {
        if *enable_dealer_tracking
            && (((game.round + game.players.len() + 1) - player.id + game.total_rounds)
                % game.players.len()
                == 0)
            && game.status == GameStatus::Ongoing
        {
            dealer_pin_position = player.id;
        }
    }

    log!("Rendering game screen.");

    render!(
        NavBar {
            game_status: game.status,
            on_click_back: |evt| on_click_playerselect.call(evt)
            on_click_home: |evt| on_click_home.call(evt)
            on_click_back_to_end: |evt| on_click_end.call(evt)
        },
        Banner {
            lang_code: *lang_code,
            banner_type: banner_type,
            winner: game.get_winner()
        },
        PlayerTable {
            players: game.players.clone(),
            game_status: game.status,
            tile_bonus_button_active: *tile_bonus_button_active,
            enable_score_editing: *enable_score_editing,
            dealer_pin_position: dealer_pin_position,
            on_score_edit: |evt| on_score_edit.call(evt),
            on_score_input: |evt| on_score_input.call(evt),
            on_click_player: |evt| on_click_player.call(evt)
        },
        use_tile_bonus.then(|| rsx!(TileBonusButton {
            lang_code: *lang_code,
            game_status: game.status,
            tile_bonus_granted: *tile_bonus_granted,
            button_pressed: *tile_bonus_button_active,
            on_click: |evt| on_click_tile_bonus.call(evt),
        })),
    )
}

#[inline_props]
fn PlayerTable<'a>(
    cx: Scope,
    players: Vec<Player>,
    game_status: GameStatus,
    tile_bonus_button_active: bool,
    enable_score_editing: bool,
    dealer_pin_position: usize,
    on_score_edit: EventHandler<'a, FormEvent>,
    on_score_input: EventHandler<'a, (FormEvent, usize, usize)>,
    on_click_player: EventHandler<'a, (MouseEvent, usize)>,
) -> Element {
    log!("Rendering player table.");
    let player_len = players.len();

    render!(
        div {
            //Main table
            class: "z-10 flex justify-evenly gap-x-4 h-[65%] px-8",
            players.iter().map(|player| {
                let player_id = player.id;
                rsx!(
                    div {
                        class: "flex flex-col gap-2 w-full",
                        NameButton {
                            name: player.name.clone(),
                            player_id: player_id,
                            color_index: player.color_index,
                            tile_bonus_button_active: *tile_bonus_button_active,
                            dealer_pin_position: *dealer_pin_position,
                            on_click_player: move |evt| on_click_player.call((evt, player_id)),
                        }
                        (!player.score.is_empty()).then(|| rsx!(
                            ScoreTable {
                                player: player.clone(),
                                enable_score_editing: *enable_score_editing,
                                on_submit: |evt| on_score_edit.call(evt),
                            }
                        ))
                        div {
                            class: "flex flex-col gap-2 w-full",
                            (*game_status == GameStatus::Ongoing).then(|| rsx!(
                                ScoreInput {
                                    id: player_id,
                                    on_submit: move |evt| on_score_input.call((evt, player_id, player_len)),
                                    color_index: player.color_index
                                },
                            ))
                            ScoreTotal {
                                color_index: player.color_index,
                                sum: player.sum
                            }
                        }

                    }
                )
            })
        }
    )
}

#[inline_props]
fn NameButton<'a>(
    cx: Scope,
    name: String,
    player_id: usize,
    color_index: usize,
    tile_bonus_button_active: bool,
    dealer_pin_position: usize,
    on_click_player: EventHandler<'a, MouseEvent>,
) -> Element {
    let (player_name_button_style, player_background, player_text_color, tabindex) =
        if *tile_bonus_button_active {
            (
                "pointer-events-auto",
                "bg-white outline outline-1 outline-black",
                "text-black",
                "0",
            )
        } else {
            (
                "pointer-events-none",
                BG_COLORS[*color_index],
                "text-white",
                "-1",
            )
        };

    render!(
        button {
            // Name - first cell
            class: "relative rounded-full h-8 {player_background} {player_name_button_style} w-full",
            tabindex: "{tabindex}",
            onclick: |evt| on_click_player.call(evt),
            (dealer_pin_position == player_id).then(|| rsx!(
                DealerPin {}
            ))
            p {
                class: "text-center my-auto {player_text_color} font-semibold",
                "{name}"
            }
        }
    )
}

#[inline_props]
fn ScoreTable<'a>(
    cx: Scope,
    player: Player,
    enable_score_editing: bool,
    on_submit: EventHandler<'a, FormEvent>,
) -> Element {
    let mut game_count = 0;
    let mut score_id = 0;

    let player_id = player.id;

    render!(
        div {
            class: "flex flex-col gap-2 w-full overflow-auto scroll-smooth",
            id: "score_{player_id}",
            style: "scrollbar-width: none;",
            player.score.values().map(|score| {
                game_count += 1;
                score_id += 1;

                rsx!(
                    ScoreItem {
                        id: score_id,
                        player_id: player_id,
                        score: *score,
                        color_index: player.color_index,
                        has_bonus: player.bonus.contains_key(&game_count),
                        enable_score_editing: *enable_score_editing,
                        on_submit: |evt| on_submit.call(evt),
                    }
                )
            })
        }
    )
}

#[inline_props]
fn ScoreItem<'a>(
    cx: Scope,
    id: i32,
    player_id: usize,
    score: i32,
    color_index: usize,
    has_bonus: bool,
    enable_score_editing: bool,
    on_submit: EventHandler<'a, FormEvent>,
) -> Element {
    let border = BORDER_COLORS[*color_index];

    let bonus_visibility = if *has_bonus { "" } else { "hidden" };

    render!(
        div {
            class: "flex flex-row justify-center relative rounded border-b-4 h-10 {border}",
            (enable_score_editing).then(|| rsx!(
                form {
                    onsubmit: |evt| on_submit.call(evt),
                    prevent_default: "onsubmit",
                    input {
                        name: "score",
                        onsubmit: |evt| on_submit.call(evt),
                        class: "text-lg appearance-none leading-6 font-light bg-transparent h-10 w-full text-center",
                        style: "-moz-appearance:textfield",
                        value: "{score}",
                        outline: "none",
                        r#type: "number",
                    }
                    input {
                        name: "score_id",
                        r#type: "hidden",
                        value: "{id}",
                    }
                    input {
                        name: "player_id",
                        r#type: "hidden",
                        value: "{player_id}",
                    }
                }
            )),
            (!enable_score_editing).then(|| rsx!(
                p {
                    class: "text-lg text-center self-center leading-6",
                    "{score}"
                }
            ))
            div {
                class: "absolute right-0 self-center h-4 {bonus_visibility} rounded-full",
                assets::BonusIcon {}
            }
        }
    )
}

#[inline_props]
fn ScoreTotal(cx: Scope, color_index: usize, sum: i32) -> Element {
    let border = BORDER_COLORS[*color_index];

    render!(
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

#[inline_props]
fn ScoreInput<'a>(
    cx: Scope,
    id: usize,
    color_index: usize,
    on_submit: EventHandler<'a, FormEvent>,
) -> Element {
    let caret = CARET_COLORS[*color_index];
    let border = BORDER_COLORS[*color_index];

    log!("Rendering score input.");
    render!(
        form {
            onsubmit: |evt| on_submit.call(evt),
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
    )
}

#[inline_props]
fn TileBonusButton<'a>(
    cx: Scope,
    lang_code: usize,
    game_status: GameStatus,
    tile_bonus_granted: bool,
    button_pressed: bool,
    on_click: EventHandler<'a, MouseEvent>,
) -> Element {
    log!("Rendering tile bonus menu.");

    let hidden = if *game_status == GameStatus::Ongoing {
        ""
    } else {
        "hidden"
    };

    let grayscale = if *tile_bonus_granted { "grayscale" } else { "" };

    let shadow = if *button_pressed {
        "inset 0 2px 4px 0 rgb(0 0 0 / 0.25)"
    } else {
        "0 1px 3px 0 rgb(0 0 0 / 0.25), 0 1px 2px -1px rgb(0 0 0 / 0.25)"
    };

    render!(
        div {
            class: "z-20 absolute bottom-4 left-4 {hidden}",
            button {
                class: "flex flex-row gap-2 h-14 w-max p-2 border border-slate-100 rounded-full {grayscale}",
                onclick: |evt| on_click.call(evt),
                box_shadow: "{shadow}",
                div {
                    class: "h-10 w-10 self-center rounded-full",
                    assets::BonusIcon {},
                }
                span {
                    class: "font-semibold text-lg self-center pr-2",
                    get_text(*lang_code, "tile_bonus")
                }
            }
        }
    )
}

#[inline_props]
fn NavBar<'a>(
    cx: Scope,
    game_status: GameStatus,
    on_click_back: EventHandler<'a, MouseEvent>,
    on_click_home: EventHandler<'a, MouseEvent>,
    on_click_back_to_end: EventHandler<'a, MouseEvent>,
) -> Element {
    let button_position = if *game_status == GameStatus::Ongoing {
        "col-start-3 justify-self-end"
    } else {
        "col-start-1 justify-self-start"
    };

    log!("Render nav bar.");
    render!(
        div {
            class: "z-10 h-16 grid grid-cols-3 sm:max-w-lg px-8",
            (*game_status == GameStatus::Ongoing).then(|| rsx!(
                button {
                    class: "col-start-1 justify-self-start",
                    onclick: |evt| on_click_back.call(evt),
                    div {
                        class: "h-10 scale-x-[-1]",
                        assets::BackIcon {}
                    }
                }
            )),
            button {
                class: "{button_position}",
                onclick: |evt| on_click_home.call(evt),
                div {
                    class: "h-10",
                    assets::HomeIcon {},
                }
            }
            (*game_status != GameStatus::Ongoing).then(|| rsx!(
                button {
                    class: "col-start-3 justify-self-end",
                    onclick: |evt| on_click_back_to_end.call(evt),
                    div {
                        class: "h-10",
                        assets::BackIcon {}
                    }
                }
            ))
        }
    )
}

#[derive(PartialEq)]
enum BannerType {
    Play,
    Bonus,
    Win,
}

#[inline_props]
fn Banner(cx: Scope, lang_code: usize, banner_type: BannerType, winner: String) -> Element {
    let (banner_text, banner_color) = match banner_type {
        BannerType::Play => (
            get_text(*lang_code, "banner_play").to_string(),
            String::from("border-green-500"),
        ),
        BannerType::Bonus => (
            get_text(*lang_code, "banner_bonus").to_string(),
            String::from("border-cyan-500"),
        ),
        BannerType::Win => (
            (format!("{} {}", winner, get_text(*lang_code, "banner_win"))),
            String::from("border-red-600"),
        ),
    };

    log!("Render banner.");
    render!(
        span {
            class: "mb-8 w-max mx-auto font-semibold text-lg border-b-2 {banner_color}",
            "{banner_text}",
        }
    )
}

fn DealerPin(cx: Scope) -> Element {
    log!("Render dealer pin.");
    render!(
        div {
            class: "h-7 absolute -top-4 -right-4 scale-x-[-1]",
            assets::DealerIcon {}
        }
    )
}
