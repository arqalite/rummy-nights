use crate::backend::VersionNumber;
use crate::prelude::*;
use dioxus::prelude::*;

pub fn CreditsScreen(cx: Scope) -> Element {
    log!("Rendering credits.");
    let state = fermi::use_atom_ref(cx, &STATE);

    render!(
        button {
            class: "absolute top-4 left-4",
            onclick: move |_| state.write().go_to_screen(Screen::Settings),
            div {
                class: "h-12 scale-x-[-1]",
                assets::BackIcon {}
            }
        },
        div {
            class: "flex flex-col gap-8 h-screen justify-center items-center px-8",
            div {
                class: "flex flex-col items-center gap-8",
                img {
                    class: "w-2/3",
                    src: "intro_logo.gif",
                }
                p {
                    class: "text-white font-semibold text-lg text-center w-max max-w-1/2 px-2 rounded-full",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    VersionNumber {},
                }
            },
            div {
                class: "flex flex-col justify-center items-center gap-8 w-full",
                p {
                    class: "w-3/4 text-center",
                    p {
                        class: "font-semibold",
                        get_text(cx,"programmer")
                    }
                    p {
                        "Antonio Curăvalea",
                    }
                }
                p {
                    class: "w-3/4 text-center",
                    p {
                        class: "font-semibold",
                        get_text(cx,"design")
                    }
                    p {
                        "Vlad Țânțărean",
                    }
                },
                div {
                    class: "w-3/4 grid grid-cols-2 justify-items-center",
                    p {
                        class: "w-full text-center col-span-1",
                        p {
                            class: "font-semibold",
                            get_text(cx,"icons")
                        }
                        p {
                            "Freepik/Flaticon",
                        }
                        p {
                            "Ikonate",
                        }
                        p {
                            "Charm Icons",
                        }
                        p {
                            "Lipis's flag-icons"
                        }
                    }
                    p {
                        class: "w-full text-center col-span-1",
                        p {
                            class: "font-semibold",
                            get_text(cx,"tech")
                        }
                        p {
                            "Rust",
                        }
                        p {
                            "Dioxus",
                        }
                        p {
                            "Tailwind CSS",
                        }
                        p {
                            "Trunk"
                        }
                    }
                }
            },
            div {
                class: "flex flex-col justify-center items-center gap-2 w-full",
                p {
                    class: "w-3/4 text-center",
                    get_text(cx,"love")
                }
            }
            div {
                class: "flex flex-col absolute bottom-4 w-2/3 gap-4 h-max justify-center place-content-center place-self-center",
                a {
                    class: "flex flex-row gap-2 h-10 items-center w-1/2 place-self-center justify-center",
                    href: "https://github.com/arqalite/rummy-nights",
                    target: "_blank",
                    div {
                        class: "h-8",
                        assets::GithubIcon {}
                    }
                }
            }
        }
    )
}
