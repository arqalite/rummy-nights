use gloo_console::log;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use serde::{Deserialize, Serialize};

use crate::backend::prelude::*;
use crate::backend::templates::Template;

pub static STATE: fermi::AtomRef<Model> = |_| Model::new();

#[derive(Clone, Serialize, Deserialize)]
pub struct Model {
    pub game: Game,
    pub screen: Screen,
    pub show_end_once: bool,
    pub checked_storage: bool,
    pub settings: Settings,
    pub templates: Vec<Template>,
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
            templates: Vec::new(),
        }
    }

    pub fn create_game(&mut self) {
        log!("Creating new game.");

        let settings = self.settings;
        let templates = self.templates.clone();
        log!(format!("Backed up settings are {settings:?}"));

        *self = Model::new();
        self.settings = settings;
        self.game.tile_bonus_value = self.settings.tile_bonus_value;
        self.game.max_score = self.settings.max_score;
        self.templates = templates;

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

    pub fn add_score(&mut self, player_id: usize, value: i32) {
        log!("Adding score.");
        self.game.add_score(player_id, value);
        self.check_status()
    }

    pub fn edit_score(&mut self, player_id: usize, score_id: usize, value: i32) {
        log!("Editing score.");
        self.game.edit_score(player_id, score_id, value);
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

    pub fn add_template(&mut self) {
        if self.game.players.len() < 2 {
            return;
        }

        let mut template_name = String::new();
        template_name.push_str(&(self.templates.len() + 1).to_string());

        self.templates.push(Template {
            id: self.templates.len() + 1,
            name: template_name,
            players: self.game.players.clone(),
            color: 1,
        });

        self.save_templates();

        log!(format!("Saved templates: {:#?}", self.templates));
    }

    pub fn edit_template(&mut self, id: usize, name: String, color_index: usize) {
        for template in &mut self.templates {
            if template.id == id {
                template.name = name.clone();
                template.color = color_index;
            }
        }
        self.save_templates();
    }

    pub fn load_template(&mut self, id: usize) {
        for template in &self.templates {
            if template.id == id {
                self.game.players = template.players.clone();
                self.screen = Screen::PlayerSelect;
            }
        }
    }

    pub fn delete_template(&mut self, id: usize) {
        self.templates.retain(|template| template.id != id);
        self.save_templates();
    }

    pub fn save_templates(&mut self) {
        LocalStorage::set("templates", self.templates.clone()).unwrap();
    }

    pub fn load_saved_templates(&mut self) {
        log!("Trying to load templates.");

        match LocalStorage::get::<serde_json::Value>("templates") {
            Ok(json_state) => match serde_json::from_value::<Vec<Template>>(json_state) {
                Ok(saved_templates) => {
                    log!(format!("Loaded: {saved_templates:#?}"));
                    self.templates = saved_templates;
                    log!(format!("Live is: {:#?}", self.templates));
                }
                Err(_) => log!("Could not parse templates."),
            },
            Err(_) => log!("Could not read templates."),
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
