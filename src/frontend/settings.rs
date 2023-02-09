use crate::prelude::*;
use dioxus::events::FormEvent;
use dioxus::prelude::*;
use dioxus::web::use_eval;
use gloo_console::log;
use gloo_storage::{LocalStorage, SessionStorage, Storage};

pub fn screen(cx: Scope) -> Element {
    log!("Rendering settings menu.");

    cx.render(rsx!(top_bar(), settings_menu(), reset_restart_buttons()))
}

fn reset_restart_buttons(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    let restart_app = move |_| {
        SessionStorage::clear();
        use_eval(&cx)("location.reload()");
    };
    let clear_data = move |_| {
        LocalStorage::clear();
        SessionStorage::clear();
        use_eval(&cx)("location.reload()");
    };

    let restart_label = get_text(state.read().settings.language, "restart").unwrap();
    let clear_data_label = get_text(state.read().settings.language, "clear_data").unwrap();

    cx.render(rsx!(
        div {
            class: "flex flex-col absolute bottom-4 w-2/3 gap-4 h-max justify-center place-content-center place-self-center",
            button {
                class: "flex flex-row gap-2 h-10 items-center w-full place-self-center justify-center",
                onclick: restart_app,
                div {
                    class: "h-8",
                    assets::replay_icon()
                }
                span {
                    class: "font-semibold text-lg leading-8 h-8",
                    "{restart_label}"
                }
            }
            button {
                class: "flex flex-row gap-2 h-8  items-center w-full place-self-center justify-center",
                onclick: clear_data,
                div {
                    class: "h-8",
                    assets::bin()
                }
                span {
                    class: "font-semibold text-lg leading-8 h-8",
                    "{clear_data_label}"
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
                class: "flex flex-col divide-y divide-slate-200 justify-evenly border-y border-slate-200",
                div {
                    class: "flex flex-col gap-4",
                    edit_enable(),
                },
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
                },
                div {
                    class: "flex flex-col gap-4",
                    language_select(),
                },
            }
        }
    ))
}

fn language_select(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    let language_label = get_text(state.read().settings.language, "language").unwrap();

    let mut ro_enabled = "";
    let mut en_enabled = "";

    match state.read().settings.language {
        2 => {
            ro_enabled = "outline";
        }
        _ => {
            en_enabled = "outline";
        }
    }

    cx.render(rsx!(
        div {
            class: "grid grid-cols-6 gap-4 h-16 pt-4",
            span {
                class: "col-span-4 justify-self-start font-semibold text-lg",
                "{language_label}"
            },
            button {
                class: "h-8 w-max {ro_enabled} outline-2 outline-offset-4 outline-[#ee609c]",
                onclick: |_| state.write().settings.language = 2,
                assets::romanian_flag_icon(),
            },
            button {
                class: "h-8 w-max {en_enabled} outline-2 outline-offset-4 outline-[#ee609c]",
                onclick: |_| state.write().settings.language = 1,
                assets::gb_flag_icon(),
            }

        }
    ))
}

fn edit_enable(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let enabled = use_state(&cx, || state.read().settings.enable_score_editing);

    let score_edit_label = get_text(state.read().settings.language, "score_editing").unwrap();

    cx.render(rsx!(
        div {
            class: "grid grid-cols-6 gap-4 h-16 pt-4",
            span {
                class: "col-span-5 justify-self-start font-semibold text-lg",
                "{score_edit_label}"
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
                        state.write().settings.enable_score_editing = *enabled.current();
                        log!(format!("Score editing enabled: {:?}", state.read().settings.enable_score_editing));
                    }
                }
                div {
                    class: "w-11 h-6 bg-gray-200 rounded-full peer peer-focus:outline-none peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#ee609c]"
                }
            }
        }
    ))
}

