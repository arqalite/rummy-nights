use dioxus::fermi::use_atom_ref;
use rummy_nights::prelude::*;
use rummy_nights::screens::render_screen;

pub fn main() {
    log!("Initializing app.");

    dioxus::web::launch(|cx| {
        let state = use_atom_ref(&cx, STATE);
        let settings = use_atom_ref(&cx, SETTINGS);

        if !settings.read().checked_storage {
            settings.write().load();
        }

        if !state.read().checked_storage {
            state.write().load_existing_game();
            state.write().settings = settings.read().clone();
        };

        render_screen(cx)
    });
}
