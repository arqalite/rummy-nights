//! The front-end part of the app, rendering the individual app screens.

mod game;
mod game_end;
mod menu;
mod player_select;
mod settings;

use crate::prelude::*;
use dioxus::prelude::*;

pub fn render_screen(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        div {
            class: "flex flex-col bg-white h-screen w-screen relative overflow-hidden",
            div {
                class: "z-10 flex flex-col grow px-8 mx-auto w-full sm:max-w-lg",
                match state.read().screen {
                    Screen::Menu => rsx!(menu::screen()),
                    Screen::PlayerSelect => rsx!(player_select::screen()),
                    Screen::Game => rsx!(game::screen()),
                    Screen::EndGame => rsx!(game_end::screen()),
                    Screen::Settings => rsx!(settings::screen()),
                },
            }
            decorative_spheres()
        }
    ))
}

fn decorative_spheres(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
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
                Screen::Settings => rsx!(
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

pub static BG_COLORS: [&str; 4] = [
    "bg-rose-400",
    "bg-cyan-400",
    "bg-green-400",
    "bg-violet-400",
];

pub static BORDER_COLORS: [&str; 4] = [
    "border-rose-400",
    "border-cyan-400",
    "border-green-400",
    "border-violet-400",
];

pub static CARET_COLORS: [&str; 4] = [
    "caret-rose-400",
    "caret-cyan-400",
    "caret-green-400",
    "caret-violet-400",
];
