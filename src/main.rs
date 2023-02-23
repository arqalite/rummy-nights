use rummy_nights::prelude::*;

pub fn main() {
    log!("Initializing app.");

    dioxus_web::launch(|cx| {
        fermi::use_init_atom_root(&cx);
        let state = fermi::use_atom_ref(cx, STATE);

        log!("Loaded new state.");

        if !state.read().checked_storage {
            state.write().load_existing_game();
            state.write().settings.load();
            state.write().load_saved_templates();
            log!("Finish loading data.");
        };

        render_screen(cx)
    });
}
