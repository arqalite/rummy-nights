use dioxus::prelude::*;
use dioxus::fermi::use_atom_state;

static MENU_TOGGLE: Atom<bool> = |_| false;

pub fn game_menu(cx: Scope) -> Element {
    let state = use_atom_state(&cx, MENU_TOGGLE);
    let hidden = match *state.get() {
        true => {
            "hidden h-0"
        },
        false => {
            "relative h-60"
        }
    };

    cx.render(
        rsx! (
            
            div {
                class: "grid grid-cols-1 absolute bottom-8 left-8",
                div {
                    class: "{hidden} transition-all duration-150 ease-in-out ml-16 bottom-2  w-60 bg-gradient-to-b from-purple-900 via-pink-700 to-rose-500 shadow-lg rounded-3xl",
                    p {
                        class: "text-center",
                        "Clicked!"
                    },
                }
                crate::game_menu::game_menu_button()
            }
            


        )
    )
}

pub fn game_menu_button(cx: Scope) -> Element {
    let state = use_atom_state(&cx, MENU_TOGGLE);

    cx.render(
        rsx!{
            img {
                src: "img/menu_button.svg",
                class: "",
                onclick: move |_| {
                    state.modify(|value| !value)
                },
            }
        }
    )
}