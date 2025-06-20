use crate::backend::VersionNumber;
use crate::prelude::*;
use dioxus::prelude::*;

static ICON: Asset = asset!("/public/intro_logo.gif");

pub fn MenuScreen() -> Element {
    log!("Rendering main menu.");

    rsx!(
        button {
            class: "absolute right-4 top-4",
            onclick: move |_| STATE.write().go_to_screen(Screen::Settings),
            div {
                class: "h-12",
                assets::SettingsIcon {}
            }
        }
        div {
            class : "flex flex-col grow gap-16 justify-center px-8",
            img {
                class: "w-full max-w-lg",
                src: ICON,
            }
            div {
                class: "flex flex-col gap-8",
                MenuButton {
                    on_click: move |_| STATE.write().create_game(),
                    label: "start_game",
                    icon: rsx!(assets::NewGameIcon {})
                }
                //Hide the resume game button if there is no ongoing game.
                if STATE.read().game.status == GameStatus::Ongoing {
                    MenuButton {
                        on_click: move |_| STATE.write().go_to_screen(Screen::Game),
                        label: "resume_game",
                        icon: rsx!(assets::ResumeIcon {})
                    }
                }
            }
        }
        p {
            class: "text-white font-semibold text-lg text-center w-max max-w-1/2 px-2 absolute bottom-2 left-2 rounded-full",
            background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
            VersionNumber {}
        }
    )
}

#[component]
fn MenuButton(on_click: EventHandler<MouseEvent>, label: String, icon: Element) -> Element {
    log!(format!("Rendering main menu button: {label}"));

    rsx!(
        button {
            class: "grid grid-cols-6 items-center",
            onclick: move |evt| on_click.call(evt),
            p {
                class: "w-max font-semibold text-center text-2xl col-span-2 col-start-2 justify-self-end",
                {get_text(&label)}

            }
            div {
                class: "h-20 w-20 col-start-5 col-span-2 rounded-full",
                background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                {icon},
            }
        }
    )
}
