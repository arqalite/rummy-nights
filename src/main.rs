#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)] // Happy Clippy, happy life
#![allow(clippy::used_underscore_binding)] // Clippy doesn't understand Dioxus's Props macro tho

use dioxus::prelude::*;
use std::collections::BTreeMap;

// mod game_menu;
mod score_table;
mod css;

static PLAYERS: Atom<Vec<Player>> = |_| {
    vec![
        Player {
            id: 1,
            name: "Antonio".to_string(),
            score: BTreeMap::new(),
        },
        Player {
            id: 2,
            name: "Vlad".to_string(),
            score: BTreeMap::new(),
        },
        // Player {
        //     id: 3,
        //     name: "Dalmina".to_string(),
        //     score: BTreeMap::new(),
        // },
        // Player {
        //     id: 4,
        //     name: "Daniel".to_string(),
        //     score: BTreeMap::new(),
        // },
    ]
};

#[derive(PartialEq, Clone)]
struct Player {
    id: usize,
    name: String,
    score: BTreeMap<usize, i32>,
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        score_table::score_table(),
        // game_menu::menu()
    ))
}

fn main() {
    dioxus::web::launch(app);
}
