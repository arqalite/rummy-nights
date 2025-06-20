use crate::prelude::*;
use dioxus::events::FormEvent;
use dioxus::prelude::*;
use gloo_console::log;
use gloo_storage::{LocalStorage, SessionStorage, Storage};

pub fn SettingsScreen() -> Element {
    log!("Rendering settings menu.");

    let settings = STATE.read().settings;

    rsx!(
        section {
            class: "flex flex-col grow justify-between",
            div {
                class: "flex flex-row mt-4 px-4 justify-between",
                button {
                    class: "",
                    onclick: move |_| {
                        STATE.write().settings.save();
                        STATE.write().go_to_screen(Screen::Menu);
                    },
                    div {
                        class: "h-12 scale-x-[-1]",
                        assets::BackIcon {}
                    }
                },
                button {
                    class: "",
                    onclick: move |_| {
                        STATE.write().settings.save();
                        STATE.write().go_to_screen(Screen::Credits);
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
                    label: get_text( "score_editing"),
                    setting: STATE.read().settings.enable_score_editing,
                    on_switch: move |enabled| STATE.write().enable_score_editing(enabled),
                }
                SwitchSetting {
                    label: get_text( "score_checking"),
                    setting: STATE.read().settings.enable_score_checking,
                    on_switch: move |enabled| STATE.write().enable_score_checking(enabled),
                }
                SwitchSetting{
                    label: get_text( "dealer_tracking"),
                    setting: STATE.read().settings.enable_dealer_tracking,
                    on_switch: move |enabled| STATE.write().enable_dealer_tracking(enabled),
                }
                div {
                    class: "flex flex-col grow gap-4 max-h-32",
                    SwitchSetting {
                        label: get_text( "tile_bonus"),
                        setting: STATE.read().settings.use_tile_bonus,
                        on_switch: move |enabled| STATE.write().enable_tile_bonus(enabled),
                    }
                    if settings.use_tile_bonus {
                        ValueSetting {
                            label: get_text( "tile_bonus_value"),
                            setting: STATE.read().settings.tile_bonus_value,
                            on_submit: move |value| STATE.write().settings.set_tile_bonus(value),
                        },
                    }
                },
                div {
                    class: "flex flex-col grow gap-4 max-h-32",
                    SwitchSetting {
                        label: get_text( "end_at_max_score"),
                        setting: STATE.read().settings.end_game_at_score,
                        on_switch: move |enabled| STATE.write().enable_max_score(enabled),
                    }
                    if settings.end_game_at_score {
                        ValueSetting {
                            label: get_text( "max_score"),
                            setting: STATE.read().settings.max_score,
                            on_submit: move |value| STATE.write().settings.set_max_score(value),
                        },
                    }
                },
                LanguageSelect  {},
            }
            div {
                class: "flex flex-col gap-2 mb-4",
                button {
                    class: "flex flex-row gap-2 items-center w-full place-self-center justify-center",
                    onclick: move |_| {
                        SessionStorage::clear();
                        document::eval("location.reload()");
                    },
                    div {
                        class: "h-8",
                        assets::ReplayIcon {}
                    }
                    span {
                        class: "font-semibold text-lg leading-8 h-8",
                        {get_text( "restart")}
                    }
                }
                button {
                    class: "flex flex-row gap-2 items-center w-full place-self-center justify-center",
                    onclick: move |_| {
                        LocalStorage::clear();
                        SessionStorage::clear();
                        document::eval("location.reload()");
                    },
                    div {
                        class: "h-8",
                        assets::BinIcon {}
                    }
                    span {
                        class: "font-semibold text-lg leading-8 h-8",
                        {get_text("clear_data")}
                    }
                }
            }
        }
    )
}

fn LanguageSelect() -> Element {
    let mut ro_enabled = "";
    let mut en_enabled = "";

    match STATE.read().settings.language {
        2 => {
            ro_enabled = "outline";
        }
        _ => {
            en_enabled = "outline";
        }
    }

    rsx!(
        div {
            class: "grid grid-cols-6 gap-4 h-12 py-4 items-center",
            span {
                class: "col-span-4 justify-self-start font-semibold text-lg",
                {get_text("language")}
            },
            button {
                class: "h-8 w-max {ro_enabled} outline-2 outline-offset-4 outline-[#ee609c]",
                onclick: move |_| STATE.write().set_language(2),
                assets::RomanianFlagIcon {},
            },
            button {
                class: "h-8 w-max {en_enabled} outline-2 outline-offset-4 outline-[#ee609c]",
                onclick: move |_| STATE.write().set_language(1),
                assets::EnglishFlagIcon {},
            }

        }
    )
}

#[component]
fn SwitchSetting(label: String, setting: bool, on_switch: EventHandler<bool>) -> Element {
    let mut enabled = use_signal(|| setting);
    rsx!(
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
                        enabled.set(!enabled());
                        on_switch.call(enabled());
                    }
                }
                div {
                    class: "w-11 h-6 bg-gray-200 rounded-full peer peer-focus:outline-none peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#ee609c]"
                }
            }
        }
    )
}

#[component]
fn ValueSetting(label: String, setting: i32, on_submit: EventHandler<i32>) -> Element {
    let mut max_score = use_signal(|| setting);
    let mut changed = use_signal(|| false);

    let is_button_hidden = if changed() {
        String::from("")
    } else {
        String::from("hidden")
    };

    rsx!(
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
                    .values()
                    .get("max_score")
                    .unwrap()
                    .join("")
                    .parse::<i32>()
                    .unwrap_or(1000);

                    log!(format!("Input value is {max_score}"));

                    let update_score = format!(
                        "document.getElementById('max_score').value = '{max_score}';"
                    );

                    if max_score > 0 {
                        changed.set(false);
                        on_submit.call(max_score);
                        document::eval(&update_score);
                    }
                },

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
                        max_score.set(evt.value().parse::<i32>().unwrap_or(0));
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
