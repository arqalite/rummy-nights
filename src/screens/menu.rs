use crate::prelude::*;
use crate::data::print_version_number;

use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, SessionStorage, Storage};

pub fn screen(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "flex relative h-screen overflow-hidden",
            decorative_spheres(),

            button {
                class: "absolute top-4 right-4",
                img {
                    class: "w-10 h-10",
                    src: "img/user.svg"
                }
            }

            div {
                class : "z-10 flex flex-col grow self-center my-16",
                img {
                    class: "mx-auto w-full max-w-lg mb-8",
                    src: "img/intro.gif",
                }
                div { 
                    class: "flex flex-col gap-y-8 mx-auto relative",
                    start_game_button()
                    resume_game_button()
                }
            }

            p {
                class: "text-white font-semibold text-lg text-center w-max max-w-1/2 px-2 absolute bottom-2 left-2 rounded-full",
                background: "linear-gradient(225deg, #9EFBD3 0%, #57E9F2 47.87%, #45D4FB 100%)",
                print_version_number()
            }
        }
    ))
}

fn start_game_button(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    let start_new_game = |_| {
        LocalStorage::clear();
        SessionStorage::clear();

        *state.write() = Model::new();
        state.write().screen = Screen::PlayerSelect;
    };

    cx.render(rsx!(
        button {
            class: "grid grid-cols-6 items-center w-full mx-auto",
            onclick: start_new_game,
            p {
                class: "font-semibold text-center text-2xl col-span-2 col-start-2 justify-self-end",
                "Start Game"
            }
            img {
                class: "h-20 w-20 col-start-5 col-span-2",
                src: "img/new.svg",
            }
        }
    ))
}

fn resume_game_button(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    if state.read().game_status == GameStatus::Ongoing {
        cx.render(rsx!(
            button {
                class: "grid grid-cols-6 items-center w-full mx-auto",
                onclick: |_| { state.write().screen = Screen::Game },
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
    } else {
        None
    }
}

fn decorative_spheres(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "z-0 absolute h-screen w-screen",
            div {
                class: "w-[80vw] h-[80vw] top-[-40vw] left-[-40vw] absolute rounded-full z-0",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
            }
            div {
                class: "w-[80vw] h-[80vw] bottom-[-40vw] right-[-40vw] absolute rounded-full z-0",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
            }
        }
    ))
}
