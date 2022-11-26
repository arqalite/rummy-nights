use dioxus::prelude::*;
use rummy_nights::prelude::*;

pub fn main() {
    log!("Initializing app.");

    dioxus_web::launch(|cx| {
        use_context_provider(&cx, Model::new);
        let model = use_context::<Model>(&cx)?;

        log!("Loaded new state.");

        if !model.read().checked_storage {
            model.write().load_existing_game();
            model.write().settings.load();
            model.write().load_saved_templates();
            log!("Finish loading data.");
        };

        cx.render(rsx!(RenderScreen {}))
    });
}
