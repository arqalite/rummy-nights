use crate::prelude::*;
use dioxus::events::FormEvent;
use dioxus::prelude::*;
use dioxus::web::use_eval;
use gloo_console::log;

pub fn screen(cx: Scope) -> Element {
    log!("Rendering settings menu.");

    cx.render(rsx!(top_bar(), settings_menu()))
}

fn max_score_setting(cx: Scope) -> Element {
    let settings = use_atom_ref(&cx, SETTINGS);
    let score = settings.read().max_score;
    let execute = use_eval(&cx);

    let changed = use_state(&cx, || false);

    let is_button_hidden = if **changed {
        String::from("")
    } else {
        String::from("hidden")
    };

    let onsubmit = move |evt: FormEvent| {
        let max_score_text = evt.values.get("max_score").unwrap();
        log!(format!("Max score text is {}", max_score_text));

        let max_score = max_score_text.parse::<i32>().unwrap_or(1000);
        log!(format!("Final max score text is {}", max_score));

        if max_score > 0 {
            settings.write().max_score = max_score;
            settings.write().save();
            settings.write().checked_storage = false;
            changed.set(false);
        };
    };

    cx.render(rsx!(
        div {
            class: "flex flex-row gap-8 h-16 justify-center items-center",
            span {
                class: "text-center font-semibold text-lg",
                "Maximum score:"
            }
            form {
                class: "flex flex-row w-1/2 justify-evenly",
                onsubmit: onsubmit,
                prevent_default: "onsubmit",
                input {
                    name: "max_score",
                    class: "text-lg appearance-none font-light bg-transparent h-10 w-3/4 text-center rounded focus:border-b-[8px] border-b-4 border-green-400",
                    id: "max_score",
                    style: "-moz-appearance:textfield",
                    outline: "none",
                    r#type: "number",
                    value: "{score}",
                    oninput: move |evt: FormEvent| if !changed {
                        changed.set(true);
                        log!(format!("Do this bitch have value? {}", evt.value));
                        execute(format!("document.getElementById('max_score').value = '{}';", evt.value));
                    },
                }
                div {
                    class: "w-10 h-10",
                    button {
                        class: "{is_button_hidden}",
                        r#type: "submit",
                        img {
                            class: "h-10",
                            src: "img/add.svg",
                        }
                    }
                }
            }
        }
    ))
}

fn settings_menu(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "flex flex-col grow justify-evenly",
            div {
                class: "flex flex-col gap-8 justify-evenly",
                max_score_setting(),
            }
        }
    ))
}

fn _switch(cx: Scope) -> Element {
    let enabled = use_state(&cx, || false);

    cx.render(rsx!(
        enabled.then(|| rsx!(
            p {
                "Text"
            }
        )),
        div {
            class: "flex flex-col justify-center h-screen w-screen",
            label {
                class: "inline-flex relative items-center cursor-pointer",
                input {
                    r#type: "checkbox",
                    id: "default-toggle",
                    class: "sr-only peer",
                    onchange: move |_| enabled.set(!enabled),
                }
                div {
                    class: "w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[url('/img/purple_gradient.svg')]"
                }
            }
        }
    ))
}

fn top_bar(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let settings = use_atom_ref(&cx, SETTINGS);

    cx.render(rsx!(
        button {
            class: "absolute top-4 left-4",
            onclick: |_| {
                state.write().settings = settings.read().clone();
                state.write().screen = Screen::Menu;
            },
            img {
                class: "h-12 scale-x-[-1]",
                src: "img/back.svg",
            }
        },
        button {
            class: "absolute top-4 right-4",
            onclick: |_| {
                //state.write().screen = Screen::Menu;
            },
            img {
                class: "h-12",
                src: "img/info.svg",
            }
        }
    ))
}
