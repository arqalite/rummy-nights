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
    let state = use_atom_ref(&cx, STATE);
    let max_score = use_state(&cx, || state.read().settings.max_score);
    let changed = use_state(&cx, || false);

    let is_button_hidden = if **changed {
        String::from("")
    } else {
        String::from("hidden")
    };

    let onsubmit = move |evt: FormEvent| {
        let max_score = evt
            .values
            .get("max_score")
            .unwrap()
            .parse::<i32>()
            .unwrap_or(1000);

        log!(format!("Input value is {}", max_score));

        if max_score > 0 {
            log!(format!("setting score to {}", max_score));
            state.write().settings.set_max_score(max_score);
            changed.set(false);
            use_eval(&cx)(format!(
                "document.getElementById('max_score').value = '{}';",
                max_score
            ));
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
                    value: "{max_score}",
                    oninput: move |evt: FormEvent| {
                        changed.set(true);
                        max_score.set(evt.value.parse::<i32>().unwrap_or(0));
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

fn tile_bonus_value_setting(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    if !state.read().settings.use_tile_bonus {
        return None
    }

    let tile_bonus = use_state(&cx, || state.read().settings.tile_bonus_value);
    let changed = use_state(&cx, || false);

    let is_button_hidden = if **changed {
        String::from("")
    } else {
        String::from("hidden")
    };

    let onsubmit = move |evt: FormEvent| {
        let tile_bonus = evt
            .values
            .get("tile_bonus")
            .unwrap()
            .parse::<i32>()
            .unwrap_or(50);

        log!(format!("Input tile bonus value is {}", tile_bonus));

        if tile_bonus > 0 {
            log!(format!("setting tile bonus to {}", tile_bonus));
            state.write().settings.set_tile_bonus(tile_bonus);
            changed.set(false);
            use_eval(&cx)(format!(
                "document.getElementById('max_score').value = '{}';",
                tile_bonus
            ));
        };
    };

    cx.render(rsx!(
        div {
            class: "flex flex-row gap-8 h-16 justify-center items-center",
            span {
                class: "text-center font-semibold text-lg",
                "Tile bonus value:"
            }
            form {
                class: "flex flex-row w-1/2 justify-evenly",
                onsubmit: onsubmit,
                prevent_default: "onsubmit",
                input {
                    name: "tile_bonus",
                    class: "text-lg appearance-none font-light bg-transparent h-10 w-3/4 text-center rounded focus:border-b-[8px] border-b-4 border-rose-400",
                    id: "tile_bonus",
                    style: "-moz-appearance:textfield",
                    outline: "none",
                    r#type: "number",
                    value: "{tile_bonus}",
                    oninput: move |evt: FormEvent| {
                        changed.set(true);
                        tile_bonus.set(evt.value.parse::<i32>().unwrap_or(0));
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
                tile_bonus_enable(),
                tile_bonus_value_setting(),
            }
        }
    ))
}

fn tile_bonus_enable(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let enabled = use_state(&cx, || state.read().settings.use_tile_bonus);

    cx.render(rsx!(
        div {
            class: "flex flex-row gap-8 h-16 justify-center items-center",
            span {
                class: "text-center font-semibold text-lg",
                "Use tile bonus"
            }
            label {
                class: "inline-flex relative items-center cursor-pointer",
                input {
                    r#type: "checkbox",
                    id: "default-toggle",
                    class: "sr-only peer",
                    checked: "{enabled}",
                    onchange: move |_| {
                        enabled.set(!enabled);
                        state.write().settings.use_tile_bonus = *enabled.current();
                        log!(format!("Tile bonus is {:?}", state.read().settings.use_tile_bonus));
                    }
                }
                div {
                    class: "w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[url('/img/purple_gradient.svg')]"
                }
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

    cx.render(rsx!(
        button {
            class: "absolute top-4 left-4",
            onclick: |_| {
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
