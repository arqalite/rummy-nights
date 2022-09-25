//! The intro screen.
//! It should only look nice and serve as a starting point
//! for creating a new game or resuming an existing one.

use dioxus::prelude::*;
use dioxus::fermi::use_atom_state;

use crate::STATE;
use crate::data::GameStatus;
use crate::data::Screen;
use crate::data::read_local_storage;

pub fn intro_screen(cx: Scope) -> Element {
    let state = use_atom_state(&cx, STATE);

    match read_local_storage() {
        Ok(new_state) => {
            state.with_mut(|mut_state| {
                mut_state.players = new_state.players;
                mut_state.game_status = new_state.game_status;
            });
        },
        // It's no big deal if an existing game cannot be read, 
        // we'll just throw an error message in the console and continue.
        // We could inform the user that it couldn't be read, 
        // but there's nothing they could do anyway.
        Err(error) => gloo_console::log!(error)
    };

    cx.render(rsx!(
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
            class: "flex grow flex-col z-10 place-content-evenly",
            div {
                class: "",
                img {
                    class: "mx-auto max-w-sm md:max-w-md mt-32",
                    src: "img/intro.gif",
                }
            }
            div {
                class: "w-full mx-auto relative justify-center content-center",
                menu(),
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
        button {
            class: "grid grid-cols-2 items-center w-full mx-auto my-8",
            onclick: new_game,
            p {
                class: "font-bold text-center text-lg justify-self-end mr-4",
                "Start Game"
            }
            img {
                class: "h-20 w-20 justify-self-start ml-4",
                src: "img/new.svg", 
            }
        },
        is_game_ongoing.then(|| rsx!(
            button {
                class: "grid grid-cols-2 items-center w-full mx-auto",
                onclick: resume_game,
                p {
                    class: "font-bold text-center text-lg justify-self-end mr-4",
                    "Resume Game"
                }
                img {
                    class: "h-20 w-20 justify-self-start ml-4",
                    src: "img/resume.svg", 
                }
            }
        ))
    ))
}