use dioxus::fermi::AtomRef;
use gloo_console::log;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use serde::{Deserialize, Serialize};

use crate::backend::prelude::*;

pub static STATE: AtomRef<Model> = |_| Model::new();

#[derive(Clone, Serialize, Deserialize)]
pub struct Model {
    pub game: Game,
    pub screen: Screen,
    pub show_end_once: bool,
    pub checked_storage: bool,
    pub settings: Settings,
}

impl Model {
    pub fn new() -> Self {
        log!("Creating new state.");
        Self {
            game: Game::new(),
            screen: Screen::Menu,
            show_end_once: true,
            checked_storage: false,
            settings: Settings::new(),
        }
    }

    pub fn create_game(&mut self) {
        log!("Creating new game.");

        let settings = self.settings.clone();
        log!(format!("Backed up settings are {:?}", settings));

        *self = Model::new();
        self.settings = settings;
        self.game.tile_bonus_value = self.settings.tile_bonus_value;
        self.game.max_score = self.settings.max_score;

        log!(format!("Actual settings are {:?}", self.settings));
        log!(format!(
            "Game settings are {:?} and {:?}",
            self.game.tile_bonus_value, self.game.max_score
        ));

        // Since we create a new game, storage is already 'checked'.
        self.checked_storage = true;
        self.screen = Screen::PlayerSelect;
    }

    pub fn load_existing_game(&mut self) {
        log!("Trying to load game from storage.");
        match LocalStorage::get::<serde_json::Value>("game") {
            Ok(json_state) => match serde_json::from_value::<Game>(json_state) {
                Ok(new_state) => {
                    self.game = new_state;
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

    // Just exposing functions so frontend doesn't have to reach deep into the state,
    // and extending those that need to modify the state.
    pub fn get_winner(&self) -> String {
        log!("Checking if winner exists.");
        self.game.get_winner()
    }

    pub fn sort_players(&mut self) {
        log!("Sorting players.");
        self.game.sort_players()
    }

    pub fn add_score(&mut self, player_id: usize, value: i32) {
        log!("Adding score.");
        self.game.add_score(player_id, value);
        self.check_status()
    }

    pub fn start_game(&mut self) {
        log!("Starting game.");

        if self.game.start_game() {
            self.screen = Screen::Game;
        };
    }

    pub fn reset_game(&mut self) {
        log!("Resetting game.");
        self.game.reset_game();
        self.screen = Screen::Game;
    }

    pub fn grant_bonus(&mut self, id: usize) {
        log!("Granting bonus.");

        self.game.grant_bonus(id);
    }

    pub fn check_status(&mut self) {
        log!("Check game status.");

        if !self.settings.end_game_at_score {
            return;
        };

        self.game.check_status();

        if self.game.status == GameStatus::Finished && self.show_end_once {
            self.screen = Screen::EndGame;
            self.show_end_once = false;
        };
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
