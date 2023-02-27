use crate::prelude::*;
use dioxus::events::FormEvent;
use dioxus::prelude::*;
use dioxus_web::use_eval;
use gloo_console::log;
use gloo_storage::{LocalStorage, SessionStorage, Storage};

pub fn SettingsScreen(cx: Scope) -> Element {
    log!("Rendering settings menu.");
    let state = fermi::use_atom_ref(cx, STATE);
    let settings = state.read().settings;

    render!(
        section {
            class: "flex flex-col grow justify-between",
            div {
                class: "flex flex-row my-4 px-4 justify-between",
                button {
                    class: "",
                    onclick: move |_| {
                        state.write().settings.save();
                        state.write().go_to_screen(Screen::Menu);
                    },
                    div {
                        class: "h-12 scale-x-[-1]",
                        assets::BackIcon {}
                    }
                },
                button {
                    class: "",
                    onclick: move |_| {
                        state.write().settings.save();
                        state.write().go_to_screen(Screen::Credits);
                    },
                    div {
                        class: "h-12",
                        assets::InfoIcon {},
                    }
                },
            }
            div {
                class: "flex flex-col divide-y divide-slate-200 justify-evenly px-8",
                div {
                    class: "flex flex-col gap-4",
                    EditEnable {},
                },
                div {
                    class: "flex flex-col gap-4",
                    DealerEnable {},
                },
                div {
                    class: "flex flex-col gap-4",
                    TileBonusEnable {},
                    settings.use_tile_bonus.then(|| rsx!(
                        TileBonusValueSetting {},
                    ))
                },
                div {
                    class: "flex flex-col gap-4",
                    MaxScoreEnable {},
                    settings.end_game_at_score.then(|| rsx!(
                        MaxScoreSetting {},
                    ))
                },
                div {
                    class: "flex flex-col gap-4",
                    LanguageSelect  {},
                },
            }
            div {
                class: "flex flex-col gap-2 mb-4",
                button {
                    class: "flex flex-row gap-2 items-center w-full place-self-center justify-center",
                    onclick: move |_| {
                        SessionStorage::clear();
                        use_eval(cx)("location.reload()");
                    },
                    div {
                        class: "h-8",
                        assets::ReplayIcon {}
                    }
                    span {
                        class: "font-semibold text-lg leading-8 h-8",
                        get_text(cx, "restart")
                    }
                }
                button {
                    class: "flex flex-row gap-2 items-center w-full place-self-center justify-center",
                    onclick: move |_| {
                        LocalStorage::clear();
                        SessionStorage::clear();
                        use_eval(cx)("location.reload()");
                    },
                    div {
                        class: "h-8",
                        assets::BinIcon {}
                    }
                    span {
                        class: "font-semibold text-lg leading-8 h-8",
                        get_text(cx, "clear_data")
                    }
                }
            }
        }
    )
}

fn LanguageSelect(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
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

    render!(
        div {
            class: "grid grid-cols-6 gap-4 h-16 pt-4",
            span {
                class: "col-span-4 justify-self-start font-semibold text-lg",
                get_text(cx, "language")
            },
            button {
                class: "h-8 w-max {ro_enabled} outline-2 outline-offset-4 outline-[#ee609c]",
                onclick: move |_| state.write().set_language(2),
                assets::RomanianFlagIcon {},
            },
            button {
                class: "h-8 w-max {en_enabled} outline-2 outline-offset-4 outline-[#ee609c]",
                onclick: move |_| state.write().set_language(1),
                assets::EnglishFlagIcon {},
            }

        }
    )
}

fn EditEnable(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    let enabled = use_state(cx, || state.read().settings.enable_score_editing);

    render!(
        div {
            class: "grid grid-cols-6 gap-4 h-16 pt-4",
            span {
                class: "col-span-5 justify-self-start font-semibold text-lg",
                get_text(cx, "score_editing")
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
                        state.write().enable_score_editing(*enabled.current());
                    }
                }
                div {
                    class: "w-11 h-6 bg-gray-200 rounded-full peer peer-focus:outline-none peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#ee609c]"
                }
            }
        }
    )
}

fn DealerEnable(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    let enabled = use_state(cx, || state.read().settings.enable_dealer_tracking);

    render!(
        div {
            class: "grid grid-cols-6 gap-4 h-16 pt-4",
            span {
                class: "col-span-5 justify-self-start font-semibold text-lg",
                get_text(cx, "dealer_tracking")
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
                        state.write().enable_dealer_tracking(*enabled.current())
                    }
                }
                div {
                    class: "w-11 h-6 bg-gray-200 rounded-full peer peer-focus:outline-none peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#ee609c]"
                }
            }
        }
    )
}

