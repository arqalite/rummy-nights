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
                class: "flex flex-row mt-4 px-4 justify-between",
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
                class: "flex flex-col grow justify-evenly px-8 max-h-[70%]",
                SwitchSetting {
                    label: get_text(cx, "score_editing"),
                    setting: state.read().settings.enable_score_editing,
                    on_switch: move |enabled| state.write().enable_score_editing(enabled),
                }
                SwitchSetting {
                    label: get_text(cx, "score_checking"),
                    setting: state.read().settings.enable_score_checking,
                    on_switch: move |enabled| state.write().enable_score_checking(enabled),
                }
                SwitchSetting{
                    label: get_text(cx, "dealer_tracking"),
                    setting: state.read().settings.enable_dealer_tracking,
                    on_switch: move |enabled| state.write().enable_dealer_tracking(enabled),
                }
                div {
                    class: "flex flex-col grow gap-4 max-h-32",
                    SwitchSetting {
                        label: get_text(cx, "tile_bonus"),
                        setting: state.read().settings.use_tile_bonus,
                        on_switch: move |enabled| state.write().enable_tile_bonus(enabled),
                    }
                    settings.use_tile_bonus.then(|| rsx!(
                        ValueSetting {
                            label: get_text(cx, "tile_bonus_value"),
                            setting: state.read().settings.tile_bonus_value,
                            on_submit: move |value| state.write().settings.set_tile_bonus(value),
                        },
                    ))
                },
                div {
                    class: "flex flex-col grow gap-4 max-h-32",
                    SwitchSetting {
                        label: get_text(cx, "end_at_max_score"),
                        setting: state.read().settings.end_game_at_score,
                        on_switch: move |enabled| state.write().enable_max_score(enabled),
                    }
                    settings.end_game_at_score.then(|| rsx!(
                        ValueSetting {
                            label: get_text(cx, "max_score"),
                            setting: state.read().settings.max_score,
                            on_submit: move |value| state.write().settings.set_max_score(value),
                        },
                    ))
                },
                LanguageSelect  {},
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
            class: "grid grid-cols-6 gap-4 h-12 py-4 items-center",
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

#[inline_props]
fn SwitchSetting<'a>(
    cx: Scope,
    label: &'a str,
    setting: bool,
    on_switch: EventHandler<'a, bool>,
) -> Element {
    let enabled = use_state(cx, || *setting);

    render!(
        div {
            class: "grid grid-cols-6 gap-4 items-center grow max-h-16",
            span {
                class: "col-span-5 justify-self-start font-semibold text-lg",
                "{label}"
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
                        on_switch.call(*enabled.current());
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
fn ValueSetting<'a>(
    cx: Scope,
    label: &'a str,
    setting: i32,
    on_submit: EventHandler<'a, i32>,
) -> Element {
    let max_score = use_state(cx, || *setting);
    let changed = use_state(cx, || false);

    let is_button_hidden = if **changed {
        String::from("")
    } else {
        String::from("hidden")
    };

    render!(
        div {
            class: "grid grid-cols-2 gap-4 h-12 pb-2",
            span {
                class: "col-span-1 justify-self-end font-semibold text-lg",
                "{label}"
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
                        on_submit.call(max_score);
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
