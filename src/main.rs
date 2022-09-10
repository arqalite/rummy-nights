//! Rummy Nights, a rummy score counter written with Rust/Dioxus and Tailwind CSS.
//! 
//! This module is the app entry-point, mostly setting up the model/state and rendering the initial screens.
//! It should be as basic as possible, with the majority of logic being written in separate modules.
//! For now all the other modules keep the code for each individual screen.

//For now I'd like to keep Clippy as annoying as possible to make sure the code looks and works somewhat fine.
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
//It doesn't understand Dioxus' quirks though, so this stays disabled.
#![allow(clippy::used_underscore_binding)] 

use dioxus::prelude::*;
use dioxus::fermi::use_atom_state;
use std::collections::BTreeMap;

// The code is split into multiple modules, right now one for each screen, 
// plus the css.rs file which holds some Tailwind CSS classes as static arrays 
// so we can add them programmatically to each player.
mod score_table;
mod player_select;
mod css;

// All the data that concerns the entire app functionality, 
// i.e. players, game status, options (soon), is stored in one single place.
// The goal is to have a single source of truth, so it's slightly inspired by MVC architecture.
// Fermi allows us to have the data available everywhere in the app while avoiding complex state management, 
// or passing down values from component to component, which gets complicated, messy and tiresome easily.

static STATE: Atom<Model> = |_| Model {
    players: Vec::new(),
    game_status: GameStatus::NotStarted
};

#[derive(Clone)]
struct Model {
    players: Vec<Player>,
    game_status: GameStatus
}

#[derive(PartialEq, Clone)]
struct Player {
    id: usize,
    name: String,
    
    // We need to make sure the scores get stored uniquely, and in the same order they get added, 
    // so BTreeMaps are the simplest structure that does the job, 
    // and it's a well supported part of the standard library.
    // Might not be the fastest option, but the data is small and simple so I believe it's fine. 
    score: BTreeMap<usize, i32>
}


// Using an enum for the game status might not be the best idea, 
// but it looks neater and removes the need for multiple booleans 
// scattered across the code and passed down from component to component.
#[derive(PartialEq, Clone)]
enum GameStatus {
    NotStarted,
    Ongoing,
    Finished(String),
}


fn app(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);
    let players = &state.players;
    let game_status = &state.game_status;

    let screen;
    
    let onclick = |_| {
        if players.len() >= 2 {
            state.with_mut(|state| {
                state.game_status = GameStatus::Ongoing
            })
        };
    };

    match game_status {
        GameStatus::NotStarted => {
            screen = rsx!(
                div {
                    class: "",
                    player_select::input_screen()
                    button {
                        class: "w-full text-center",
                        onclick: onclick,
                        "Start"
                    }
                }
            )
        },
        GameStatus::Ongoing | GameStatus::Finished(_) => {
            screen = rsx! (
                score_table::score_table(),
            )
        }
    }
    
    cx.render(screen)
}

// For now we target web via WebAssembly, 
// but the plan is to have a mobile app at some point.
// Most likely we'll go for a progressive web app to avoid having two codebases.
fn main() {
    dioxus::web::launch(app);
}
