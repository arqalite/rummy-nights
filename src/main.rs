use dioxus::prelude::*;
use rummy_nights::prelude::*;

pub fn main() {
    log!("Initializing app.");

    dioxus::web::launch(|cx| {
        let state = use_atom_ref(&cx, STATE);

        if !state.read().checked_storage {
            state.write().load_existing_game();
            state.write().settings.load();
        };

        render_screen(cx)
    });
}
