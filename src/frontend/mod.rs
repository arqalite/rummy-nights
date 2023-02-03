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
