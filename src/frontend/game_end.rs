use crate::prelude::*;
use dioxus::prelude::*;

pub fn EndScreen(cx: Scope) -> Element {
    log!("Rendering end screen.");
    let state = fermi::use_atom_ref(cx, STATE);
    if !state.read().game.is_sorted {
        state.write().game.sort_players();
    }
    let sorted_players = state.read().game.sorted_players.clone();

    render!(
        NavBar {
        },
        div {
            class: "px-8 flex flex-col absolute w-screen px-8 sm:max-w-lg top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 gap-6 justify-evenly",
            div {
                class: "h-24 w-24 mx-auto rounded-full",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                assets::TrophyIcon {}
            }
            p {
                class: "text-center font-bold text-4xl",
                get_text(cx,"winner_label")
            }
            sorted_players.iter().map(|player| {
                rsx!(
                    PlayerItem {
                        player: player.clone(),
                    }
                )
            })
        }
    )
}

#[inline_props]
fn PlayerItem(cx: Scope, player: Player) -> Element {
    log!("Rendering player: ");

    let background = BG_COLORS[player.color_index];
    let border = BORDER_COLORS[player.color_index];
    let score = player.sum;
    let mut style;
    let style2;

    if player.winner {
        style = "h-20 w-20 rounded-full text-white font-bold text-lg ".to_string();
        style.push_str(background);
        style2 = "relative top-[50%] -translate-y-1/2".to_string();
    } else {
        style = "border-b-[7px] rounded-md my-auto w-20 ".to_string();
        style.push_str(border);
        style2 = String::new();
    };

    render!(
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
}

fn NavBar(cx: Scope) -> Element {
    log!("Rendering nav bar.");
    let state = fermi::use_atom_ref(cx, STATE);
    render!(
        div {
            class: "h-16 grid grid-cols-3 px-8",
            button {
                class: "col-start-1 justify-self-start",
                onclick: move |_| state.write().go_to_screen(Screen::Game),
                div {
                    class: "h-10 scale-x-[-1]",
                    assets::BackIcon {}
                }
            }
            button {
                class: "col-start-2 justify-self-center",
                onclick: move |_| state.write().finish_game(),
                div {
                    class: "h-10",
                    assets::HomeIcon {}
                }
            }
            button {
                class: "col-start-3 justify-self-end",
                onclick: move |_| state.write().reset_game(),
                div {
                    class: "h-10",
                    assets::ReplayIcon {}
                }
            }
        }
    )
}
