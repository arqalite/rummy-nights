//! Rummy Nights - a rummy score counter app.

use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;
use rummy_nights::prelude::*;
use rummy_nights::screens::render_screen;

/// This is the app entry-point.
/// It initializes the state, loads existing games (if any) from storage and renders the screens.
fn app(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    state.write().load_existing_game();

    render_screen(cx, &state.read().screen)
}

fn main() {
    dioxus::web::launch(app);
}
