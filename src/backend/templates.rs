use serde::{Serialize, Deserialize};
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub id: usize,
    pub name: String,
    pub players: Vec<Player>,
}