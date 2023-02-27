use dioxus::prelude::*;
use gloo_console::log;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod game;
mod model;
mod settings;

pub mod prelude {
    pub use crate::backend::game::Game;
    pub use crate::backend::model::Model;
    pub use crate::backend::model::STATE;
    pub use crate::backend::settings::Settings;
    pub use crate::backend::GameStatus;
    pub use crate::backend::GameTemplate;
    pub use crate::backend::Player;
    pub use crate::backend::Screen;
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Copy, Debug)]
pub enum GameStatus {
    NotStarted,
    Ongoing,
    Finished,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Copy)]
pub enum Screen {
    Menu,
    PlayerSelect,
    Templates,
    Game,
    EndGame,
    Settings,
    Credits,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GameTemplate {
    pub id: usize,
    pub name: String,
    pub players: Vec<prelude::Player>,
    pub color: usize,
}

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

/// Renders the version number (for releases) or the timestamp
/// (for dev builds).
///
/// It uses the `BUILD_VERSION` environment variable created in build.rs.
pub fn VersionNumber(cx: Scope) -> Element {
    log!("Calculating version number/timestamp.");
    let version = env!("BUILD_VERSION");
    render!("{version}")
}
