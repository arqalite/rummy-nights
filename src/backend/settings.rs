use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Settings {
    pub end_game_at_score: bool,
    pub max_score: i32,
    pub use_tile_bonus: bool,
    pub tile_bonus_value: i32,
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
        log!(format!("actually setting score to {}", value));
        self.max_score = value;
        self.save();
        self.checked_storage = false;
        log!(format!("so now score is {}", self.max_score));
    }

    pub fn set_tile_bonus(&mut self, value: i32) {
        log!(format!("actually setting tile bonus to {}", value));
        self.tile_bonus_value = value;
        self.save();
        self.checked_storage = false;
        log!(format!("so now tile bonus is {}", self.tile_bonus_value));

    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
