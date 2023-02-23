use crate::prelude::*;
use dioxus::events::FormEvent;
use dioxus::prelude::*;

pub fn screen(cx: Scope) -> Element {
    cx.render(rsx!(TopBar {}, TemplateList {},))
}

fn TemplateList(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    let no_templates_yet = get_text(state.read().settings.language, "no_templates_yet").unwrap();
    let template_add = get_text(state.read().settings.language, "template_add").unwrap();
    let name_this_template = get_text(state.read().settings.language, "name_template").unwrap();
    let template_not_enough =
        get_text(state.read().settings.language, "template_not_enough").unwrap();
    let template_too_many = get_text(state.read().settings.language, "template_too_many").unwrap();

    let hide_color_bar = use_state(&cx, || true);
    let color_index = use_state(&cx, || 0);
    let selected_color = BG_COLORS[**color_index];
    let mut color_id = 0;
    let hidden = if **hide_color_bar { "hidden" } else { "" };

    cx.render(rsx!(
        div {
            class: "flex flex-col grow justify-center gap-4 px-8",
            (state.read().templates.is_empty()).then(|| rsx!(
                p {
                    class: "font-semibold text-lg border-b-2 border-indigo-500 mx-auto",
                    "{no_templates_yet}!"
                }
            )),
            state.read().templates.iter().map(|template| {
                let id = template.id;
                let color = template.color;
                let background_color = BG_COLORS[color];
                let show_template_edit = use_state(&cx, || false);
                let buffer = use_state(&cx, || template.name.clone());

                let onsubmit = move |evt: FormEvent| {
                    let name = evt.values.get("template-name").unwrap().to_string();
                    if !name.is_empty() {
                        if let Ok(template_id) = evt.values.get("template_id").unwrap().parse::<usize>() {
                            state.write().edit_template(template_id, name, **color_index);
                            show_template_edit.set(!show_template_edit);
                        }
                    };
                };

                let oninput = move |evt: FormEvent| {
                    buffer.set(evt.value.clone())
                };

                rsx!(
                    (!show_template_edit).then(|| rsx!(
                        div {
                            class: "flex justify-evenly h-16 rounded-full bg-slate-200",
                            button {
                                class: "flex justify-center h-8 w-3/5 self-center rounded-full {background_color}",
                                onclick: move |_| show_template_edit.set(!show_template_edit),
                                p {
                                    class: "flex self-center text-white font-semibold",
                                    "{template.name}"
                                }
                            }
                            button {
                                onclick: move |_| state.write().load_template(id),
                                div {
                                    class: "h-10",
                                    assets::PlayIcon {}
                                }
                            }
                            button {
                                onclick: move |_| state.write().delete_template(id),
                                div {
                                    class: "h-10",
                                    assets::RemoveIcon {}
                                }
                            }
                        }
                    )),
                    (show_template_edit).then(|| rsx!(
                        form {
                            id: "template_name_input",
                            class: "flex flex-row w-full justify-evenly items-center h-16 rounded-full bg-slate-200",
                            prevent_default: "onsubmit",
                            onsubmit: onsubmit,
                            input {
                                name: "template-name",
                                class: "rounded-full w-3/5 h-8 ring-1 ring-grey text-center self-center",
                                placeholder: "{name_this_template}",
                                oninput: oninput,
                                value: "{buffer}"
                            }
                            input {
                                name: "template_id",
                                r#type: "hidden",
                                value: "{id}",
                            }
                            button {
                                r#type: "submit",
                                class: "h-10",
                                assets::AddIcon {},
                            }
                            button {
                                class: "flex flex-col justify-center h-16 w-8",
                                prevent_default: "onclick",
                                onclick: move |_| hide_color_bar.set(!hide_color_bar),
                                div {
                                    class: "h-6 w-6 rounded-full {selected_color} place-self-center"
                                }
                            }
                        }
                        div {
                            class: "{hidden} flex flex-row w-full justify-evenly h-10 mt-2 rounded-full bg-slate-200",
                            BG_COLORS.iter().map(|color| {
                                color_id += 1;
                                rsx!(
                                    button {
                                        id: "{color_id}",
                                        class: "h-6 w-6 rounded-full {color} place-self-center",
                                        onclick: move |_| color_index.set(color_id-1),
                                    }
                                )
                            })
                        }
                    ))
                )
            })
        }
        div {
            class: "z-20 absolute bottom-4 right-4",
            if state.read().templates.len() < 6 {
                rsx!(
                    (state.read().game.players.len() >= 2).then(|| rsx!(
                        button {
                            class: "flex flex-row gap-2 h-14 w-max p-2 rounded-full justify-end",
                            onclick: move |_| state.write().add_template(),
                            span {
                                class: "font-semibold text-lg self-center",
                                "{template_add}"
                            },
                            div {
                                class: "h-10 w-10 self-center",
                                assets::SaveIcon {}
                            }
                        }
                    )),
                    (state.read().game.players.len() < 2).then(|| rsx!(
                        div {
                            class: "flex flex-row gap-2 h-14 w-max p-2 rounded-full justify-end",
                            span {
                                class: "font-semibold text-lg self-center",
                                "{template_not_enough}"
                            }
                        }
                    ))
                )
            } else {
                rsx!(
                    div {
                        class: "flex flex-row gap-2 h-14 w-max p-2 rounded-full justify-end",
                        span {
                            class: "font-semibold text-lg self-center",
                            "{template_too_many}"
                        }
                    }
                )
            }
        }
    ))
}

fn TopBar(cx: Scope) -> Element {
    log!("Rendering top bar.");

    let state = fermi::use_atom_ref(cx, STATE);

    cx.render(rsx!(
        div {
            class: "absolute top-0 h-16 grid grid-cols-3 z-10 mx-auto w-full sm:max-w-lg px-8",
            button {
                class: "col-start-1 justify-self-start",
                onclick: move |_| {
                    state.write().screen = Screen::PlayerSelect;
                },
                div {
                    class: "h-10 scale-x-[-1]",
                    assets::BackIcon {}
                }
            }
        }
    ))
}
