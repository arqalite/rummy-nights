use crate::prelude::*;
use dioxus::prelude::*;

pub fn screen(cx: Scope) -> Element {
    cx.render(rsx!(top_bar(), template_list(),))
}

fn template_list(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        div {
            class: "flex flex-col grow justify-center",
            (state.read().templates.is_empty()).then(|| rsx!(
                span {
                    class: "font-semibold text-lg border-b-2 border-indigo-500 w-max mx-auto mb-8",
                    "No templates saved yet - add some!"
                }
            )),
            state.read().templates.iter().map(|template| {
                let id = template.id;
                let background_color = BG_COLORS[id - 1];

                rsx!(
                    div {
                        class: "flex justify-evenly h-16 rounded-full bg-slate-200",
                        button {
                            class: "flex justify-center h-8 w-3/5 self-center rounded-full {background_color}",
                            p {
                                class: "flex self-center text-white font-semibold",
                                "{template.name}"
                            }
                        }
                        button {
                            onclick: move |_| state.write().load_template(id),
                            div {
                                class: "h-10",
                                assets::play_icon()
                            }
                        }
                        button {
                            onclick: move |_| state.write().delete_template(id),
                            div {
                                class: "h-10",
                                assets::remove()
                            }
                        }
                    },
                )
            })
        }
        div {
            class: "z-20 absolute bottom-4 right-4",
            (state.read().game.players.len() >= 2).then(|| rsx!(
                button {
                    class: "flex flex-row gap-2 h-14 w-max p-2 rounded-full justify-end",
                    onclick: |_| state.write().add_template(),
                    span {
                        class: "font-semibold text-lg self-center",
                        "Add a template"
                    }
                    div {
                        class: "h-10 w-10 self-center",
                        assets::save_icon()
                    }
                }
            ))
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
                onclick: |_| {
                    state.write().screen = Screen::PlayerSelect;
                },
                div {
                    class: "h-10 scale-x-[-1]",
                    assets::back()
                }
            }
        }
    ))
}
