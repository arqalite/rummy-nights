//! Rummy Nights, a rummy score counter written with Rust/Dioxus and Tailwind CSS.
//! This is the app entry-point - should be kept as minimal as possible.

// Make Clippy annoying so the code looks and works somewhat fine.
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::used_underscore_binding, // Underscore bindings are allowed because it flags an issue in Dioxus.
    clippy::use_self, // This is bugged and flags structs for no reason.
    clippy::derive_partial_eq_without_eq // It interferes with Dioxus's inline props feature.
)]

use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;
use rummy_nights::prelude::*;
use rummy_nights::{load_existing_game, render_screen};

// Two things are done here:
// Setting up the state and screens,
// and checking for (and loading) saved games from storage.
fn app(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let screen = state.read().screen.clone();

    load_existing_game(cx);
    render_screen(cx, screen)
}

fn main() {
    dioxus::web::launch(app);
}
