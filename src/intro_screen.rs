//! The intro screen.
//! It should only look nice and serve as a starting point
//! for creating a new game or resuming an existing one.

use dioxus::fermi::use_atom_state;
use dioxus::prelude::*;

use crate::data::read_local_storage;
use crate::data::GameStatus;
use crate::data::Screen;
use crate::STATE;

pub fn intro_screen(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);

    match read_local_storage() {
        Ok(new_state) => {
            state.with_mut(|mut_state| {
                mut_state.players = new_state.players;
                mut_state.game_status = new_state.game_status;
            });
        }
        // It's no big deal if an existing game cannot be read,
        // we'll just throw an error message in the console and continue.
        // We could inform the user that it couldn't be read,
        // but there's nothing they could do anyway.
        Err(error) => gloo_console::log!(error),
    };

    cx.render(rsx!(
        div {
            class: "flex flex-col relative mx-auto h-screen w-screen overflow-hidden",    
            div {
                class: "z-0 absolute h-screen w-screen",
                div {
                    class: "w-[250px] h-[250px] top-[-125px] left-[-125px] absolute rounded-full z-0",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                },
                div {
                    class: "w-[250px] h-[250px] bottom-[-125px] right-[-125px] absolute rounded-full z-0",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                },
            }
            div {
                class: "flex flex-col grow z-10 place-content-evenly mt-32",
                button {
                    class: "absolute top-4 right-4",
                    // TODO: Settings menu.
                    img {
                        class: "w-10 h-10",
                        src: "img/user.svg"
                    }
                },
                img {
                    class: "mx-auto max-w-sm md:max-w-md",
                    src: "img/intro.gif",
                }
                menu(),
                button {
                    class: "flex flex-row items-center justify-center gap-4 w-8/12 h-16 md:w-64 bottom-2 left-2 absolute rounded-full",
                    background: "linear-gradient(225deg, #9EFBD3 0%, #57E9F2 47.87%, #45D4FB 100%)",
                    img {
                        class: "w-10 h-10",
                        src: "img/share.svg",
                    },
                    p {
                        class: "text-white font-semibold text-lg text-center",
                        // TODO: Share functionality.
                        "Share the app!"
                    }
                }
            }
        }
    ))
}

fn menu(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);
    let is_game_ongoing = state.game_status == GameStatus::Ongoing;

    let new_game = |_| {
        state.with_mut(|mut_state| {
            mut_state.players = Vec::new();
            mut_state.screen = Screen::PlayerSelect;
        });
    };

    let resume_game = |_| {
        state.with_mut(|mut_state| {
            mut_state.screen = Screen::Game;
        });
    };

    cx.render(rsx!(
        div {
            class: "flex flex-col gap-y-8 max-w-md mx-auto relative mb-16",
            button {
                class: "grid grid-cols-6 items-center w-full mx-auto",
                onclick: new_game,
                p {
                    class: "font-semibold text-center text-2xl col-span-2 col-start-2 justify-self-end",
                    "Start Game"
                }
                img {
                    class: "h-24 w-24 col-start-5 col-span-2",
                    src: "img/new.svg", 
                }
            },
            is_game_ongoing.then(|| rsx!(
                button {
                    class: "grid grid-cols-6 items-center w-full mx-auto",
                    onclick: resume_game,
                    p {
                        class: "font-semibold text-center text-2xl col-span-3 col-start-1 justify-self-end",
                        "Resume Game"
                    }
                    img {
                        class: "h-24 w-24 col-start-5 col-span-2",
                        src: "img/resume.svg", 
                    }
                }
            ))
        }
    ))
}
