use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, SessionStorage, Storage};

use crate::data::tailwind_classes;
use crate::prelude::*;

pub fn screen(cx: Scope) -> Element {
    log!("Rendering end screen.");

    let state = use_atom_ref(&cx, STATE);

    if !state.read().is_sorted {
        let mut state_writer = state.write();

        log!("Sorting players.");

        state_writer.sorted_players = state_writer.players.clone();
        log!("Getting players worked.");

        state_writer.sorted_players.sort_by(|a, b| {
            let temp_sum_a = a.sum;
            let temp_sum_b = b.sum;

            temp_sum_a.cmp(&temp_sum_b)
        });
        log!("Sorting players worked.");

        state_writer.sorted_players.reverse();
        log!("Reversing players worked.");

        state_writer.is_sorted = true;
        log!("Finishing players worked.");
    };

    let mut player_count = 0;

    cx.render(rsx!(
        nav_bar(),
        div {
            class: "flex flex-col absolute w-screen px-8 sm:max-w-lg top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 gap-6 justify-evenly",
            img {
                src: "img/trophy.svg",
                class: "h-24 w-24 mx-auto rounded-full",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
            }
            p {
                class: "text-center font-bold text-4xl",
                "THE WINNER IS"
            }
            state.read().sorted_players.iter().map(|player| {
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
                    class: "flex flex-row basis-1/2 justify-evenly items-center",
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
    ))
}

fn nav_bar(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    let delete_and_exit_game = |_| {
        LocalStorage::delete("state");
        SessionStorage::delete("session");
        *state.write() = Model::new();
    };

    let restart_game = |_| {
        state.write().sorted_players.clear();
        state.write().is_sorted = false;
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
                    class: "h-10 scale-x-[-1]",
                    src: "img/back.svg",
                }
            }
            button {
                class: "col-start-2 justify-self-center",
                onclick: delete_and_exit_game,
                img {
                    class: "h-10",
                    src: "img/home.svg",
                }
            }
            button {
                class: "col-start-3 justify-self-end",
                onclick: restart_game,
                img {
                    class: "h-10",
                    src: "img/replay.svg",
                }
            }
        }
    ))
}
