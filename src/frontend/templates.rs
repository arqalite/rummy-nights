use crate::prelude::*;
use dioxus::events::FormEvent;
use dioxus::prelude::*;
use fermi::use_atom_ref;

pub fn TemplateScreen(cx: Scope) -> Element {
    log!("Rendering template screen.");
    render!(TopBar {}, TemplateList {})
}

fn TemplateList(cx: Scope) -> Element {
    log!("Rendering template list.");

    let state = fermi::use_atom_ref(cx, STATE);
    let templates = state.read().templates.clone();

    render!(
        div {
            class: "flex flex-col grow gap-4 px-8 my-16",
            span {
                class: "font-semibold text-lg border-b-2 border-blue-600 w-max mx-auto mb-4",
                get_text(cx, "template_prompt")
            }
            (templates.is_empty()).then(|| rsx!(
                div {
                    class: "flex flex-col grow justify-center",
                    p {
                        class: "font-semibold italic text-slate-400 text-lg mx-auto",
                        get_text(cx, "no_templates_yet")
                    }
                }
            )),
            templates.iter().map(|template| {
                rsx!(
                    TemplateItem {
                        template: template.clone(),
                    }
                )
            })
        }
        div {
            class: "z-20 absolute bottom-4 right-4",
            if templates.len() < 5 {
                rsx!(
                    (state.read().game.players.len() >= 2).then(|| rsx!(
                        AddTemplateButton {}
                    )),
                    (state.read().game.players.len() < 2).then(|| rsx!(
                        div {
                            class: "flex flex-row gap-2 h-14 w-max p-2 rounded-full justify-end",
                            span {
                                class: "font-semibold text-lg self-center italic text-slate-700",
                                get_text(cx, "template_not_enough")
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
                            get_text(cx, "template_too_many")
                        }
                    }
                )
            }
        }
    )
}

#[inline_props]
fn TemplateItem(cx: Scope, template: GameTemplate) -> Element {
    log!("Rendering template.");
    let state = use_atom_ref(cx, STATE);

    let id = template.id;
    let background_color = BG_COLORS[template.color];
    let show_template_edit = use_state(cx, || false);
    let buffer = use_state(cx, || template.name.clone());
    let color_index = use_state(cx, || template.color);
    let selected_color = BG_COLORS[**color_index];
    let hide_color_bar = use_state(cx, || true);
    let mut color_id = 0;

    let oninput = move |evt: FormEvent| buffer.set(evt.value.clone());

    render!(
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
                onsubmit: move |evt| {
                    let name = evt.values.get("template-name").unwrap().to_string();
                    if !name.is_empty() {
                        state.write().edit_template(evt, **color_index);
                        show_template_edit.set(!show_template_edit);
                    }
                },
                input {
                    name: "template-name",
                    class: "rounded-full w-3/5 h-8 ring-1 ring-grey text-center self-center",
                    placeholder: get_text(cx, "name_template"),
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
            (!hide_color_bar).then(|| rsx!(
                div {
                    class: "flex flex-row w-full justify-evenly h-10 mt-2 rounded-full bg-slate-200",
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
        ))
    )
}

fn AddTemplateButton(cx: Scope) -> Element {
    log!("Rendering add template button.");
    let state = fermi::use_atom_ref(cx, STATE);

    render!(
        button {
            class: "flex flex-row gap-2 h-14 w-max",
            onclick: move |_| state.write().add_template(),
            span {
                class: "font-semibold text-lg self-center pl-2",
                get_text(cx, "template_add")
            }
            div {
                class: "h-10 w-10 self-center rounded-full",
                assets::SaveIcon {},
            }
            
        }
    )
}

fn TopBar(cx: Scope) -> Element {
    log!("Rendering top bar.");
    let state = fermi::use_atom_ref(cx, STATE);

    render!(
        div {
            class: "absolute top-0 h-16 grid grid-cols-3 z-10 mx-auto w-full sm:max-w-lg px-8",
            button {
                class: "col-start-1 justify-self-start",
                onclick: move |_| state.write().go_to_screen(Screen::PlayerSelect),
                div {
                    class: "h-10 scale-x-[-1]",
                    assets::BackIcon {}
                }
            }
        }
    )
}
