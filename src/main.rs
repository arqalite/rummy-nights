use dioxus::{prelude::*, fermi::use_atom_state, core::UiEvent};
use dioxus::events::*;

static PLAYERS: Atom<Vec<Player>> = |_| vec![
        Player {
            id: 1,
            name: "Antonio".to_string(),
            score: vec![],
        },
        Player {
            id: 2,
            name: "Vlad".to_string(),
            score: vec![],
        },
        Player {
            id: 3,
            name: "Dalmina".to_string(),
            score: vec![],
        },
        Player {
            id: 4,
            name: "Daniel".to_string(),
            score: vec![],
        }
];

static TITLE_COLORS: [&str; 4] = [
    "bg-rose-400",
    "bg-cyan-400",
    "bg-green-400",
    "bg-violet-400"
];

static BORDER_COLORS: [&str; 4] = [
    "border-rose-400",
    "border-cyan-400",
    "border-green-400",
    "border-violet-400"
];
static FOCUS_OUTLINE_COLORS: [&str; 4] = [
    "focus:outline-rose-400",
    "focus:outline-cyan-400",
    "focus:outline-green-400",
    "focus:outline-violet-400"
];

static CARET_COLORS: [&str; 4] = [
    "caret-rose-400",
    "caret-cyan-400",
    "caret-green-400",
    "caret-violet-400"
];

static COLUMN_NUMBERS: [&str; 3] = [
    "grid-cols-2",
    "grid-cols-3",
    "grid-cols-4"
];

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
            crate::score_table()
    ))
}

fn score_table(cx: Scope) -> Element{
    let state = use_atom_state(&cx, PLAYERS);
    let columns = COLUMN_NUMBERS[state.len()-2];

    cx.render(rsx! (
        div{
            //Main table
            class: "grid {columns} mx-auto max-w-md mt-16 gap-x-5",

            state.iter().map(|player| {
                let sum = player.score.iter().sum::<i32>().to_string();
                let background = TITLE_COLORS[player.id-1];
                let border = BORDER_COLORS[player.id-1];

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
                            crate::score_input{
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
fn score_input(cx: Scope, id: usize) -> Element {
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
    let caret = CARET_COLORS[id-1];
    let border = FOCUS_OUTLINE_COLORS[id-1];

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

#[derive(PartialEq, Clone)]
struct Player {
    id: usize,
    name: String,
    score: Vec<i32>
}