use dioxus::fermi::use_atom_state;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage, SessionStorage};

use crate::data::{GameStatus, Model, Screen, BORDER_COLORS, TITLE_COLORS};
use crate::STATE;

static HAS_SORTED_ONCE: Atom<bool> = |_| false;

pub fn winner_screen(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);
    let is_sorted = use_atom_state(&cx, HAS_SORTED_ONCE);

    let mut player_count = 0;

    let return_to_table = |_| {
        state.with_mut(|state| {
            state.screen = Screen::Game;
        });
    };

    let delete_and_exit_game = |_| {
        LocalStorage::clear();
        SessionStorage::clear();
        state.set(Model {
            players: Vec::new(),
            game_status: GameStatus::NotStarted,
            screen: Screen::Menu,
        });
    };

    if !is_sorted {
        state.with_mut(|mut_state| {
            mut_state.players.sort_by(|a, b| {
                let temp_sum_a = a.score.values().sum::<i32>();
                let temp_sum_b = b.score.values().sum::<i32>();

                temp_sum_a.cmp(&temp_sum_b)
            });

            mut_state.players.reverse();
        });

        is_sorted.set(true);
    };

    LocalStorage::clear();
    SessionStorage::clear();

    cx.render(rsx!(
        div {
            class: "flex flex-col grow h-screen w-screen relative overflow-hidden px-[5%]",
            div {
                class: "z-0 absolute h-screen w-screen",
                div {
                    class: "w-[300px] h-[300px] top-[-150px] left-[-150px] absolute rounded-full",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                },
                div {
                    class: "w-[300px] h-[300px] bottom-[-150px] right-[-150px] absolute rounded-full",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                },
            },
            div {
                class: "z-10 flex flex-col grow mx-auto w-full sm:max-w-lg",
                div {
                    class: "h-16 grid grid-cols-3",
                    button {
                        class: "col-start-1 justify-self-start",
                        onclick: return_to_table,
                        img {
                            class: "h-8 w-8",
                            src: "img/back.svg",
                        }
                    }
                    button {
                        class: "col-start-3 justify-self-end",
                        onclick: delete_and_exit_game,
                        img {
                            class: "h-8 w-8",
                            src: "img/exit.svg",
                        }
                    }
                },
                div {
                    img {
                        src: "img/trophy.svg",
                        class: "h-20 w-20 mx-auto"
                    }
                    p {
                        class: "text-center font-bold text-4xl my-4",
                        "THE WINNER IS"
                    }
                },
                div {
                    class: "flex flex-col basis-1/2 grow-0 shrink justify-evenly content-evenly",
                    state.players.iter().map(|player| {
                        let background = TITLE_COLORS[player.id-1];
                        let border = BORDER_COLORS[player.id-1];
                        let score = player.score.values().sum::<i32>();
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
        }
    ))
}
