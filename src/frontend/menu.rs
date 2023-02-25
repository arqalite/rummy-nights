use crate::backend::VersionNumber;
use crate::prelude::*;
use dioxus::prelude::*;

#[inline_props]
pub fn MenuScreen<'a>(
    cx: Scope,
    lang_code: usize,
    game_status: GameStatus,
    on_click_settings: EventHandler<'a, MouseEvent>,
    on_click_start: EventHandler<'a, MouseEvent>,
    on_click_resume: EventHandler<'a, MouseEvent>,
) -> Element {
    log!("Rendering main menu.");

    render!(
        button {
            class: "absolute right-4 top-4",
            onclick: |evt| on_click_settings.call(evt),
            div {
                class: "h-12",
                assets::SettingsIcon {}
            }
        }
        div {
            class : "flex flex-col grow gap-16 justify-center px-8",
            img {
                class: "w-full max-w-lg",
                src: "intro_logo.gif",
            }
            div {
                class: "flex flex-col gap-8",
                MenuButton {
                    on_click: |evt| on_click_start.call(evt)
                    button_props: MenuButtonSettings { label: "start_game", lang_code: *lang_code, icon: render!(assets::NewGameIcon {}) }
                }
                //Hide the resume game button if there is no ongoing game.
                (*game_status == GameStatus::Ongoing).then(|| rsx!(
                    MenuButton {
                        on_click: |evt| on_click_resume.call(evt)
                        button_props: MenuButtonSettings { label: "resume_game", lang_code: *lang_code, icon: render!(assets::ResumeIcon {}) }
                    }
                ))
            }
        }
        p {
            class: "text-white font-semibold text-lg text-center w-max max-w-1/2 px-2 absolute bottom-2 left-2 rounded-full",
            background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
            VersionNumber {}
        }
    )
}

struct MenuButtonSettings<'a> {
    label: &'a str,
    lang_code: usize,
    icon: Element<'a>,
}

#[inline_props]
fn MenuButton<'a>(
    cx: Scope,
    on_click: EventHandler<'a, MouseEvent>,
    button_props: MenuButtonSettings<'a>,
) -> Element {
    log!(format!(
        "Rendering main menu button: {}",
        button_props.label
    ));

    render!(
        button {
            class: "grid grid-cols-6 items-center",
            onclick: |evt| on_click.call(evt),
            p {
                class: "w-max font-semibold text-center text-2xl col-span-2 col-start-2 justify-self-end",
                get_text(button_props.lang_code,button_props.label)

            }
            div {
                class: "h-20 w-20 col-start-5 col-span-2 rounded-full",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                button_props.icon.clone(),
            }
        }
    )
}
