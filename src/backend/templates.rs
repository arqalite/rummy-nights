use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Template {
    pub id: usize,
    pub name: String,
    pub players: Vec<Player>,
    pub color: usize,
}
