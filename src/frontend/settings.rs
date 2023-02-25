use crate::prelude::*;
use dioxus::events::FormEvent;
use dioxus::prelude::*;
use gloo_console::log;

#[inline_props]
pub fn SettingsScreen<'a>(
    cx: Scope,
    settings: Settings,
    on_restart_app: EventHandler<'a, MouseEvent>,
    on_clear_data: EventHandler<'a, MouseEvent>,
    on_tile_enable: EventHandler<'a, bool>,
    on_set_tile_bonus_value: EventHandler<'a, i32>,
    on_set_language: EventHandler<'a, usize>,
    on_return_to_menu: EventHandler<'a, MouseEvent>,
    on_go_to_credits: EventHandler<'a, MouseEvent>,
    on_toggle_edit: EventHandler<'a, bool>,
    on_toggle_dealer: EventHandler<'a, bool>,
    on_toggle_max_score: EventHandler<'a, bool>,
    on_score_change: EventHandler<'a, i32>,
) -> Element {
    log!("Rendering settings menu.");

    render!(
        section {
            class: "flex flex-col grow justify-between",
            div {
                class: "flex flex-row my-4 px-4 justify-between",
                button {
                    class: "",
                    onclick: |evt| on_return_to_menu.call(evt),
                    div {
                        class: "h-12 scale-x-[-1]",
                        assets::BackIcon {}
                    }
                },
                button {
                    class: "",
                    onclick: |evt| on_go_to_credits.call(evt),
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
                    EditEnable {
                        lang_code: settings.language,
                        enabled: settings.enable_score_editing,
                        on_toggle_edit: |value| on_toggle_edit.call(value)
                    },
                },
                div {
                    class: "flex flex-col gap-4",
                    DealerEnable {
                        lang_code: settings.language,
                        enabled: settings.enable_dealer_tracking,
                        on_toggle_dealer: |value| on_toggle_dealer.call(value)
                    },
                },
                div {
                    class: "flex flex-col gap-4",
                    TileBonusEnable {
                        lang_code: settings.language,
                        enabled: settings.use_tile_bonus,
                        on_tile_enable: |enabled| on_tile_enable.call(enabled),
                    },
                    settings.use_tile_bonus.then(|| rsx!(
                        TileBonusValueSetting {
                            lang_code: settings.language,
                            value: settings.tile_bonus_value,
                            on_tile_bonus_value: |value| on_set_tile_bonus_value.call(value)
                        },
                    ))
                },
                div {
                    class: "flex flex-col gap-4",
                    MaxScoreEnable {
                        lang_code: settings.language,
                        enabled: settings.end_game_at_score,
                        on_toggle_max_score: |enabled| on_toggle_max_score.call(enabled)
                    },
                    settings.end_game_at_score.then(|| rsx!(
                        MaxScoreSetting {
                            lang_code: settings.language,
                            max_score: settings.max_score,
                            on_score_change: |score| on_score_change.call(score)
                        },
                    ))
                },
                div {
                    class: "flex flex-col gap-4",
                    LanguageSelect  {
                        lang_code: settings.language,
                        on_set_language: |value: usize| on_set_language.call(value)
                    },
                },
            }
            div {
                class: "flex flex-col gap-2 mb-4",
                button {
                    class: "flex flex-row gap-2 items-center w-full place-self-center justify-center",
                    onclick: |evt| on_restart_app.call(evt),
                    div {
                        class: "h-8",
                        assets::ReplayIcon {}
                    }
                    span {
                        class: "font-semibold text-lg leading-8 h-8",
                        get_text(settings.language, "restart")
                    }
                }
                button {
                    class: "flex flex-row gap-2 items-center w-full place-self-center justify-center",
                    onclick: |evt| on_clear_data.call(evt),
                    div {
                        class: "h-8",
                        assets::BinIcon {}
                    }
                    span {
                        class: "font-semibold text-lg leading-8 h-8",
                        get_text(settings.language, "clear_data")
                    }
                }
            }
        }
    )
}

#[inline_props]
fn LanguageSelect<'a>(
    cx: Scope,
    lang_code: usize,
    on_set_language: EventHandler<'a, usize>,
) -> Element {
    let mut ro_enabled = "";
    let mut en_enabled = "";

    match lang_code {
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
                get_text(*lang_code, "language")
            },
            button {
                class: "h-8 w-max {ro_enabled} outline-2 outline-offset-4 outline-[#ee609c]",
                onclick: move |_| on_set_language.call(2),
                assets::RomanianFlagIcon {},
            },
            button {
                class: "h-8 w-max {en_enabled} outline-2 outline-offset-4 outline-[#ee609c]",
                onclick: move |_| on_set_language.call(1),
                assets::EnglishFlagIcon {},
            }

        }
    )
}

