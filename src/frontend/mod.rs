//! The front-end part of the app, rendering the individual app screens.

pub mod assets;
pub mod credits;
pub mod game;
pub mod game_end;
pub mod menu;
pub mod player_select;
pub mod settings;
pub mod templates;

use crate::prelude::*;
use dioxus::prelude::*;
use phf::phf_map;

pub fn TopLeftSphere(cx: Scope) -> Element {
    render!(div {
        class: "w-[50vw] h-[50vw] top-[-25vw] left-[-25vw] absolute rounded-full z-0",
        background:
            "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
    })
}

pub fn TopRightSphere(cx: Scope) -> Element {
    render!(div {
        class: "w-[50vw] h-[50vw] top-[-25vw] right-[-25vw] absolute rounded-full z-0",
        background:
            "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
    })
}

pub fn BottomLeftSphere(cx: Scope) -> Element {
    render!(div {
        class: "w-[50vw] h-[50vw] bottom-[-25vw] left-[-25vw] absolute rounded-full z-0",
        background:
            "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
    })
}

pub fn BottomRightSphere(cx: Scope) -> Element {
    render!(div {
        class: "w-[50vw] h-[50vw] bottom-[-25vw] right-[-25vw] absolute rounded-full z-0",
        background:
            "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
    })
}

pub fn DecorativeSpheres(cx: Scope) -> Element {
    log!("Rendering decorations.");

    let state = fermi::use_atom_ref(cx, STATE);
    let screen = state.read().screen;
    render!(
        div {
            class: "z-0 absolute h-screen w-screen",
            match screen {
                Screen::Menu |  Screen::EndGame => rsx!(
                    TopLeftSphere {}
                    BottomRightSphere {}
                ),
                Screen::PlayerSelect => rsx!(
                    BottomRightSphere {}
                    BottomLeftSphere {}
                ),
                Screen::Templates => rsx!(
                    TopRightSphere {}
                    BottomLeftSphere {}
                ),
                Screen::Game => rsx!(
                    BottomRightSphere {}
                ),
                Screen::Settings | Screen::Credits => rsx!(
                    TopLeftSphere {}
                    TopRightSphere {}
                    BottomLeftSphere {}
                    BottomRightSphere {}
                ),
            }
        }
    )
}

pub static BG_COLORS: [&str; 7] = [
    "bg-red-600",
    "bg-orange-500",
    "bg-yellow-400",
    "bg-green-500",
    "bg-blue-600",
    "bg-violet-600",
    "bg-pink-500",
];

pub static BORDER_COLORS: [&str; 7] = [
    "border-red-600",
    "border-orange-500",
    "border-yellow-400",
    "border-green-500",
    "border-blue-600",
    "border-violet-600",
    "border-pink-500",
];

pub static CARET_COLORS: [&str; 7] = [
    "caret-red-600",
    "caret-orange-500",
    "caret-yellow-400",
    "caret-green-500",
    "caret-blue-600",
    "caret-violet-600",
    "caret-pink-500",
];

pub static ENGLISH: phf::Map<&'static str, &'static str> = phf_map! {
    "start_game" => "New game",
    "resume_game" => "Resume game",
    "tile_bonus" => "Tile bonus",
    "restart" => "Restart app",
    "clear_data" => "Clear data",
    "language" => "Language",
    "score_editing" => "Allow score editing",
    "dealer_tracking" => "Dealer tracking",
    "max_score" => "Maximum score:",
    "end_at_max_score" => "End game at maximum score",
    "tile_bonus_value" => "Tile bonus value:",
    "programmer" => "Programming:",
    "design" => "UI/UX Design:",
    "icons" => "Icons:",
    "tech" => "Tech:",
    "love" => "Made with ❤️ in Romania.",
    "start_game_button" => "Start game",
    "insert_player" => "Insert player name",
    "add_players" => "Add up to 4 players",
    "banner_win" => "won!",
    "banner_bonus" => "Who gets the bonus?",
    "banner_play" => "Good luck and have fun!",
    "winner_label" => "THE WINNER IS",
    "no_templates_yet" => "No templates saved yet - add some!",
    "template_add" => "Save current players",
    "name_template" => "Name this template",
    "template_not_enough" => "Add some players first!",
    "template_too_many" => "Only 5 templates are allowed!",
    "template_prompt" => "Add up to 5 templates",
    "score_checking" => "Check scores"
};

pub static ROMANIAN: phf::Map<&'static str, &'static str> = phf_map! {
    "start_game" => "Joc nou",
    "resume_game" => "Reluați jocul",
    "tile_bonus" => "Atu",
    "restart" => "Reporniți",
    "clear_data" => "Ștergeți datele",
    "language" => "Limbă",
    "score_editing" => "Permiteți editarea scorurilor",
    "dealer_tracking" => "Urmărire dealer",
    "max_score" => "Scorul maxim:",
    "end_at_max_score" => "Limită de scor",
    "tile_bonus_value" => "Valoarea atuului:",
    "programmer" => "Programator:",
    "design" => "Design UI/UX:",
    "icons" => "Pictograme:",
    "tech" => "Tehnologii:",
    "love" => "Creat cu ❤️ în România.",
    "start_game_button" => "Începe jocul",
    "insert_player" => "Introdu un nume",
    "add_players" => "Adaugă până la 4 jucători",
    "banner_win" => "a câștigat!",
    "banner_bonus" => "Cine primește atuuul?",
    "banner_play" => "Cel mai bun să câștige!",
    "winner_label" => "CÂȘTIGĂTORUL ESTE",
    "no_templates_yet" => "Niciun șablon salvat - adaugă câteva!",
    "template_add" => "Salvați jucătorii actuali",
    "name_template" => "Numește acest șablon",
    "template_not_enough" => "Adaugă mai întâi niște jucători!",
    "template_too_many" => "Poți avea doar 5 șabloane!",
    "template_prompt" => "Adaugă până la 5 șabloane",
    "score_checking" => "Verifică scorurile adăugate"
};

pub fn get_text<'a>(cx: &ScopeState, text_key: &'a str) -> &'a str {
    match fermi::use_atom_ref(cx, STATE).read().settings.language {
        2 => ROMANIAN.get(text_key).cloned().unwrap(),
        _ => ENGLISH.get(text_key).cloned().unwrap(),
    }
}
