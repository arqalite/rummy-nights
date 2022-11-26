use crate::backend::VersionNumber;
use crate::prelude::*;
use dioxus::prelude::*;


pub fn Screen(cx: Scope) -> Element {
    let state = use_context::<Model>(&cx)?;

    log!("Rendering main menu.");
    cx.render(rsx!(
        button {
            class: "absolute right-4 top-4",
            onclick: move |_| state.write().screen = Screen::Settings,
            img {
                class: "h-12",
                src: "img/settings.svg"
            }
        }
        div {
            class : "flex flex-col grow gap-16 justify-center",
            img {
                class: "w-full max-w-lg",
                src: "img/intro.gif",
            }
            div {
                class: "flex flex-col gap-8",
                StartButton {},
                ResumeButton {},
            }
        }
        p {
            class: "text-white font-semibold text-lg text-center w-max max-w-1/2 px-2 absolute bottom-2 left-2 rounded-full",
            background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
            VersionNumber {},
        }
    ))
}

fn StartButton(cx: Scope) -> Element {
    let state = use_context::<Model>(&cx)?;

    log!("Rendering start game button.");
    cx.render(rsx!(
        button {
            class: "grid grid-cols-6 items-center",
            onclick: move |_| state.write().create_game(),
            p {
                class: "w-max font-semibold text-center text-2xl col-span-2 col-start-2 justify-self-end",
                "Start Game"
            }
            img {
                class: "h-20 w-20 col-start-5 col-span-2 rounded-full",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                src: "img/new_game.svg",
            }
        }
    ))
}

fn ResumeButton(cx: Scope) -> Element {
    let state = use_context::<Model>(&cx)?;

    //Hide the resume game button if there is no ongoing game.
    if state.read().game.status != GameStatus::Ongoing {
        return None;
    }

    log!("Rendering resume game button.");
    cx.render(rsx!(
        button {
            class: "grid grid-cols-6 items-center",
            onclick: move |_| state.write().screen = Screen::Game,
            p {
                class: "w-max font-semibold text-center text-2xl col-span-3 col-start-1 justify-self-end",
                "Resume Game"
            }
            img {
                class: "h-20 w-20 col-start-5 col-span-2 rounded-full",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                src: "img/resume_game.svg",
            }
        }
    ))
}
