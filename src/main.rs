//! Rummy Nights, a rummy score counter written with Rust/Dioxus and Tailwind CSS.
//!
//! This module is the app entry-point, mostly setting up the model/state and rendering the initial screens.
//! It should be as basic as possible, with the majority of logic being written in separate modules.

// Make Clippy annoying so the code looks and works somewhat fine.
// It doesn't understand Dioxus' quirks though, so the warning for underscore bindings stays disabled.
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)] 
#![allow(clippy::used_underscore_binding)] 

// The code is split into multiple modules:
//      css.rs holds Tailwind CSS classes so we can change colors programmatically.
//      data.rs holds the custom data structures/types.
//      The rest deal with each app screen individually.

mod data;
mod intro_screen;
mod player_select;
mod score_table;
mod winner_screen;

use dioxus::prelude::*;
use dioxus::fermi::use_atom_state;
use data::{Model, Screen};

// As detailed in data.rs, going for a MVC-style Model structure makes things nicer.
static STATE: Atom<Model> = |_| Model {
    players: Vec::new(),
    game_status: data::GameStatus::NotStarted,
    screen: data::Screen::Intro,
};


// This is the actual entry-point, and it should be kept as simple as possible.
// For now just managing the various screens is enough.
// Other work should be done in its respective modules.
fn app(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);

    let screen = match state.screen {
        Screen::Intro => rsx!(intro_screen::intro()),
        Screen::PlayerSelect => rsx!(player_select::player_select()),
        Screen::Game => rsx!(score_table::score_table()),
        Screen::Winner => rsx!(winner_screen::winner_screen())
    };

    cx.render(rsx!(
        div {
            // For now we design for mobile, 
            // so we're restricting the max-width on desktop to match how a phone would look.
            class: "mx-auto max-w-md h-screen overflow-hidden",
            screen,
        }
    ))
}

fn main() {
    dioxus::web::launch(app);
}
