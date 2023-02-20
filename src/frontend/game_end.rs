use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, SessionStorage, Storage};

use crate::prelude::*;

pub fn screen(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let mut player_count = 0;

    if !state.read().game.is_sorted {
        state.write().game.sort_players();
    }

    let winner_label = get_text(state.read().settings.language, "winner_label").unwrap();

    log!("Rendering end screen.");
    cx.render(rsx!(
        nav_bar(),
        div {
            class: "px-8 flex flex-col absolute w-screen px-8 sm:max-w-lg top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 gap-6 justify-evenly",
            div {
                class: "h-24 w-24 mx-auto rounded-full",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                assets::trophy_icon()
            }
            p {
                class: "text-center font-bold text-4xl",
                "{winner_label}"
            }
            state.read().game.sorted_players.iter().map(|player| {
                log!("Rendering players.");

                let background = BG_COLORS[player.color_index];
                let border = BORDER_COLORS[player.color_index];
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
        log!("Deleting game and returning to main menu.");
        LocalStorage::delete("state");
        SessionStorage::delete("session");
        *state.write() = Model::new();
    };

    let restart_game = |_| {
        log!("Restarting game.");
        state.write().reset_game();
    };

    log!("Rendering nav bar.");
    cx.render(rsx!(
        div {
            class: "h-16 grid grid-cols-3 px-8",
            button {
                class: "col-start-1 justify-self-start",
                onclick: |_| {
                    state.write().screen = Screen::Game;
                },
                div {
                    class: "h-10 scale-x-[-1]",
                    assets::back()
                }
            }
            button {
                class: "col-start-2 justify-self-center",
                onclick: delete_and_exit_game,
                div {
                    class: "h-10",
                    assets::home(),
                }
            }
            button {
                class: "col-start-3 justify-self-end",
                onclick: restart_game,
                div {
                    class: "h-10",
                    assets::replay_icon()
                }
            }
        }
    ))
}
