use crate::backend::prelude::*;
use gloo_console::log;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Game {
    pub players: Vec<Player>,
    pub status: GameStatus,
    pub round: usize,
    pub total_rounds: usize,
    pub new_round_started: bool,
    pub tile_bonus_button_active: bool,
    pub tile_bonus_granted: bool,
    pub double_game_button_active: bool,
    pub double_game_granted: bool,
    pub sorted_players: Vec<Player>,
    pub is_sorted: bool,
    pub max_score: i32,
    pub tile_bonus_value: i32,
    pub winner_name: String,
    pub warn_incorrect_score: bool,
}

impl Game {
    pub fn new() -> Self {
        log!("Initializing game.");
        Self {
            players: Vec::new(),
            status: GameStatus::NotStarted,
            round: 0,
            total_rounds: 0,
            new_round_started: true,
            tile_bonus_button_active: false,
            tile_bonus_granted: false,
            double_game_button_active: false,
            double_game_granted: false,
            sorted_players: Vec::new(),
            is_sorted: false,
            max_score: 1000,
            tile_bonus_value: 50,
            winner_name: String::new(),
            warn_incorrect_score: false,
        }
    }

    pub fn add_player(&mut self, name: String, color_index: usize) {
        log!("Adding player.");

        if self.players.len() < 4 && !name.is_empty() {
            let id = self.players.len() + 1;

            self.players.push(Player {
                id,
                name,
                score: BTreeMap::new(),
                sum: 0,
                bonus: BTreeMap::new(),
                list_of_doubled_games: BTreeMap::new(),
                doubles: BTreeMap::new(),
                color_index,
                winner: false,
            });
        };
    }

    pub fn edit_player_name(&mut self, id: usize, name: String) {
        log!("Adding player.");

        if !name.is_empty() {
            self.players[id].name = name;
        };
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

    pub fn change_player_color(&mut self, player_id: usize, color_id: usize) {
        for player in &mut self.players {
            if player_id == player.id {
                player.color_index = color_id - 1;
            }
        }
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
            self.double_game_granted = false;
        } else {
            self.new_round_started = false;
        }

        self.save_game()
    }

    pub fn add_score(&mut self, player_id: usize, value: i32, round: usize) {
        log!("Adding score.");
        for player in &mut self.players {
            if player_id == player.id {
                player.score.insert(player.score.len(), value);
                if player.list_of_doubled_games.contains_key(&(round + 1)) {
                    player.doubles.insert(round + 1, value);
                }
                player.sum = player.score.values().sum::<i32>()
                    + player.bonus.values().sum::<i32>()
                    + player.doubles.values().sum::<i32>();
            }
        }
        self.check_round();
        self.save_game();
    }

    pub fn sort_players(&mut self) {
        log!("Sorting players.");

        self.sorted_players = self.players.clone();
        log!("Getting players worked.");

        self.sorted_players.sort_by(|a, b| {
            let temp_sum_a = a.sum;
            let temp_sum_b = b.sum;

            temp_sum_a.cmp(&temp_sum_b)
        });
        log!("Sorting players worked.");

        self.sorted_players.reverse();
        log!("Reversing players worked.");

        self.sorted_players[0].winner = true;
        self.is_sorted = true;
        log!("Finishing players worked.");
    }

    pub fn check_status(&mut self) {
        log!("Checking game status.");

        for player in &mut self.players {
            log!(format!(
                "Player {} has: score {:?}, bonus {:?}, doubles {:?}, list of doubles: {:?}",
                player.name,
                player.score,
                player.bonus,
                player.doubles,
                player.list_of_doubled_games,
            ));

            for game in player.list_of_doubled_games.keys() {
                log!(format!("trying {game}"));
                if player.score.contains_key(&(game - 1)) {
                    player
                        .doubles
                        .insert(*game, *player.score.get(&(game - 1)).unwrap());
                }
            }

            player.sum = player.score.values().sum::<i32>()
                + player.bonus.values().sum::<i32>()
                + player.doubles.values().sum::<i32>();
        }

        let total_scores: Vec<i32> = self
            .players
            .iter()
            .map(|player| {
                player.score.values().sum::<i32>()
                    + player.bonus.values().sum::<i32>()
                    + player.doubles.values().sum::<i32>()
            })
            .collect();

        let max = *(total_scores.iter().max().unwrap());

        if max >= self.max_score && self.new_round_started {
            let no_of_winners = self
                .players
                .iter()
                .filter(|player| {
                    player.score.values().sum::<i32>()
                        + player.bonus.values().sum::<i32>()
                        + player.doubles.values().sum::<i32>()
                        >= max
                })
                .count();

            if no_of_winners == 1 {
                let winner: Vec<&Player> = self
                    .players
                    .iter()
                    .filter(|player| player.sum >= self.max_score)
                    .collect();
                self.winner_name = winner[0].name.clone();

                self.status = GameStatus::Finished;
                self.save_game();
            }
        }

        log!("Done checking status.")
    }

    pub fn get_winner(&self) -> String {
        self.winner_name.clone()
    }

    pub fn start_game(&mut self) -> bool {
        log!("Starting new game.");

        if self.players.len() >= 2 {
            let mut counter = 1;
            for player in &mut self.players {
                player.id = counter;
                counter += 1;
            }

            LocalStorage::delete("state");
            SessionStorage::delete("session");

            self.status = GameStatus::Ongoing;
            self.save_game();

            true
        } else {
            false
        }
    }
    pub fn save_game(&self) {
        log!("Saving game.");

        LocalStorage::set("game", self.clone()).unwrap();
        SessionStorage::set("session", true).unwrap();
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
