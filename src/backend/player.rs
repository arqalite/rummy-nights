use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub score: BTreeMap<usize, i32>,
    pub sum: i32,
    pub bonus: BTreeMap<usize, i32>,
    pub color_index: usize,
    pub winner: bool,
}

impl Player {
    pub fn add_score(&mut self, value: i32) {
        self.score.insert(self.score.len(), value);
        self.sum = self.score.values().sum::<i32>() + self.bonus.values().sum::<i32>();
    }

    pub fn edit_score(&mut self, score_id: usize, value: i32) {
        let score = self.score.get_mut(&(score_id - 1)).unwrap();
        *score = value;

        self.sum = self.score.values().sum::<i32>() + self.bonus.values().sum::<i32>();
    }

    pub fn grant_bonus(&mut self, round: usize, value: i32) {
        self.bonus.insert(round, value);
        self.sum = self.score.values().sum::<i32>() + self.bonus.values().sum::<i32>();
    }

    pub fn change_color(&mut self, color_id: usize) {
        self.color_index = color_id - 1;
    }
}
