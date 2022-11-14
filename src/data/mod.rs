//! The back-end part - dealing with data and logic.

pub mod tailwind_classes;

use dioxus::fermi::AtomRef;
use dioxus::prelude::*;
use gloo_console::log;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub static STATE: AtomRef<Model> = |_| Model::new();
pub static SETTINGS: AtomRef<Settings> = |_| Settings::new();

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Settings {
    pub max_score: i32,
    pub tile_bonus_value: i32,
    pub checked_storage: bool,
}

impl Settings {
    fn new() -> Self {
        Settings {
            max_score: 1000,
            tile_bonus_value: 50,
            checked_storage: false,
        }
    }

    pub fn load(&mut self) {
        log!("Trying to load settings from storage.");
        match LocalStorage::get::<serde_json::Value>("settings") {
            Ok(json_settings) => match serde_json::from_value::<Self>(json_settings) {
                Ok(new_settings) => {
                    *self = new_settings;
                    log!(format!("Loaded settings: {:?}", self));
                }
                Err(_) => log!("Could not parse settings from local storage."),
            },
            Err(_) => log!("Could not read settings from local storage."),
        }
        self.checked_storage = true;
    }

    pub fn save(&self) {
        log!("Saving settings.");

        LocalStorage::set("settings", self.clone()).unwrap();
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Model {
    pub players: Vec<Player>,
    pub game_status: GameStatus,
    pub screen: Screen,
    pub checked_storage: bool,
    pub round: usize,
    pub total_rounds: usize,
    pub new_round_started: bool,
    pub show_end_once: bool,
    pub tile_bonus_toggle: bool,
    pub tile_bonus_granted: bool,
    pub sorted_players: Vec<Player>,
    pub is_sorted: bool,
    pub settings: Settings,
}

impl Model {
    pub fn new() -> Self {
        log!("Creating new state.");
        Self {
            players: Vec::new(),
            game_status: GameStatus::NotStarted,
            screen: Screen::Menu,
            checked_storage: false,
            round: 0,
            total_rounds: 0,
            new_round_started: true,
            show_end_once: true,
            tile_bonus_toggle: false,
            tile_bonus_granted: false,
            sorted_players: Vec::new(),
            is_sorted: false,
            settings: Settings::new(),
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
                sum: 0,
                bonus: BTreeMap::new(),
            });
        };
    }

    pub fn remove_player(&mut self, id: usize) {
        log!("Removing player.");

        self.players.retain(|player| player.id != id);

        let mut counter = 1;
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

        if *max_games == *min_games && self.round != *max_games {
            self.round = *max_games;
            self.new_round_started = true;
            self.tile_bonus_granted = false;
        } else {
            self.new_round_started = false;
        }

        self.save_game()
    }

    pub fn grant_bonus(&mut self, id: usize) {
        log!("Granting player bonus.");

        for mut player in &mut self.players {
            if player.id == id {
                player
                    .bonus
                    .insert(self.round, self.settings.tile_bonus_value);
                player.sum =
                    player.score.values().sum::<i32>() + player.bonus.values().sum::<i32>();
            }
        }
        self.tile_bonus_granted = true;
        self.save_game();
    }

    pub fn create_game(&mut self) {
        log!("Creating new game.");

        let settings = self.settings.clone();
        log!(format!("Backed up settings are {:?}", settings));

        *self = Model::new();
        self.settings = settings;
        log!(format!("Actual settings are {:?}", self.settings));

        // Since we create a new game, storage is already 'checked'.
        self.checked_storage = true;
        self.screen = Screen::PlayerSelect;
    }

    pub fn start_game(&mut self) {
        log!("Starting new game.");

        if self.players.len() >= 2 {
            let mut counter = 1;
            for player in &mut self.players {
                player.id = counter;
                counter += 1;
            }

            LocalStorage::delete("state");
            SessionStorage::delete("session");

            self.game_status = GameStatus::Ongoing;
            self.screen = Screen::Game;
            self.save_game();
        };
    }

    pub fn load_existing_game(&mut self) {
        log!("Trying to load game from storage.");
        match LocalStorage::get::<serde_json::Value>("state") {
            Ok(json_state) => match serde_json::from_value::<Self>(json_state) {
                Ok(new_state) => {
                    *self = new_state;
                    self.screen = Screen::Menu;

                    log!("Loaded game.");
                    match SessionStorage::get::<serde_json::Value>("session") {
                        Ok(json_state) => match serde_json::from_value::<bool>(json_state) {
                            Ok(_) => {
                                self.screen = Screen::Game;
                                log!("Loaded session.");
                            }
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

        let total_scores: Vec<i32> = self
            .players
            .iter()
            .map(|player| player.score.values().sum::<i32>() + player.bonus.values().sum::<i32>())
            .collect();

        let max = *(total_scores.iter().max().unwrap());

        if max >= self.settings.max_score && self.new_round_started {
            let no_of_winners = self
                .players
                .iter()
                .filter(|player| {
                    player.score.values().sum::<i32>() + player.bonus.values().sum::<i32>() >= max
                })
                .count();

            if no_of_winners == 1 {
                let winner: Vec<&Player> = self
                    .players
                    .iter()
                    .filter(|player| player.sum >= self.settings.max_score)
                    .collect();
                let winner_name = &winner[0].name;

                self.game_status = GameStatus::Finished(winner_name.to_string());
                if self.show_end_once {
                    self.screen = Screen::EndGame;
                    self.show_end_once = false;
                };
                self.save_game();
            }
        }
    }

    pub fn reset_game(&mut self) {
        for player in &mut self.players {
            player.score.clear();
            player.bonus.clear();
            player.sum = 0;
        }
        self.screen = Screen::Game;
        self.game_status = GameStatus::Ongoing;
        self.show_end_once = true;
        self.total_rounds += self.round;
        self.round = 0;
        self.new_round_started = true;
        self.show_end_once = true;
        self.tile_bonus_toggle = false;
        self.tile_bonus_granted = false;
        self.sorted_players = Vec::new();
        self.is_sorted = false;
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
    pub sum: i32,
    pub bonus: BTreeMap<usize, i32>,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum GameStatus {
    NotStarted,
    Ongoing,
    Finished(String),
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Screen {
    Menu,
    PlayerSelect,
    Game,
    EndGame,
    Settings,
}

/// Renders the version number (for releases) or the timestamp
/// (for dev builds).
///
/// It uses the `BUILD_VERSION` environment variable created in build.rs.
pub fn print_version_number(cx: Scope) -> Element {
    let version = env!("BUILD_VERSION");
    cx.render(rsx!("{version}"))
}
