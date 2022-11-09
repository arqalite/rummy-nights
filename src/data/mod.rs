//! The back-end part - dealing with data and logic.

pub mod tailwind_classes;

use dioxus::fermi::AtomRef;
use dioxus::prelude::*;
use gloo_console::log;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

static FINAL_SCORE: i32 = 1000;
static TILE_BONUS_VALUE: i32 = 50;

pub static STATE: AtomRef<Model> = |_| Model::new();

#[derive(Clone, Serialize, Deserialize)]
pub struct Model {
    pub players: Vec<Player>,
    pub game_status: GameStatus,
    pub screen: Screen,
    pub checked_storage: bool,
    round: usize,
    pub new_round_started: bool,
}

#[derive(Clone, Serialize, Deserialize)]
struct TileBonusModel;

impl Model {
    pub fn new() -> Self {
        log!("Creating new state.");
        Self {
            players: Vec::new(),
            game_status: GameStatus::NotStarted,
            screen: Screen::Menu,
            checked_storage: false,
            round: 0,
            new_round_started: true,
        }
    }

    pub fn add_player(&mut self, name: String) {
        log!("Adding player.");

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
        log!("Removing player.");

        let mut counter = 1;

        self.players.retain(|player| player.id != id);

        for player in &mut self.players {
            player.id = counter;
            counter += 1;
        }
    }

    pub fn move_up(&mut self, id: usize) {
        log!("Moving player up.");

        for i in 0..self.players.len() {
            if i != 0 && self.players[i].id == id {
                let moved_player = self.players.remove(i);

                self.players.insert(i - 1, moved_player);
            }
        }
    }

    pub fn move_down(&mut self, id: usize) {
        log!("Moving player down.");

        for i in 0..self.players.len() - 1 {
            if self.players[i].id == id {
                let moved_player = self.players.remove(i);

                if i < self.players.len() {
                    self.players.insert(i + 1, moved_player);
                };

                break;
            }
        }
    }

    pub fn check_round(&mut self) {
        log!("Checking round status.");

        let games_played: Vec<usize> = self
            .players
            .iter()
            .map(|player| player.score.len())
            .collect();

        let max_games = games_played.iter().max().unwrap();
        let min_games = games_played.iter().min().unwrap();

        if max_games == min_games {
            self.round = *max_games;
            self.new_round_started = true;
        }

        log!(format!("round is {}", self.round))
    }

    pub fn grant_bonus(&mut self, id: usize) {
        log!("Granting player bonus.");

        self.players[id - 1]
            .bonus
            .insert(self.round, TILE_BONUS_VALUE);

        self.new_round_started = false;
    }

    pub fn create_game(&mut self) {
        log!("Creating new game.");

        LocalStorage::clear();
        SessionStorage::clear();

        *self = Model::new();
        
        // Since we create a new game, storage is already 'checked'.
        self.checked_storage = true;
        
        self.screen = Screen::PlayerSelect;
    }

    pub fn start_game(&mut self) {
        log!("Starting new game.");

        if self.players.len() >= 2 {
            self.game_status = GameStatus::Ongoing;
            self.screen = Screen::Game;
        };
    }

    pub fn load_existing_game(&mut self) {
            log!("Trying to load game from storage.");
            match LocalStorage::get::<serde_json::Value>("state") {
                Ok(json_state) => match serde_json::from_value::<Self>(json_state) {
                    Ok(new_state) => {
                        self.players = new_state.players;
                        self.game_status = new_state.game_status;
                        self.new_round_started = new_state.new_round_started;
                        self.round = new_state.round;

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
        self.checked_storage = true;
    }

    pub fn save_game(&self) {
        log!("Saving game.");

        LocalStorage::set("state", self.clone()).unwrap();
        SessionStorage::set("session", true).unwrap();
    }

    pub fn check_game_status(&mut self) {
        log!("Checking game status.");

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

        if max >= FINAL_SCORE && self.new_round_started {
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
                self.game_status = GameStatus::Finished;
            }
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
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
