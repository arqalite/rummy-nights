//! Rummy Nights, a rummy score counter written with Rust/Dioxus and Tailwind CSS.
//!
//! This module is the app entry-point, mostly setting up the model/state and rendering the initial screens.
//! It should be as basic as possible, with the majority of logic being written in separate modules.
//! For now all the other modules keep the code for each individual screen.

//For now I'd like to keep Clippy as annoying as possible to make sure the code looks and works somewhat fine.
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
//It doesn't understand Dioxus' quirks though, so this stays disabled.
#![allow(clippy::used_underscore_binding)]

use dioxus::fermi::use_atom_state;
use dioxus::prelude::*;
use std::collections::BTreeMap;

// The code is split into multiple modules, right now one for each screen,
// plus the css.rs file which holds some Tailwind CSS classes as static arrays
// so we can add them programmatically to each player.
mod css;
mod intro_screen;
mod player_select;
mod score_table;
mod winner_screen;

// All the data that concerns the entire app functionality,
// i.e. players, game status, options (soon), is stored in one single place.
// The goal is to have a single source of truth, so it's slightly inspired by MVC architecture.
// Fermi allows us to have the data available everywhere in the app while avoiding complex state management,
// or passing down values from component to component, which gets complicated, messy and tiresome easily.

static STATE: Atom<Model> = |_| Model {
    players: Vec::new(),
    game_status: GameStatus::NotStarted,
    screen: Screen::Intro,
};

#[derive(Clone)]
struct Model {
    players: Vec<Player>,
    game_status: GameStatus,
    screen: Screen,
}

#[derive(PartialEq, Clone)]
struct Player {
    id: usize,
    name: String,

    // We need to make sure the scores get stored uniquely, and in the same order they get added,
    // so BTreeMaps are the simplest structure that does the job,
    // and it's a well supported part of the standard library.
    // Might not be the fastest option, but the data is small and simple so I believe it's fine.
    score: BTreeMap<usize, i32>,
}

// Using an enum for the game status might not be the best idea,
// but it looks neater and removes the need for multiple booleans
// scattered across the code and passed down from component to component.
#[derive(PartialEq, Clone)]
enum GameStatus {
    NotStarted,
    Ongoing,
    Finished(String), //This String holds the winner's name
}
#[derive(PartialEq, Clone)]
enum Screen {
    Intro,
    PlayerSelect,
    Game,
    Winner
}

// This is the actual entry-point, and it should be kept as simple as possible.
// For now just managing the various screens is enough.
// Other work should be done in its respective modules.

fn app(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);
    let mut screen;

    match state.screen {
        Screen::Intro => screen = rsx!(intro_screen::intro()),
        Screen::PlayerSelect => screen = rsx!(player_select::player_select()),
        Screen::Game => screen = rsx!(score_table::score_table()),
        Screen::Winner => screen = rsx!(winner_screen::winner_screen())
    };

    //TODO: Remove after final screen is done.
    // state.with_mut(|mut_state| {
    //     mut_state.players = vec![
    //         Player {
    //             name: "Antonio".to_string(),
    //             id: 1,
    //             score: BTreeMap::new(), 
    //         },
    //         Player {
    //             name: "Vlad".to_string(),
    //             id: 2,
    //             score: BTreeMap::new(), 
    //         },
    //         Player {
    //             name: "Dani".to_string(),
    //             id: 3,
    //             score: BTreeMap::new(), 
    //         },
    //         Player {
    //             name: "Dalmina".to_string(),
    //             id: 4,
    //             score: BTreeMap::new(), 
    //         },
    //     ]
    // });

    // screen = rsx!(winner_screen::winner_screen());

    cx.render(rsx!(div {
        // For now we design for mobile, 
        // so we're restricting the max-width on desktop to match how a phone would look.
        class: "mx-auto max-w-md bg-slate-50 shadow-2xl border-x border-black h-screen",
        screen,
    }))
}

fn main() {
    dioxus::web::launch(app);
}
