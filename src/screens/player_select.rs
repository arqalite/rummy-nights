use dioxus::events::FormEvent;
use dioxus::fermi::use_atom_ref;
use dioxus::prelude::*;
use dioxus::web::use_eval;

use crate::data::tailwind_classes;
use crate::prelude::*;

pub fn screen(cx: Scope) -> Element {
    log!("Rendering player select.");

    cx.render(rsx!(
        div {
            class: "flex flex-col grow h-screen w-screen relative overflow-hidden px-[5%]",
            decorative_spheres()
            top_bar()

            div {
                class: "z-10 flex flex-col grow relative mx-auto w-full sm:max-w-lg",
                div {
                    class: "mb-6 w-max mx-auto",
                    span {
                        class: "font-semibold text-lg border-b-2 border-emerald-300",
                        "Add up to 4 players"
                    }
                }
                player_list()
                start_game_button()
            }
        }
    ))
}

fn start_game_button(cx: Scope) -> Element {
    log!("Rendering begin game button.");

    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        button {
            class: "z-10 flex absolute self-end w-max gap-2 border-b-[6px] border-emerald-300 right-0 bottom-32",
            onclick: |_| state.write().start_game(),
            span {
                class: "flex self-center text-xl font-bold w-max",
                "Start game"
            }
            img {
                class: "h-12 w-12",
                src: "img/arrow.svg"
            }
        }
    ))
}

fn player_list(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    log!("Rendering player list.");

    cx.render(rsx!(
        div {
            class: "flex flex-col gap-6",
            state.read().players.iter().map(|player| {
                log!("Rendering player.");

                let background_color = tailwind_classes::BG_COLORS[player.id-1];
                let id = player.id;

                rsx!(
                    div {
                        class: "flex justify-evenly h-16 rounded-full bg-slate-200 pr-2",
                        div {
                            class: "flex justify-center	content-center h-8 w-3/5 self-center rounded-full {background_color}",
                            p {
                                class: "flex self-center text-white font-semibold",
                                "{player.name}"
                            }
                        }
                        button {
                            onclick: move |_| state.write().remove_player(id),
                            img {
                                class: "h-8",
                                src: "img/remove.svg",
                            }
                        }
                        div {
                            class: "flex flex-col gap-1 justify-center",
                            button {
                                onclick: move |_| state.write().move_up(id),
                                img {
                                    class: "h-5",
                                    src: "img/up.svg"
                                },
                            }
                            button {
                                onclick: move |_| state.write().move_down(id),
                                img {
                                    class: "h-5 rotate-180",
                                    src: "img/up.svg"
                                },
                            }
                        }
                    }
                )
            }),

            (state.read().players.len() <= 3).then(|| player_input(cx)),
        }
    ))
}

fn player_input(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let execute = use_eval(&cx);

    let onsubmit = move |evt: FormEvent| {
        let player_name = evt.values.get("player-name").unwrap();

        if !player_name.is_empty() {
            state.write().add_player(player_name.to_string());
            execute("document.getElementById('name_input').reset();".to_string());
        };
    };

    cx.render(rsx!(
        form {
            id: "name_input",
            class: "flex flex-row w-full justify-evenly h-16 rounded-full bg-slate-200 pr-2",
            prevent_default: "onsubmit",
            onsubmit: onsubmit,
            input {
                name: "player-name",
                class: "rounded-full w-3/5 h-8 ring-1 ring-grey text-center self-center",
                placeholder: "Insert player name",
            }
            button {
                r#type: "submit",
                img {
                    class: "h-8",
                    src: "img/add-player.svg",
                }
            }
            button {
                div {
                    class: "h-5 w-5 rounded-full bg-emerald-400"
                }
            }
        }
    ))
}

fn top_bar(cx: Scope) -> Element {
    log!("Rendering top bar.");

    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        div {
            class: "h-16 grid grid-cols-3 z-10 mx-auto w-full sm:max-w-lg",
            button {
                class: "col-start-1 justify-self-start",
                onclick: |_| state.write().screen = Screen::Menu,
                img {
                    class: "h-8 w-8",
                    src: "img/back.svg",
                }
            }
            button {
                class: "col-start-3 justify-self-end",
                //onclick:
                img {
                    class: "h-8 w-8",
                    src: "img/save.svg",
                }
            }
        }
    ))
}

fn decorative_spheres(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "z-0 absolute h-screen w-screen",
            div {
                class: "w-[100vw] h-[100vw] bottom-[-50vw] left-[-50vw] absolute rounded-full z-0",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
            }
        }
    ))
}
