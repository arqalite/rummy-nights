use crate::data::print_version_number;
use crate::prelude::*;
use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;

pub fn screen(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    log!("Rendering main menu.");

    cx.render(rsx!(
        div {
            class : "flex flex-col grow gap-8 justify-center",
            img {
                class: "w-full max-w-lg",
                src: "img/intro.gif",
            }
            div {
                class: "flex flex-col gap-8",
                start_game_button()
                (state.read().game_status == GameStatus::Ongoing).then(|| resume_game_button(cx)),
            }
        }
        p {
            class: "text-white font-semibold text-lg text-center w-max max-w-1/2 px-2 absolute bottom-2 left-2 rounded-full",
            background: "linear-gradient(225deg, #9EFBD3 0%, #57E9F2 47.87%, #45D4FB 100%)",
            print_version_number()
        }
    ))
}

fn start_game_button(cx: Scope) -> Element {
    log!("Rendering start game button.");

    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        button {
            class: "grid grid-cols-6 items-center",
            onclick: |_| state.write().create_game(),
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
    log!("Rendering resume game button.");

    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        button {
            class: "grid grid-cols-6 items-center",
            onclick: |_| state.write().screen = Screen::Game,
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
