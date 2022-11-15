use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub score: BTreeMap<usize, i32>,
    pub sum: i32,
    pub bonus: BTreeMap<usize, i32>,
}

impl Player {
    pub fn add_score(&mut self, value: i32) {
        self.score.insert(self.score.len(), value);
        self.sum = self.score.values().sum::<i32>() + self.bonus.values().sum::<i32>();
    }

    pub fn grant_bonus(&mut self, round: usize, value: i32) {
        self.bonus.insert(round, value);
        self.sum = self.score.values().sum::<i32>() + self.bonus.values().sum::<i32>();
    }
}
