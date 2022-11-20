use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Settings {
    pub end_game_at_score: bool,
    pub max_score: i32,
    pub use_tile_bonus: bool,
    pub tile_bonus_value: i32,
    pub enable_dealer_tracking: bool,
    pub enable_score_editing: bool,
    pub checked_storage: bool,
}

impl Settings {
    pub fn new() -> Self {
        log!("Initializing settings.");
        Settings {
            end_game_at_score: true,
            max_score: 1000,
            use_tile_bonus: true,
            tile_bonus_value: 50,
            enable_dealer_tracking: true,
            enable_score_editing: true,
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

    pub fn set_max_score(&mut self, value: i32) {
        log!("Set max score.");
        self.max_score = value;
        self.save();
        self.checked_storage = false;
    }

    pub fn set_tile_bonus(&mut self, value: i32) {
        log!("Set tile bonus.");
        self.tile_bonus_value = value;
        self.save();
        self.checked_storage = false;
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
