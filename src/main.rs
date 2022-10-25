//! Rummy Nights, a rummy score counter written with Rust/Dioxus and Tailwind CSS.
//! This is the app entry-point, should be kept pretty minimal, just managing the global state and various screens of the app.

// Make Clippy annoying so the code looks and works somewhat fine.
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::used_underscore_binding, // Underscore bindings are allowed because it flags an issue in Dioxus.
    clippy::use_self, // This is bugged and flags structs for no reason.
    clippy::derive_partial_eq_without_eq // It interferes with Dioxus's inline props feature.
)]

use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;

use rummy_nights::load_existing_game;
use rummy_nights::prelude::*;

// Two things are done here, setting up the state and screens,
// and checking for LocalStorage to see if an ongoing game exists (and loading it into memory).
// Other work should be done in other modules.
fn app(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    load_existing_game(cx);

    match state.read().screen {
        Screen::Menu => render_menu_screen(cx),
        Screen::PlayerSelect => render_player_select_screen(cx),
        Screen::Game => render_game_screen(cx),
        Screen::Winner => render_game_end_screen(cx),
    }
}

fn main() {
    dioxus::web::launch(app);
}
