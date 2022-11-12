use dioxus::fermi::use_atom_ref;
use rummy_nights::prelude::*;
use rummy_nights::screens::render_screen;

pub fn main() {
    log!("Initializing app.");

    dioxus::web::launch(|cx| {
        let state = use_atom_ref(&cx, STATE);

        if !state.read().checked_storage {
            state.write().load_existing_game();
        };

        render_screen(cx)
    });
}
