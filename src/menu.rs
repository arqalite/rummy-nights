//! The intro screen.
//! It should only look nice and serve as a starting point
//! for creating a new game or resuming an existing one.

use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;

use crate::data::GameStatus;
use crate::data::Screen;
use crate::STATE;

pub fn intro_screen(cx: Scope) -> Element {
    cx.render(rsx!(
        div { //Screen container
            class: "flex relative h-screen overflow-hidden",

            div { // Decorative spheres
                class: "z-0 absolute h-screen w-screen",
                div {
                    class: "w-[300px] h-[300px] top-[-150px] left-[-150px] absolute rounded-full z-0",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                }
                div {
                    class: "w-[300px] h-[300px] bottom-[-150px] right-[-150px] absolute rounded-full z-0",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                }
            }

            button { // Settings button - fixed to top-left corner
                class: "absolute top-4 right-4",
                // TODO: Settings menu.
                img {
                    class: "w-10 h-10",
                    src: "img/user.svg"
                }
            }

            div { // Logo and menu area
                class : "z-10 flex flex-col grow self-center my-16",
                img {
                    class: "mx-auto w-full max-w-lg mb-8",
                    src: "img/intro.gif",
                }
                menu()
                p {
                    class: "text-white font-semibold text-lg text-center max-w-1/2 px-2 absolute bottom-4 left-4 rounded-full",
                    background: "linear-gradient(225deg, #9EFBD3 0%, #57E9F2 47.87%, #45D4FB 100%)",
                    "build 2022-10-17"
                }
            }
        }
    ))
}

fn menu(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let is_game_ongoing = state.write().game_status == GameStatus::Ongoing;

    let new_game = |_| {
        state.write().players = Vec::new();
        state.write().screen = Screen::PlayerSelect;
    };

    let resume_game = |_| {
        state.write().screen = Screen::Game;
    };

    cx.render(rsx!(
        div {
            class: "flex flex-col gap-y-8 mx-auto relative",
            button { //New game button
                class: "grid grid-cols-6 items-center w-full mx-auto",
                onclick: new_game,
                p {
                    class: "font-semibold text-center text-2xl col-span-2 col-start-2 justify-self-end",
                    "Start Game"
                }
                img {
                    class: "h-20 w-20 col-start-5 col-span-2",
                    src: "img/new.svg", 
                }
            },
            is_game_ongoing.then(|| rsx!(
                //Resume game button - shown only if an existing game is found
                button {
                    class: "grid grid-cols-6 items-center w-full mx-auto",
                    onclick: resume_game,
                    p {
                        class: "font-semibold text-center text-2xl col-span-3 col-start-1 justify-self-end",
                        "Resume Game"
                    }
                    img {
                        class: "h-20 w-20 col-start-5 col-span-2",
                        src: "img/resume.svg", 
                    }
                }
            ))
        }
    ))
}
