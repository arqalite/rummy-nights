use dioxus::fermi::{use_atom_ref, use_atom_state};
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, SessionStorage, Storage};

use crate::data::tailwind_classes;
use crate::prelude::*;

static HAS_SORTED_ONCE: Atom<bool> = |_| false;
static CLONED_PLAYERS: AtomRef<Vec<Player>> = |_| Vec::new();

pub fn screen(cx: Scope) -> Element {
    log!("Rendering end screen.");

    let state = use_atom_ref(&cx, STATE);
    let cloned_players = use_atom_ref(&cx, CLONED_PLAYERS);
    let is_sorted = use_atom_state(&cx, HAS_SORTED_ONCE);

    if !is_sorted {
        log!("Sorting players.");

        *cloned_players.write() = state.read().players.clone();

        cloned_players.write().sort_by(|a, b| {
            let temp_sum_a = a.sum;
            let temp_sum_b = b.sum;

            temp_sum_a.cmp(&temp_sum_b)
        });

        cloned_players.write().reverse();

        is_sorted.set(true);
    };

    let mut player_count = 0;

    cx.render(rsx!(
        div {
            class: "z-10 flex flex-col grow mx-auto w-full sm:max-w-lg",
            nav_bar(),
            div {
                class: "mt-8",
                img {
                    src: "img/trophy.svg",
                    class: "h-20 w-20 mx-auto"
                }
                p {
                    class: "text-center font-bold text-4xl mt-2",
                    "THE WINNER IS"
                }
            },
            div {
                class: "flex flex-col basis-1/2 grow-0 shrink justify-evenly content-evenly",
                cloned_players.read().iter().map(|player| {
                    log!("Rendering players.");

                    let background = tailwind_classes::BG_COLORS[player.id-1];
                    let border = tailwind_classes::BORDER_COLORS[player.id-1];
                    let score = player.score.values().sum::<i32>() + player.bonus.values().sum::<i32>();
                    let mut style;
                    let style2;

                    if player_count == 0 {
                        style = "h-20 w-20 rounded-full text-white font-bold text-lg ".to_string();
                        style.push_str(background);
                        style2 = "relative top-[50%] -translate-y-1/2".to_string();
                    } else {
                        style = "border-b-[7px] rounded-md my-auto w-20 ".to_string();
                        style.push_str(border);
                        style2 = String::new();
                    };

                    player_count += 1;

                    rsx! (
                    div {
                        class: "flex flex-row justify-evenly items-center",
                        div {
                            class: "{style}",
                            p {
                                class: "text-center mb-2 {style2}",
                                "{score}"
                            }
                        }
                        div {
                            class: "h-12 basis-1/3 {background} rounded-full",
                            p {
                                class: "text-center relative top-[50%] -translate-y-1/2 text-white font-semibold",
                                "{player.name}"
                            }
                        }
                    }
                    )
                })
            }
        }
    ))
}

fn nav_bar(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let cloned_players = use_atom_ref(&cx, CLONED_PLAYERS);
    let is_sorted = use_atom_state(&cx, HAS_SORTED_ONCE);

    let delete_and_exit_game = |_| {
        LocalStorage::clear();
        SessionStorage::clear();
        *state.write() = Model::new();
    };

    let restart_game = |_| {
        cloned_players.write().clear();
        is_sorted.set(false);

        state.write().reset_game();
    };

    cx.render(rsx!(
        div {
            class: "h-16 grid grid-cols-3",
            button {
                class: "col-start-1 justify-self-start",
                onclick: |_| {
                    state.write().screen = Screen::Game;
                },
                img {
                    class: "h-8 w-8",
                    src: "img/back.svg",
                }
            }
            button {
                class: "col-start-2 justify-self-center",
                onclick: delete_and_exit_game,
                img {
                    class: "h-8 w-8",
                    src: "img/home.svg",
                }
            }
            button {
                class: "col-start-3 justify-self-end",
                onclick: restart_game,
                img {
                    class: "h-8 w-8",
                    src: "img/replay.svg",
                }
            }
        }
    ))
}