#[inline_props]
fn EditEnable<'a>(
    cx: Scope,
    lang_code: usize,
    enabled: bool,
    on_toggle_edit: EventHandler<'a, bool>,
) -> Element {
    let enabled = use_state(cx, || *enabled);

    render!(
        div {
            class: "grid grid-cols-6 gap-4 h-16 pt-4",
            span {
                class: "col-span-5 justify-self-start font-semibold text-lg",
                get_text(*lang_code, "score_editing")
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
                        on_toggle_edit.call(*enabled.current());
                    }
                }
                div {
                    class: "w-11 h-6 bg-gray-200 rounded-full peer peer-focus:outline-none peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#ee609c]"
                }
            }
        }
    )
}

#[inline_props]
fn DealerEnable<'a>(
    cx: Scope,
    lang_code: usize,
    enabled: bool,
    on_toggle_dealer: EventHandler<'a, bool>,
) -> Element {
    let enabled = use_state(cx, || *enabled);

    render!(
        div {
            class: "grid grid-cols-6 gap-4 h-16 pt-4",
            span {
                class: "col-span-5 justify-self-start font-semibold text-lg",
                get_text(*lang_code, "dealer_tracking")
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
                        on_toggle_dealer.call(*enabled.current());
                    }
                }
                div {
                    class: "w-11 h-6 bg-gray-200 rounded-full peer peer-focus:outline-none peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#ee609c]"
                }
            }
        }
    )
}

#[inline_props]
fn MaxScoreSetting<'a>(
    cx: Scope,
    lang_code: usize,
    max_score: i32,
    on_score_change: EventHandler<'a, i32>,
) -> Element {
    let max_score = use_state(cx, || *max_score);
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
                get_text(*lang_code, "max_score")
            }
            form {
                class: "flex flex-row w-full justify-evenly",
                onsubmit: |evt| {
                    let max_score = evt
                    .values
                    .get("max_score")
                    .unwrap()
                    .parse::<i32>()
                    .unwrap_or(1000);

                    log!(format!("Input value is {max_score}"));

                    if max_score > 0 {
                        changed.set(false);
                        on_score_change.call(max_score);
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

#[inline_props]
fn TileBonusValueSetting<'a>(
    cx: Scope,
    lang_code: usize,
    value: i32,
    on_tile_bonus_value: EventHandler<'a, i32>,
) -> Element {
    let value = use_state(cx, || *value);
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
                get_text(*lang_code, "tile_bonus_value")
            }
            form {
                class: "flex flex-row w-full justify-evenly",
                onsubmit: |evt| {
                    let tile_bonus = evt
                    .values
                    .get("tile_bonus")
                    .unwrap()
                    .parse::<i32>()
                    .unwrap_or(50);

                    log!(format!("Input tile bonus value is {tile_bonus}"));

                    if tile_bonus > 0 {
                        changed.set(false);
                        on_tile_bonus_value.call(tile_bonus);
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

#[inline_props]
fn TileBonusEnable<'a>(
    cx: Scope,
    lang_code: usize,
    enabled: bool,
    on_tile_enable: EventHandler<'a, bool>,
) -> Element {
    let enabled = use_state(cx, || *enabled);

    render!(
        div {
            class: "grid grid-cols-2 gap-4 h-16 pt-4",
            span {
                class: "col-span-1 justify-self-start font-semibold text-lg",
                get_text(*lang_code, "tile_bonus")
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
                        on_tile_enable.call(*enabled.current());
                    }
                }
                div {
                    class: "w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#ee609c]"
                }
            }
        }
    )
}

#[inline_props]
fn MaxScoreEnable<'a>(
    cx: Scope,
    lang_code: usize,
    enabled: bool,
    on_toggle_max_score: EventHandler<'a, bool>,
) -> Element {
    let option_enabled = use_state(cx, || *enabled);

    render!(
        div {
            class: "grid grid-cols-6 gap-4 h-16 pt-4",
            span {
                class: "col-span-5 justify-self-start font-semibold text-lg",
                get_text(*lang_code, "end_at_max_score")
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
                        on_toggle_max_score.call(*option_enabled.current());
                    }
                },
                div {
                    class: "w-11 h-6 bg-gray-200 rounded-full peer peer-focus:outline-none peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#ee609c]"
                }
            }
        }
    )
}
