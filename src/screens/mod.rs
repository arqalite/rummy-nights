//! The front-end part of the app, rendering the individual app screens.

mod game;
mod game_end;
mod menu;
mod player_select;

use crate::data::{Screen, STATE};
use dioxus::prelude::*;

pub fn render_screen(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        div {
            class: "flex flex-col bg-white h-screen w-screen relative overflow-hidden",
            div {
                class: "z-10 flex flex-col relative grow px-8 sm:max-w-lg mx-auto w-full",
                match state.read().screen {
                    Screen::Menu => rsx!(menu::screen()),
                    Screen::PlayerSelect => rsx!(player_select::screen()),
                    Screen::Game => rsx!(game::screen()),
                    Screen::Winner => rsx!(game_end::screen()),
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
                        class: "w-[80vw] h-[80vw] top-[-40vw] left-[-40vw] lg:max-w-[800px] lg:max-h-[800px] lg:top-[-400px] lg:left-[-400px] absolute rounded-full z-0",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    }
                    div {
                        class: "w-[80vw] h-[80vw] bottom-[-40vw] right-[-40vw] lg:max-w-[800px] lg:max-h-[800px] lg:bottom-[-400px] lg:right-[-400px] absolute rounded-full z-0",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    }
                ),
                Screen::PlayerSelect => rsx!(
                    div {
                        class: "w-[100vw] h-[100vw] bottom-[-50vw] left-[-50vw] lg:max-w-[1000px] lg:max-h-[1000px] lg:bottom-[-500px] lg:left-[-500px] absolute rounded-full z-0",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    }
                ),
                Screen::Game => rsx!(
                    div {
                        class: "w-[100vw] h-[100vw] bottom-[-50vw] right-[-50vw] lg:max-w-[1000px] lg:max-h-[1000px] lg:bottom-[-500px] lg:right-[-500px] absolute rounded-full z-0",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    }
                ),
                Screen::Winner => rsx!(
                    div {
                        class: "w-[80vw] h-[80vw] top-[-40vw] left-[-40vw] lg:max-w-[800px] lg:max-h-[800px] lg:top-[-400px] lg:left-[-400px] absolute rounded-full",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    },
                    div {
                        class: "w-[80vw] h-[80vw] bottom-[-40vw] right-[-40vw] lg:max-w-[800px] lg:max-h-[800px] lg:bottom-[-400px] lg:right-[-400px] absolute rounded-full",
                        background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    },
                ),
            }
        }
    ))
}
