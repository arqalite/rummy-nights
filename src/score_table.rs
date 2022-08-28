use dioxus::prelude::*;
use dioxus::events::*;
use dioxus::fermi::use_atom_state;
use dioxus::core::UiEvent;

use crate::PLAYERS;
use crate::statics;

pub fn score_table(cx: Scope) -> Element{
    let state = use_atom_state(&cx, PLAYERS);
    let columns = statics::COLUMN_NUMBERS[state.len()-2];

    cx.render(rsx! (
        div{
            //Main table
            class: "grid {columns} mx-auto px-5 max-w-md mt-16 gap-x-5",

            state.iter().map(|player| {
                let sum = player.score.iter().sum::<i32>().to_string();
                let background = statics::TITLE_COLORS[player.id-1];
                let border = statics::BORDER_COLORS[player.id-1];

                rsx!(
                    div{
                        //Column for each player
                        class: "",
                        div {
                            // Name - first cell
                            class: "rounded-full h-8 {background} py-1 mb-2 shadow",
                            p {
                                class: "text-center my-auto",
                                "{player.name}"
                            }
                        }
                        div {
                            //Scores - dynamic
                            player.score.iter().map(|score| {
                                let score_text = score.to_string();
                                rsx!(
                                    p {
                                        class: "rounded text-center border-b-2 mb-2 h-8 {border}",
                                        "{score_text}"
                                    }
                                )
                            })
                        }
                        div {
                            //Input box
                            crate::score_table::score_input{
                                id: player.id
                            }
                        }
                        div {
                            //Total box
                            class: "rounded border-y-4 {border} h-8",
                            p {
                                class: "text-center font-semibold",
                                "{sum}"
                            }
                        }
                    }
                )
            })
        }
    ))
}

#[inline_props]
pub fn score_input(cx: Scope, id: usize) -> Element {
    let buffer = use_state(&cx, || String::new());
    let onkeypress = move |evt: UiEvent<KeyboardData>| {
        if evt.key.as_str() == "Enter"{
            match buffer.parse::<i32>() {
                Ok(number) => {
                    let state = use_atom_state(&cx, PLAYERS);

                    state.with_mut(|mut_state| {
                        for player in mut_state.iter_mut() {
                            if *id == player.id {
                                player.score.push(number);
                            }
                        }
                    })
                }
                Err(_) => {
                    ()
                }
            };
            buffer.set(String::new());
        }
    };
    let oninput = move |evt:UiEvent<FormData>| {
        buffer.set(evt.value.clone());
    };
    let caret = statics::CARET_COLORS[id-1];
    let border = statics::FOCUS_OUTLINE_COLORS[id-1];

    cx.render(rsx!(
        input {
            class: "{caret} appearance-none bg-transparent h-8 w-full mb-2 text-center rounded-full focus:outline-1 {border}",
            placeholder: "Insert score",
            value: "{buffer}",
            onkeypress: onkeypress,
            oninput: oninput,
        }
    ))
}