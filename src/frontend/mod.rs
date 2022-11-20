//! The front-end part of the app, rendering the individual app screens.

mod credits;
mod game;
mod game_end;
mod menu;
mod player_select;
mod settings;
mod templates;

use crate::prelude::*;
use dioxus::prelude::*;

pub fn render_screen(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    log!("Start render.");
    cx.render(rsx!(
        div {
            class: "flex flex-col bg-white h-screen w-screen relative overflow-hidden",
            div {
                class: "z-10 flex flex-col h-screen px-8 mx-auto w-full sm:max-w-lg",
                match state.read().screen {
                    Screen::Menu => rsx!(menu::screen()),
                    Screen::PlayerSelect => rsx!(player_select::screen()),
                    Screen::Templates => rsx!(templates::screen()),
                    Screen::Game => rsx!(game::screen()),
                    Screen::EndGame => rsx!(game_end::screen()),
                    Screen::Settings => rsx!(settings::screen()),
                    Screen::Credits => rsx!(credits::screen()),
                },
            }
            decorative_spheres()
        }
    ))
}

fn decorative_spheres(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    log!("Rendering decorations.");
    cx.render(rsx!(
        div {
            class: "z-0 absolute h-screen w-screen",
            match state.read().screen {
                Screen::Menu => rsx!(
                    div {
                        class: "w-[50vw] h-[50vw] top-[-25vw] left-[-25vw] absolute rounded-full z-0",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    }
                    div {
                        class: "w-[50vw] h-[50vw] bottom-[-25vw] right-[-25vw] absolute rounded-full z-0",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    }
                ),
                Screen::PlayerSelect => rsx!(
                    div {
                        class: "w-[50vw] h-[50vw] bottom-[-25vw] right-[-25vw] absolute rounded-full z-0",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    }
                    div {
                        class: "w-[50vw] h-[50vw] bottom-[-25vw] left-[-25vw] absolute rounded-full z-0",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    }
                ),
                Screen::Templates => rsx!(
                    div {
                        class: "w-[50vw] h-[50vw] top-[-25vw] right-[-25vw] absolute rounded-full z-0",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    }
                    div {
                        class: "w-[50vw] h-[50vw] bottom-[-25vw] left-[-25vw] absolute rounded-full z-0",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    }
                ),
                Screen::Game => rsx!(
                    div {
                        class: "w-[50vw] h-[50vw] bottom-[-25vw] right-[-25vw] absolute rounded-full z-0",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    }
                ),
                Screen::EndGame => rsx!(
                    div {
                        class: "w-[50vw] h-[50vw] top-[-25vw] left-[-25vw] absolute rounded-full",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    },
                    div {
                        class: "w-[50vw] h-[50vw] bottom-[-25vw] right-[-25vw] absolute rounded-full",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    },
                ),
                Screen::Settings | Screen::Credits => rsx!(
                    div {
                        class: "w-[50vw] h-[50vw] top-[-25vw] left-[-25vw] absolute rounded-full",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    },
                    div {
                        class: "w-[50vw] h-[50vw] top-[-25vw] right-[-25vw] absolute rounded-full",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    },
                    div {
                        class: "w-[50vw] h-[50vw] bottom-[-25vw] left-[-25vw] absolute rounded-full",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    },
                    div {
                        class: "w-[50vw] h-[50vw] bottom-[-25vw] right-[-25vw] absolute rounded-full",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    },
                ),
            }
        }
    ))
}

pub static BG_COLORS: [&str; 7] = [
    "bg-red-500",
    "bg-orange-500",
    "bg-yellow-500",
    "bg-green-500",
    "bg-blue-500",
    "bg-indigo-500",
    "bg-violet-500",
];

pub static BORDER_COLORS: [&str; 7] = [
    "border-red-500",
    "border-orange-500",
    "border-yellow-500",
    "border-green-500",
    "border-blue-500",
    "border-indigo-500",
    "border-violet-500",
];

pub static CARET_COLORS: [&str; 7] = [
    "caret-red-500",
    "caret-orange-500",
    "caret-yellow-500",
    "caret-green-500",
    "caret-blue-500",
    "caret-indigo-500",
    "caret-violet-500",
];
