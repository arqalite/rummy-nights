use dioxus::fermi::AtomRef;
use gloo_console::log;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// MVC-style model, keeping all the app data in one place, so we have a single source of truth.
// Fermi allows us to have access available everywhere in the app while avoiding complex state management,
// or passing down values from component to component, which gets complicated, messy and tiresome easily.
pub static STATE: AtomRef<Model> = |_| Model {
    players: Vec::new(),
    game_status: GameStatus::NotStarted,
    screen: Screen::Menu,
    checked_storage: false,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Model {
    pub players: Vec<Player>,
    pub game_status: GameStatus,
    pub screen: Screen,
    checked_storage: bool,
}

impl Model {
    pub const fn new() -> Self {
        Self {
            players: Vec::new(),
            game_status: GameStatus::NotStarted,
            screen: Screen::Menu,
            checked_storage: false,
        }
    }

    pub fn add_player(&mut self, name: String) {
        let id = self.players.len() + 1;

        if self.players.len() < 4 {
            self.players.push(Player {
                id,
                name,
                score: BTreeMap::new(),
                bonus: BTreeMap::new(),
            });
        };
    }

    pub fn remove_player(&mut self, id: usize) {
        let mut counter = 1;

        self.players.retain(|player| player.id != id);

        for player in &mut self.players {
            player.id = counter;
            counter += 1;
        }
    }

    pub fn grant_bonus(&mut self, id: usize) {
        let games_played: Vec<usize> = self
            .players
            .iter()
            .map(|player| player.score.len())
            .collect();

        self.players[id - 1]
            .bonus
            .insert(*games_played.iter().max().unwrap(), 50);
    }

    pub fn load_existing_game(&mut self) {
        if !self.checked_storage {
            match LocalStorage::get::<serde_json::Value>("state") {
                Ok(json_state) => match serde_json::from_value::<Self>(json_state) {
                    Ok(new_state) => {
                        self.players = new_state.players;
                        self.game_status = new_state.game_status;

                        // SessionStorage is currently used to keep track of ongoing game sessions.
                        // If they refresh or tab out in the current session,
                        // we make sure in main.rs that they return to the screen they were in already.
                        match SessionStorage::get::<serde_json::Value>("session") {
                            Ok(json_state) => match serde_json::from_value::<bool>(json_state) {
                                Ok(_) => self.screen = Screen::Game,
                                Err(_) => log!("Could not parse session storage."),
                            },
                            Err(_) => log!("Could not read session storage."),
                        }
                    }
                    Err(_) => log!("Could not parse local storage."),
                },
                Err(_) => log!("Could not read local storage."),
            }
        };
        self.checked_storage = true;
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
    pub score: BTreeMap<usize, usize>,
    pub bonus: BTreeMap<usize, usize>,
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
