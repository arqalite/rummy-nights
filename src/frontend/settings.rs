use crate::prelude::*;
use dioxus::events::FormEvent;
use dioxus::prelude::*;
use dioxus::web::use_eval;
use gloo_console::log;

pub fn screen(cx: Scope) -> Element {
    log!("Rendering settings menu.");

    cx.render(rsx!(top_bar(), settings_menu()))
}

fn settings_menu(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "flex flex-col grow justify-evenly",
            div {
                class: "flex flex-col divide-y divide-slate-200 justify-evenly border-y border-slate-200",
                div {
                    class: "flex flex-col gap-4",
                    dealer_enable(),
                },
                div {
                    class: "flex flex-col gap-4",
                    tile_bonus_enable(),
                    tile_bonus_value_setting(),
                },
                div {
                    class: "flex flex-col gap-4",
                    max_score_enable(),
                    max_score_setting(),
                }
            }
        }
    ))
}

fn dealer_enable(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let enabled = use_state(&cx, || state.read().settings.enable_dealer_tracking);

    cx.render(rsx!(
        div {
            class: "grid grid-cols-6 gap-4 h-16 pt-4",
            span {
                class: "col-span-5 justify-self-start font-semibold text-lg",
                "Dealer tracking"
            }
            label {
                class: "inline-flex relative cursor-pointer justify-self-end",
                input {
                    r#type: "checkbox",
                    id: "default-toggle",
                    class: "sr-only peer",
                    checked: "{enabled}",
                    onchange: move |_| {
                        enabled.set(!enabled);
                        state.write().settings.enable_dealer_tracking = *enabled.current();
                        log!(format!("Dealer enabled: {:?}", state.read().settings.enable_dealer_tracking));
                    }
                }
                div {
                    class: "w-11 h-6 bg-gray-200 rounded-full peer peer-focus:outline-none peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[url('/img/purple_gradient.svg')]"
                }
            }
        }
    ))
}

fn max_score_setting(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    if !state.read().settings.end_game_at_score {
        return None;
    }

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
            class: "grid grid-cols-2 gap-4 h-16",
            span {
                class: "col-span-1 justify-self-end font-semibold text-lg",
                "Maximum score:"
            }
            form {
                class: "flex flex-row w-full justify-evenly",
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
                    class: "w-10 h-10 flex justify-center items-center",
                    button {
                        class: "{is_button_hidden}",
                        r#type: "submit",
                        img {
                            class: "h-6",
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
        return None;
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
            class: "grid grid-cols-2 gap-4 h-16",
            span {
                class: "col-span-1 justify-self-end font-semibold text-lg",
                "Tile bonus value:"
            }
            form {
                class: "flex flex-row w-full justify-evenly",
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
                    class: "w-10 h-10 flex justify-center items-center",
                    button {
                        class: "{is_button_hidden}",
                        r#type: "submit",
                        img {
                            class: "h-6",
                            src: "img/add.svg",
                        }
                    }
                }
            }
        }
    ))
}

fn tile_bonus_enable(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let enabled = use_state(&cx, || state.read().settings.use_tile_bonus);

    cx.render(rsx!(
        div {
            class: "grid grid-cols-2 gap-4 h-16 pt-4",
            span {
                class: "col-span-1 justify-self-start font-semibold text-lg",
                "Tile bonus"
            }
            label {
                class: "inline-flex relative cursor-pointer justify-self-end",
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

fn max_score_enable(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let enabled = use_state(&cx, || state.read().settings.end_game_at_score);

    cx.render(rsx!(
        div {
            class: "grid grid-cols-6 gap-4 h-16 pt-4",
            span {
                class: "col-span-5 justify-self-start font-semibold text-lg",
                "End game at maximum score"
            }
            label {
                class: "inline-flex relative cursor-pointer justify-self-end",
                input {
                    r#type: "checkbox",
                    id: "default-toggle",
                    class: "sr-only peer",
                    checked: "{enabled}",
                    onchange: move |_| {
                        enabled.set(!enabled);
                        state.write().settings.end_game_at_score = *enabled.current();
                        log!(format!("Max score enabled: {:?}", state.read().settings.end_game_at_score));
                    }
                }
                div {
                    class: "w-11 h-6 bg-gray-200 rounded-full peer peer-focus:outline-none peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[url('/img/purple_gradient.svg')]"
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
                state.write().settings.save();
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
