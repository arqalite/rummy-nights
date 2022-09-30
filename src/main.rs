//! Rummy Nights, a rummy score counter written with Rust/Dioxus and Tailwind CSS.
//!
//! This module is the app entry-point, mostly setting up the model/state and rendering the initial screens.
//! It should be as basic as possible, with the majority of logic being written in separate modules.

// Make Clippy annoying so the code looks and works somewhat fine.
// It doesn't understand Dioxus' quirks though, so the warning for underscore bindings stays disabled.
// The "use_self" one I have no idea what it means, and it popped up randomly with no explanation.
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::used_underscore_binding, clippy::use_self)]

// The code is split into multiple modules:
//      data.rs holds the custom data structures/types, and arrays of CSS classes.
//      The rest deal with each app screen individually.

mod data;
mod intro_screen;
mod player_select;
mod score_table;
mod winner_screen;

use data::{Model, Screen, STATE};
use fermi::use_atom_state;
use dioxus::prelude::*;

// Here we should just show the individual screens depending on the state.
// Other work should be done in other modules.
fn app(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);

    match state.screen {
        Screen::Intro => cx.render(rsx!(intro_screen::intro_screen(cx))),
        Screen::PlayerSelect => cx.render(rsx!(player_select::player_select(cx))),
        Screen::Game => cx.render(rsx!(score_table::score_table(cx))),
        Screen::Winner => cx.render(rsx!(winner_screen::winner_screen(cx))),
    }
}

fn main() {
    dioxus_web::launch(app);
}