fn MaxScoreSetting(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    let max_score = use_state(cx, || state.read().settings.max_score);
    let changed = use_state(cx, || false);

    let is_button_hidden = if **changed {
        String::from("")
    } else {
        String::from("hidden")
    };

    render!(
        div {
            class: "grid grid-cols-2 gap-4 h-16",
            span {
                class: "col-span-1 justify-self-end font-semibold text-lg",
                get_text(cx, "max_score")
            }
            form {
                class: "flex flex-row w-full justify-evenly",
                onsubmit: move |evt| {
                    let max_score = evt
                    .values
                    .get("max_score")
                    .unwrap()
                    .parse::<i32>()
                    .unwrap_or(1000);

                    log!(format!("Input value is {max_score}"));

                    if max_score > 0 {
                        changed.set(false);
                        state.write().settings.set_max_score(max_score);
                        use_eval(cx)(format!(
                            "document.getElementById('max_score').value = '{max_score}';"
                        ));
                    }
                },
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
                        assets::AddIcon {},
                    }
                }
            }
        }
    )
}

fn TileBonusValueSetting(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    let value = use_state(cx, || state.read().settings.tile_bonus_value);
    let changed = use_state(cx, || false);

    let is_button_hidden = if **changed {
        String::from("")
    } else {
        String::from("hidden")
    };

    render!(
        div {
            class: "grid grid-cols-2 gap-4 h-16",
            span {
                class: "col-span-1 justify-self-end font-semibold text-lg",
                get_text(cx, "tile_bonus_value")
            }
            form {
                class: "flex flex-row w-full justify-evenly",
                onsubmit: move |evt| {
                    let tile_bonus = evt
                    .values
                    .get("tile_bonus")
                    .unwrap()
                    .parse::<i32>()
                    .unwrap_or(50);

                    log!(format!("Input tile bonus value is {tile_bonus}"));

                    if tile_bonus > 0 {
                        changed.set(false);
                        state.write().settings.set_tile_bonus(tile_bonus);
                        use_eval(cx)(format!(
                            "document.getElementById('max_score').value = '{tile_bonus}';"
                        ));
                    }
                },
                prevent_default: "onsubmit",
                input {
                    name: "tile_bonus",
                    class: "text-lg appearance-none font-light bg-transparent h-10 w-3/4 text-center rounded focus:border-b-[8px] border-b-4 border-[#ee609c]",
                    id: "tile_bonus",
                    style: "-moz-appearance:textfield",
                    outline: "none",
                    r#type: "number",
                    value: "{value}",
                    oninput: move |evt: FormEvent| {
                        changed.set(true);
                        value.set(evt.value.parse::<i32>().unwrap_or(0));
                    },
                }
                div {
                    class: "w-10 h-10 flex justify-center items-center",
                    button {
                        class: "h-6 {is_button_hidden}",
                        r#type: "submit",
                        assets::AddIcon {},
                    }
                }
            }
        }
    )
}

fn TileBonusEnable(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    let enabled = use_state(cx, || state.read().settings.use_tile_bonus);

    render!(
        div {
            class: "grid grid-cols-2 gap-4 h-16 pt-4",
            span {
                class: "col-span-1 justify-self-start font-semibold text-lg",
                get_text(cx, "tile_bonus")
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
                        state.write().enable_tile_bonus(*enabled.current());
                    }
                }
                div {
                    class: "w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#ee609c]"
                }
            }
        }
    )
}

fn MaxScoreEnable(cx: Scope) -> Element {
    let state = fermi::use_atom_ref(cx, STATE);
    let option_enabled = use_state(cx, || state.read().settings.end_game_at_score);

    render!(
        div {
            class: "grid grid-cols-6 gap-4 h-16 pt-4",
            span {
                class: "col-span-5 justify-self-start font-semibold text-lg",
                get_text(cx, "end_at_max_score")
            }
            label {
                class: "inline-flex relative cursor-pointer justify-self-end",
                input {
                    r#type: "checkbox",
                    id: "default-toggle",
                    class: "sr-only peer",
                    checked: "{option_enabled}",
                    onchange: move |_| {
                        option_enabled.set(!option_enabled);
                        state.write().enable_max_score(*option_enabled.current())
                    }
                },
                div {
                    class: "w-11 h-6 bg-gray-200 rounded-full peer peer-focus:outline-none peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#ee609c]"
                }
            }
        }
    )
}
