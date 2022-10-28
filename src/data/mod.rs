//! The back-end part - dealing with data and logic.

pub mod tailwind_classes;

use dioxus::fermi::AtomRef;
use dioxus::prelude::*;
use gloo_console::log;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

static FINAL_SCORE: i32 = 1000;

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

    pub fn create_game(&mut self) {
        LocalStorage::clear();
        SessionStorage::clear();

        *self = Model::new();
        self.screen = Screen::PlayerSelect;
    }

    pub fn start_game(&mut self) {
        if self.players.len() >= 2 {
            self.game_status = GameStatus::Ongoing;
            self.screen = Screen::Game;
        };
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

    pub fn save_game(&self) {
        LocalStorage::set("state", self.clone()).unwrap();
        SessionStorage::set("session", true).unwrap();
    }

    pub fn check_game_status(&self) -> GameStatus {
        let mut game_status = GameStatus::Ongoing;

        let (total_scores, games_played): (Vec<i32>, Vec<usize>) = self
            .players
            .iter()
            .map(|player| {
                let total = player.score.values().sum::<i32>() + player.bonus.values().sum::<i32>();

                (total, player.score.len())
            })
            .unzip();

        let max = *(total_scores.iter().max().unwrap());
        log!(format!("max is {}", max));

        if max >= FINAL_SCORE {
            let no_of_winners = self
                .players
                .iter()
                .filter(|player| {
                    player.score.values().sum::<i32>() + player.bonus.values().sum::<i32>() >= max
                })
                .count();

            if (games_played.iter().min().unwrap() == games_played.iter().max().unwrap())
                && no_of_winners == 1
            {
                game_status = GameStatus::Finished;
            }
        }

        game_status
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
