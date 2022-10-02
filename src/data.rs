//! data.rs - data structures and custom types
//! Here we should only have structs, enums and vectors of Tailwind CSS classes.

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use dioxus::fermi::{Atom, use_atom_state};
use dioxus::prelude::*;

// MVC-style model, keeping all the app data in one place, so we have a single source of truth.
// Fermi allows us to have access available everywhere in the app while avoiding complex state management,
// or passing down values from component to component, which gets complicated, messy and tiresome easily.
pub static STATE: Atom<Model> = |_| Model {
    players: Vec::new(),
    game_status: GameStatus::NotStarted,
    screen: Screen::Menu,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Model {
    pub players: Vec<Player>,
    pub game_status: GameStatus,
    pub screen: Screen,
}

// Player data - one of these is constructed for each player in the game
#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: usize, //for tracking in the Vec, as order might change (e.g. deletion, sorting)
    pub name: String,

    // We need to make sure the scores get stored uniquely, and in the same order they get added,
    // so BTreeMaps are the simplest structure that does the job,
    // and it's a well supported part of the standard library.
    // Might not be the fastest option, but the data is small and simple so I believe it's fine.
    pub score: BTreeMap<usize, i32>,
}

pub fn add_player(cx: Scope, name: String) {
    let state = use_atom_state(&cx, STATE);

    let mut lowest_available_id = 0;

    for i in 1..5 {
        let slot = state.players.iter().find(|item| item.id == i);

        if slot == None {
            lowest_available_id = i;
            break;
        };
    }

    state.with_mut(|state| {
        if state.players.len() < 4 && lowest_available_id != 0 {
            state.players.push(Player {
                id: lowest_available_id,
                name,
                score: BTreeMap::new(),
            });
        };
    });
}

// Using an enum for the game status might not be the best idea,
// but it looks neater and removes the need for multiple booleans
// scattered across the code and passed down from component to component.
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum GameStatus {
    NotStarted,
    Ongoing,
    Finished, //This String holds the winner's name
}

// Another enum but for screen management.
// Add a new entry here if you need to add a screen, then edit the match arms in main.rs.
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Screen {
    Menu,
    PlayerSelect,
    Game,
    Winner,
}

// We use LocalStorage to keep track of unfinished games.
// This is helpful in case of accidental refreshes, or just browsers bugging out for no reason.
// No need for error handling.
pub fn read_local_storage() -> Result<Model, &'static str> {
    match LocalStorage::get::<serde_json::Value>("state") {
        Ok(json_state) => match serde_json::from_value::<crate::Model>(json_state) {
            Ok(new_state) => Ok(new_state),
            Err(_) => Err("Could not parse local storage."),
        },
        Err(_) => Err("Could not read local storage."),
    }
}

//
// Place here any Tailwind classes you might need to use programmatically in the project.
//
pub static TITLE_COLORS: [&str; 4] = [
    "bg-rose-400",
    "bg-cyan-400",
    "bg-green-400",
    "bg-violet-400",
];

pub static BORDER_COLORS: [&str; 4] = [
    "border-rose-400",
    "border-cyan-400",
    "border-green-400",
    "border-violet-400",
];

pub static CARET_COLORS: [&str; 4] = [
    "caret-rose-400",
    "caret-cyan-400",
    "caret-green-400",
    "caret-violet-400",
];

pub static COLUMN_NUMBERS: [&str; 3] = ["grid-cols-2", "grid-cols-3", "grid-cols-4"];
