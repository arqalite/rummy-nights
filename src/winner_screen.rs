
use dioxus::prelude::*;
use dioxus::fermi::use_atom_state;
use gloo_storage::{LocalStorage, Storage};

use crate::STATE;
use crate::data::{
    Screen,
    TITLE_COLORS,
    BORDER_COLORS
};


pub fn winner_screen(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);
    let mut player_count = 0;

    let return_to_table = |_| {
        state.with_mut(|state| {
            state.screen = Screen::Game;
        });
    };
    
    state.with_mut(|mut_state| {
        mut_state.players.sort_by(|a,b| {
            let temp_sum_a = a.score.values().sum::<i32>();
            let temp_sum_b = b.score.values().sum::<i32>();
            
            temp_sum_a.cmp(&temp_sum_b)
        });

        mut_state.players.reverse();
    });

    LocalStorage::clear();

    cx.render(rsx!(
        div {
            class: "h-16 grid grid-cols-3",
            button {
                class: "mx-auto h-16 relative left-[-30%]",
                onclick: return_to_table,
                img {
                    class: "h-8 w-8",
                    src: "img/back.svg",
                }
            }
            p {
                ""
            }
            button {
                class: "mx-auto h-16 relative right-[-30%]",
                //onclick:
                img {
                    class: "h-8 w-8",
                    src: "img/exit.svg",
                }
            }
        },
        div {
            class: "mx-auto mt-8",
            img {
                src: "img/trophy.svg",
                class: "h-32 w-32 mx-auto"
            }
            p {
                class: "text-center font-bold text-5xl my-12",
                "THE WINNER IS"
            }
        },
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
                class: "grid grid-cols-2 my-8 h-20",
                div {
                    class: "{style} justify-self-center",
                    p {
                        class: "text-center mb-2 {style2}",
                        "{score}"
                    }
                }
                div {
                    class: "h-12 basis-1/4 {background} self-center rounded-full mr-16",
                    p {
                        class: "text-center relative top-[50%] -translate-y-1/2 text-white font-semibold",
                        "{player.name}"
                    }
                }
            }
            )
        })
    ))
}