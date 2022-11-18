use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_console::log;

mod game;
mod model;
mod player;
mod settings;

pub mod prelude {
    pub use crate::backend::game::Game;
    pub use crate::backend::model::Model;
    pub use crate::backend::model::STATE;
    pub use crate::backend::player::Player;
    pub use crate::backend::settings::Settings;
    pub use crate::backend::GameStatus;
    pub use crate::backend::Screen;
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum GameStatus {
    NotStarted,
    Ongoing,
    Finished,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Screen {
    Menu,
    PlayerSelect,
    Game,
    EndGame,
    Settings,
    Credits,
}

/// Renders the version number (for releases) or the timestamp
/// (for dev builds).
///
/// It uses the `BUILD_VERSION` environment variable created in build.rs.
pub fn print_version_number(cx: Scope) -> Element {
    log!("Calculating version number.");
    let version = env!("BUILD_VERSION");
    cx.render(rsx!("{version}"))
}
