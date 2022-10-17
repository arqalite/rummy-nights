//! Rummy Nights, a rummy score counter written with Rust/Dioxus and Tailwind CSS.
//! This is the entry-point, should be kept pretty minimal, just managing the global state and various screens of the app.

// Make Clippy annoying so the code looks and works somewhat fine.
// It doesn't understand Dioxus' quirks though, so the warning for underscore bindings stays disabled.
// The "use_self" one I have no idea what it means, and it popped up randomly with no explanation.
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::used_underscore_binding, clippy::use_self)]

// The code is split into multiple modules:
//      data.rs holds the custom data structures/types, and arrays of CSS classes.
//      The rest deal with each app screen individually.

mod data;
mod menu;
mod player_select;
mod score_table;
mod winner_screen;

use data::read_local_storage;
use data::{read_session_storage, GameStatus, Model, Screen, STATE};
use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;

// Two things are done here, setting up the state and screens,
// and checking for LocalStorage to see if an ongoing game exists (and loading it into memory).
// Other work should be done in other modules.
fn app(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let has_checked_storage = use_state(&cx, || false);

    if !has_checked_storage {
        match read_local_storage() {
            Ok(new_state) => {
                state.write().players = new_state.players;
                state.write().game_status = new_state.game_status;

                if read_session_storage().is_ok() && state.read().game_status == GameStatus::Ongoing
                {
                    state.write().screen = Screen::Game;
                };

                has_checked_storage.set(true);
            }
            // It's no big deal if an existing game cannot be read,
            // we'll just throw an error message in the console and continue.
            // We could inform the user that it couldn't be read,
            // but there's nothing they could do anyway.
            Err(_) => {
                has_checked_storage.set(true);
            }
        };
    };

    match state.read().screen {
        Screen::Menu => cx.render(rsx!(menu::intro_screen())),
        Screen::PlayerSelect => cx.render(rsx!(player_select::player_select())),
        Screen::Game => cx.render(rsx!(score_table::score_table())),
        Screen::Winner => cx.render(rsx!(winner_screen::winner_screen())),
    }
}

fn main() {
    dioxus::web::launch(app);
}
