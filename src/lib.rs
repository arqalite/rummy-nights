//! data.rs - data structures and custom types
//! Here we should only have structs, enums, functions that deal with data, and Tailwind CSS classes.

use dioxus::prelude::*;
use gloo_console::log;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod screens;

// Preludes generate a lot of mixed opinions,
// but in my opinion having a decent code editor solves most of the issues it generates.
// Having frequently-used types and functions ready without importing them makes the code so much cleaner and readable.
pub mod prelude {
    pub use crate::screens::*; //All the screens reference other screens so this is a must.
    pub use crate::Screen;

    pub use crate::STATE; //This is a must as well, the state is used everywhere.

    //Exposing the main data types.
    pub use crate::GameStatus;
    pub use crate::Model;
    pub use crate::Player;
}

// MVC-style model, keeping all the app data in one place, so we have a single source of truth.
// Fermi allows us to have access available everywhere in the app while avoiding complex state management,
// or passing down values from component to component, which gets complicated, messy and tiresome easily.
pub static STATE: AtomRef<Model> = |_| Model {
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

impl Model {
    pub const fn new() -> Self {
        Self {
            players: Vec::new(),
            game_status: GameStatus::NotStarted,
            screen: Screen::Menu,
        }
    }
}

// Player data - one of these is constructed for each player in the game
#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: usize, //for tracking in the Vec, as order might change (e.g. deletion, sorting)
    pub name: String,

    // We need to make sure the scores get stored uniquely, and in the same order they get added,
    // so BTreeMaps are the simplest structure that does the job,
    // and it's a well supported part of the standard library.
    // Might not be the fastest option, but the data is small and simple so I believe it's fine.
    pub score: BTreeMap<usize, i32>,
}

// Remove players and assign consecutive IDs without gaps.
pub fn remove_player(cx: Scope, id: usize) {
    let state = use_atom_ref(&cx, STATE);
    let mut counter = 1;

    state.write().players.retain(|player| player.id != id);

    for player in &mut state.write().players {
        player.id = counter;
        counter += 1;
    }
}

// Add a new player.
// As the delete function resets IDs to make sure they're consecutive,
// we can just assume the smallest available ID is len() + 1.
pub fn add_player(cx: Scope, name: String) {
    let mut state = use_atom_ref(&cx, STATE).write();
    let id = state.players.len() + 1;

    if state.players.len() < 4 {
        state.players.push(Player {
            id,
            name,
            score: BTreeMap::new(),
        });
    };
}

// Using an enum for the game status might not be the best idea,
// but it looks neater and removes the need for multiple booleans
// scattered across the code and passed down from component to component.
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum GameStatus {
    NotStarted,
    Ongoing,
    Finished,
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

pub fn load_existing_game(cx: Scope) {
    let state = use_atom_ref(&cx, STATE);
    let has_checked_storage = use_state(&cx, || false);

    if !has_checked_storage {
        match LocalStorage::get::<serde_json::Value>("state") {
            Ok(json_state) => match serde_json::from_value::<crate::Model>(json_state) {
                Ok(new_state) => {
                    state.write().players = new_state.players;
                    state.write().game_status = new_state.game_status;

                    // SessionStorage is currently used to keep track of ongoing game sessions.
                    // If they refresh or tab out in the current session,
                    // we make sure in main.rs that they return to the screen they were in already.
                    match SessionStorage::get::<serde_json::Value>("session") {
                        Ok(json_state) => match serde_json::from_value::<bool>(json_state) {
                            Ok(_) => state.write().screen = Screen::Game,
                            Err(_) => log!("Could not parse session storage."),
                        },
                        Err(_) => log!("Could not read session storage."),
                    }

                    has_checked_storage.set(true);
                }
                Err(_) => log!("Could not parse local storage."),
            },
            Err(_) => log!("Could not read local storage."),
        }
    };
}

pub fn print_version_number(cx: Scope) -> Element {
    let version = env!("BUILD_VERSION");
    cx.render(rsx!("{version}"))
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
