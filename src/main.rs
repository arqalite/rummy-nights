//! # Rummy Nights
//! Rummy Nights is a cross-platform rummy score counter app.
//!
//! This is the internal documentation, meant for developers and contributors.

use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;
use rummy_nights::prelude::*;
use rummy_nights::screens::render_screen;

/// The app entry-point function.
/// 
/// It does three things:
/// - Set up the state
/// - Load existing games, if any
/// - Render the front-end
pub fn main() {
    let app: Component = |cx| {
        let state = use_atom_ref(&cx, STATE);
        state.write().load_existing_game();
        render_screen(cx, &state.read().screen)
    };

    dioxus::web::launch(app);
}
