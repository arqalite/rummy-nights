use gloo_console::log;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use serde::{Deserialize, Serialize};

use crate::backend::prelude::*;
use crate::backend::GameTemplate;
use dioxus::prelude::*;

pub static STATE: fermi::AtomRef<Model> = |_| Model::new();

#[derive(Clone, Serialize, Deserialize)]
pub struct Model {
    pub game: Game,
    pub screen: Screen,
    pub show_end_once: bool,
    pub checked_storage: bool,
    pub settings: Settings,
    pub templates: Vec<GameTemplate>,
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

    pub fn initialize_storage(&mut self) {
        log!("Initializing storage.");
        if !self.checked_storage {
            self.load_existing_game();
            self.settings.load();
            self.load_saved_templates();
        } else {
            log!("Storage already checked this session - skipping.");
        }
    }

    pub fn get_dealer(&self) -> usize {
        let mut position = 0;

        for player in self.game.players.iter() {
            if self.settings.enable_dealer_tracking
                && (((self.game.round + self.game.players.len() + 1) - player.id
                    + self.game.total_rounds)
                    % self.game.players.len()
                    == 0)
                && self.game.status == GameStatus::Ongoing
            {
                position = player.id;
            }
        }

        position
    }

    pub fn add_player(&mut self, name: String, color_index: usize) {
        self.game.add_player(name, color_index);
    }

    pub fn edit_player_name(&mut self, evt: FormEvent, id: usize) {
        let name = evt.values.get("player-name").unwrap().to_string();
        if !name.is_empty() {
            self.game.edit_player_name(id - 1, name);
        };
    }

    pub fn go_to_screen(&mut self, screen: Screen) {
        self.screen = screen
    }

    pub fn clear_and_go_to_menu(&mut self) {
        self.go_to_screen(Screen::Menu);
        self.checked_storage = false;
        SessionStorage::delete("session");
    }

    pub fn toggle_tile_bonus(&mut self) {
        if self.game.tile_bonus_button_active {
            self.game.warn_incorrect_score = false;
            self.game.tile_bonus_button_active = false;
        } else if !self.game.tile_bonus_granted && self.game.status == GameStatus::Ongoing {
            self.game.warn_incorrect_score = false;
            self.game.tile_bonus_button_active = true;
        };
    }

    pub fn finish_game(&mut self) {
        log!("Deleting game and returning to main menu.");
        LocalStorage::delete("state");
        SessionStorage::delete("session");
        *self = Model::new();
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

    pub fn add_score(&mut self, evt: FormEvent, player_id: usize) -> bool {
        log!("Adding score.");

        if let Ok(score) = evt.values.get("score").unwrap().parse::<i32>() {
            if !self.settings.enable_score_checking || (score % 5 == 0) {
                self.game.warn_incorrect_score = false;
                self.game.add_score(player_id, score);
                self.check_status();
                true
            } else {
                self.game.warn_incorrect_score = true;
                false
            }
        } else {
            false
        }
    }

    pub fn edit_score(&mut self, evt: FormEvent) {
        log!(format!("This has {:?}", evt.values));
        if let Ok(score) = evt.values.get("score").unwrap().parse::<i32>() {
            if let Ok(score_id) = evt.values.get("score_id").unwrap().parse::<usize>() {
                if let Ok(player_id) = evt.values.get("player_id").unwrap().parse::<usize>() {
                    if !self.settings.enable_score_checking || (score % 5 == 0) {
                        self.game.warn_incorrect_score = false;

                        for player in &mut self.game.players {
                            if player_id == player.id {
                                *player.score.get_mut(&(score_id - 1)).unwrap() = score;
                                player.sum = player.score.values().sum::<i32>()
                                    + player.bonus.values().sum::<i32>();
                            }
                        }
                        self.game.check_round();
                        self.game.save_game();
                        self.check_status()
                    }  else {
                        self.game.warn_incorrect_score = true;
                    }
                }
            }
        };
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
        self.show_end_once = true;
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
        if self.game.players.len() < 2 || self.templates.len() >= 5 {
            return;
        }

        let mut template_name = String::new();
        template_name.push_str(&(self.templates.len() + 1).to_string());

        self.templates.push(GameTemplate {
            id: self.templates.len() + 1,
            name: template_name,
            players: self.game.players.clone(),
            color: 1,
        });

        self.save_templates();

        log!(format!("Saved templates: {:#?}", self.templates));
    }

    pub fn edit_template(&mut self, evt: FormEvent, color_index: usize) {
        let name = evt.values.get("template-name").unwrap().to_string();
        if !name.is_empty() {
            if let Ok(template_id) = evt.values.get("template_id").unwrap().parse::<usize>() {
                for template in &mut self.templates {
                    if template.id == template_id {
                        template.name = name.clone();
                        template.color = color_index;
                    }
                }
                self.save_templates();
            }
        };
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
            Ok(json_state) => match serde_json::from_value::<Vec<GameTemplate>>(json_state) {
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

    pub fn grant_bonus(&mut self, id: usize) {
        if !self.game.tile_bonus_granted && self.settings.use_tile_bonus {
            log!("Granting player bonus.");

            for player in &mut self.game.players {
                if player.id == id {
                    player
                        .bonus
                        .insert(self.game.round + 1, self.game.tile_bonus_value);
                    player.sum =
                        player.score.values().sum::<i32>() + player.bonus.values().sum::<i32>();
                }
            }
            self.game.tile_bonus_granted = true;
            self.game.new_round_started = false;
            self.game.tile_bonus_button_active = false;
            self.game.save_game();
        }
    }

    pub fn enable_tile_bonus(&mut self, enabled: bool) {
        self.settings.use_tile_bonus = enabled;
        log!(format!("Tile bonus is {:?}", self.settings.use_tile_bonus));
    }

    pub fn set_language(&mut self, language: usize) {
        self.settings.language = language
    }

    pub fn enable_score_editing(&mut self, enabled: bool) {
        self.settings.enable_score_editing = enabled;
        log!(format!(
            "Score editing enabled: {:?}",
            self.settings.enable_score_editing
        ));
    }
    pub fn enable_dealer_tracking(&mut self, enabled: bool) {
        self.settings.enable_dealer_tracking = enabled;
        log!(format!(
            "Dealer tracking enabled: {:?}",
            self.settings.enable_dealer_tracking
        ));
    }
    pub fn enable_max_score(&mut self, enabled: bool) {
        self.settings.end_game_at_score = enabled;
        log!(format!(
            "Max score enabled: {:?}",
            self.settings.end_game_at_score
        ));
    }
    pub fn enable_score_checking(&mut self, enabled: bool) {
        self.settings.enable_score_checking = enabled;
        log!(format!(
            "Score checking enabled: {:?}",
            self.settings.enable_score_checking
        ));
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
