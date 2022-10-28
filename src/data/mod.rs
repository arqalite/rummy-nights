//! The back-end part - dealing with data and logic.

pub mod tailwind_classes;

use dioxus::prelude::*;
use dioxus::fermi::AtomRef;
use gloo_console::log;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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

    pub fn start_new_game(&mut self) {
        LocalStorage::clear();
        SessionStorage::clear();

        *self = Model::new();
        self.screen = Screen::PlayerSelect;
    }

    pub fn load_existing_game(&mut self) {
        if !self.checked_storage {
            match LocalStorage::get::<serde_json::Value>("state") {
                Ok(json_state) => match serde_json::from_value::<Self>(json_state) {
                    Ok(new_state) => {
                        self.players = new_state.players;
                        self.game_status = new_state.game_status;

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

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub score: BTreeMap<usize, i32>,
    pub bonus: BTreeMap<usize, i32>,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum GameStatus {
    NotStarted,
    Ongoing,
    Finished,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Screen {
    Menu,
    PlayerSelect,
    Game,
    Winner,
}

/// Renders the version number (for releases) or the timestamp
/// (for dev builds).
/// 
/// It uses the `BUILD_VERSION` environment variable created in build.rs.
pub fn print_version_number(cx: Scope) -> Element {
    let version = env!("BUILD_VERSION");
    cx.render(rsx!("{version}"))
}
