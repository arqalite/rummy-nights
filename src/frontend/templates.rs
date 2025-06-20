use crate::prelude::*;
use dioxus::events::FormEvent;
use dioxus::prelude::*;

pub fn TemplateScreen() -> Element {
    log!("Rendering template screen.");
    rsx!(TopBar {}, TemplateList {})
}

fn TemplateList() -> Element {
    log!("Rendering template list.");

    let templates = STATE.read().templates.clone();

    rsx!(
        div {
            class: "flex flex-col grow gap-4 px-8 my-16",
            span {
                class: "font-semibold text-lg border-b-2 border-blue-600 w-max mx-auto mb-4",
                {get_text( "template_prompt")}
            }
            if templates.is_empty() {
                div {
                    class: "flex flex-col grow justify-center",
                    p {
                        class: "font-semibold italic text-slate-400 text-lg mx-auto",
                        {get_text( "no_templates_yet")}
                    }
                }
            }
            for template in templates.iter() {
                TemplateItem {
                    template: template.clone(),
                }
            }
        }
        div {
            class: "z-20 absolute bottom-4 right-4",
            if templates.len() < 5 {
                if STATE.read().game.players.len() >= 2 {
                    AddTemplateButton {}
                } else {
                    div {
                        class: "flex flex-row gap-2 h-14 w-max p-2 rounded-full justify-end",
                        span {
                            class: "font-semibold text-lg self-center italic text-slate-700",
                            {get_text("template_not_enough")}
                        }
                    }
                }
            } else {
                div {
                    class: "flex flex-row gap-2 h-14 w-max p-2 rounded-full justify-end",
                    span {
                        class: "font-semibold text-lg self-center",
                        {get_text("template_too_many")}
                    }
                }
            }
        }
    )
}

#[component]
fn TemplateItem(template: GameTemplate) -> Element {
    log!("Rendering template.");

    let id = template.id;
    let background_color = BG_COLORS[template.color];
    let mut show_template_edit = use_signal(|| false);
    let mut buffer = use_signal(|| template.name.clone());
    let mut color_index = use_signal(|| template.color);
    let selected_color = BG_COLORS[color_index()];
    let mut hide_color_bar = use_signal(|| true);

    let oninput = move |evt: FormEvent| buffer.set(evt.value().clone());

    rsx!(
        if !show_template_edit() {
            div {
                class: "flex justify-evenly h-16 rounded-full bg-slate-200",
                button {
                    class: "flex justify-center h-8 w-3/5 self-center rounded-full {background_color}",
                    onclick: move |_| show_template_edit.set(!show_template_edit()),
                    p {
                        class: "flex self-center text-white font-semibold",
                        "{template.name}"
                    }
                }
                button {
                    onclick: move |_| STATE.write().load_template(id),
                    div {
                        class: "h-10",
                        assets::PlayIcon {}
                    }
                }
                button {
                    onclick: move |_| STATE.write().delete_template(id),
                    div {
                        class: "h-10",
                        assets::RemoveIcon {}
                    }
                }
            }
        } else {
            form {
                id: "template_name_input",
                class: "flex flex-row w-full justify-evenly items-center h-16 rounded-full bg-slate-200",
                onsubmit: move |evt| {
                    let name = evt.values().get("template-name").unwrap().join("");
                    if !name.is_empty() {
                        STATE.write().edit_template(evt, color_index());
                        show_template_edit.set(!show_template_edit());
                    }
                },
                input {
                    name: "template-name",
                    class: "rounded-full w-3/5 h-8 ring-1 ring-grey text-center self-center",
                    placeholder: get_text( "name_template"),
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
                    onclick: move |evt| {
                        evt.prevent_default();
                        hide_color_bar.set(!hide_color_bar());
                    },
                    div {
                        class: "h-6 w-6 rounded-full {selected_color} place-self-center"
                    }
                }
            }
            if !hide_color_bar() {
                div {
                    class: "flex flex-row w-full justify-evenly h-10 mt-2 rounded-full bg-slate-200",
                    for (color_id, color) in BG_COLORS.iter().enumerate() {
                        button {
                            id: "{color_id}",
                            class: "h-6 w-6 rounded-full {color} place-self-center",
                            onclick: move |_| color_index.set(color_id-1),
                        }
                    }
                }
            }
        }
    )
}

fn AddTemplateButton() -> Element {
    log!("Rendering add template button.");

    rsx!(
        button {
            class: "flex flex-row gap-2 h-14 w-max",
            onclick: move |_| STATE.write().add_template(),
            span {
                class: "font-semibold text-lg self-center pl-2",
                {get_text( "template_add")}
            }
            div {
                class: "h-10 w-10 self-center rounded-full",
                assets::SaveIcon {},
            }

        }
    )
}

fn TopBar() -> Element {
    log!("Rendering top bar.");

    rsx!(
        div {
            class: "absolute top-0 h-16 grid grid-cols-3 z-10 mx-auto w-full sm:max-w-lg px-8",
            button {
                class: "col-start-1 justify-self-start",
                onclick: move |_| STATE.write().go_to_screen(Screen::PlayerSelect),
                div {
                    class: "h-10 scale-x-[-1]",
                    assets::BackIcon {}
                }
            }
        }
    )
}
