#![allow(non_snake_case)]
use dioxus::prelude::*;
use rummy_nights::prelude::*;

pub fn App() -> Element {
    
    let screen = STATE.read().screen;

    if !(STATE.read().checked_storage) {
        STATE.write().initialize_storage();
        //STATE.write()._debug_game_screen();
    };

    log!("Start render.");
    rsx!(
        div {
            class: "flex flex-col bg-white h-screen w-screen relative overflow-hidden",
            rummy_nights::frontend::DecorativeSpheres {},
            div {
                class: "z-10 flex flex-col h-screen mx-auto w-full sm:max-w-lg",
                match screen {
                    Screen::Menu => rsx!(rummy_nights::frontend::menu::MenuScreen {}),
                    Screen::PlayerSelect => rsx!(rummy_nights::frontend::player_select::PlayerSelectScreen {}),
                    Screen::Templates => rsx!(rummy_nights::frontend::templates::TemplateScreen {}),
                    Screen::Game => rsx!(rummy_nights::frontend::game::GameScreen {}),
                    Screen::EndGame => rsx!(rummy_nights::frontend::game_end::EndScreen {}),
                    Screen::Settings => rsx!(rummy_nights::frontend::settings::SettingsScreen {}),
                    Screen::Credits => rsx!(rummy_nights::frontend::credits::CreditsScreen {}),
                }
            }
        }
    )
}

pub fn main() {
    log!("Initializing app.");

    dioxus::launch(App);
}