fn dealer_enable(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let enabled = use_state(&cx, || state.read().settings.enable_dealer_tracking);

    let dealer_label = get_text(state.read().settings.language, "dealer_tracking").unwrap();

    cx.render(rsx!(
        div {
            class: "grid grid-cols-6 gap-4 h-16 pt-4",
            span {
                class: "col-span-5 justify-self-start font-semibold text-lg",
                "{dealer_label}"
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
                    class: "w-11 h-6 bg-gray-200 rounded-full peer peer-focus:outline-none peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#ee609c]"
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
    let max_score_label = get_text(state.read().settings.language, "max_score").unwrap();

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

        log!(format!("Input value is {max_score}"));

        if max_score > 0 {
            log!(format!("setting score to {max_score}"));
            state.write().settings.set_max_score(max_score);
            changed.set(false);
            use_eval(&cx)(format!(
                "document.getElementById('max_score').value = '{max_score}';"
            ));
        };
    };

    cx.render(rsx!(
        div {
            class: "grid grid-cols-2 gap-4 h-16",
            span {
                class: "col-span-1 justify-self-end font-semibold text-lg",
                "{max_score_label}:"
            }
            form {
                class: "flex flex-row w-full justify-evenly",
                onsubmit: onsubmit,
                prevent_default: "onsubmit",
                input {
                    name: "max_score",
                    class: "text-lg appearance-none font-light bg-transparent h-10 w-3/4 text-center rounded focus:border-b-[8px] border-b-4 border-[#ee609c]",
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
                        class: "h-6 {is_button_hidden}",
                        r#type: "submit",
                        assets::add_button(),
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
    let tile_bonus_label = get_text(state.read().settings.language, "tile_bonus_value").unwrap();

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

        log!(format!("Input tile bonus value is {tile_bonus}"));

        if tile_bonus > 0 {
            log!(format!("setting tile bonus to {tile_bonus}"));
            state.write().settings.set_tile_bonus(tile_bonus);
            changed.set(false);
            use_eval(&cx)(format!(
                "document.getElementById('max_score').value = '{tile_bonus}';"
            ));
        };
    };

    cx.render(rsx!(
        div {
            class: "grid grid-cols-2 gap-4 h-16",
            span {
                class: "col-span-1 justify-self-end font-semibold text-lg",
                "{tile_bonus_label}:"
            }
            form {
                class: "flex flex-row w-full justify-evenly",
                onsubmit: onsubmit,
                prevent_default: "onsubmit",
                input {
                    name: "tile_bonus",
                    class: "text-lg appearance-none font-light bg-transparent h-10 w-3/4 text-center rounded focus:border-b-[8px] border-b-4 border-[#ee609c]",
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
                        class: "h-6 {is_button_hidden}",
                        r#type: "submit",
                        assets::add_button(),
                    }
                }
            }
        }
    ))
}

fn tile_bonus_enable(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let enabled = use_state(&cx, || state.read().settings.use_tile_bonus);

    let tile_bonus_text = get_text(state.read().settings.language, "tile_bonus").unwrap();

    cx.render(rsx!(
        div {
            class: "grid grid-cols-2 gap-4 h-16 pt-4",
            span {
                class: "col-span-1 justify-self-start font-semibold text-lg",
                "{tile_bonus_text}"
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
                    class: "w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#ee609c]"
                }
            }
        }
    ))
}

fn max_score_enable(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let enabled = use_state(&cx, || state.read().settings.end_game_at_score);

    let max_score_label = get_text(state.read().settings.language, "end_at_max_score").unwrap();

    cx.render(rsx!(
        div {
            class: "grid grid-cols-6 gap-4 h-16 pt-4",
            span {
                class: "col-span-5 justify-self-start font-semibold text-lg",
                "{max_score_label}"
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
                    class: "w-11 h-6 bg-gray-200 rounded-full peer peer-focus:outline-none peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#ee609c]"
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
            div {
                class: "h-12 scale-x-[-1]",
                assets::back()
            }
        },
        button {
            class: "absolute top-4 right-4",
            onclick: |_| {
                state.write().settings.save();
                state.write().screen = Screen::Credits;
            },
            div {
                class: "h-12",
                assets::info(),
            }
        }
    ))
}
