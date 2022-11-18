use crate::backend::print_version_number;
use crate::prelude::*;
use dioxus::prelude::*;

pub fn screen(cx: Scope) -> Element {
    log!("Rendering credits.");
    cx.render(rsx!(
        top_bar(),
        div {
            class: "flex flex-col gap-8 h-screen justify-center items-center",
            div {
                class: "flex flex-col items-center gap-8",
                img {
                    class: "w-2/3",
                    src: "img/intro.gif",
                }
                p {
                    class: "text-white font-semibold text-lg text-center w-max max-w-1/2 px-2 rounded-full",
                    background: "linear-gradient(270deg, #B465DA 0%, #CF6CC9 28.04%, #EE609C 67.6%, #EE609C 100%)",
                    print_version_number(),
                }
            },
            div {
                class: "flex flex-col justify-center items-center gap-8 w-full",
                p {
                    class: "w-3/4 text-center",
                    p {
                        class: "font-semibold",
                        "Programming:"
                    }
                    p {
                        "Antonio Curavalea",
                    }
                }
                p {
                    class: "w-3/4 text-center",
                    p {
                        class: "font-semibold",
                        "UX/UI Design:"
                    }
                    p {
                        "Vlad Tantarean",
                    }
                },
                div {
                    class: "w-3/4 grid grid-cols-2 justify-items-center",
                    p {
                        class: "w-full text-center col-span-1",
                        p {
                            class: "font-semibold",
                            "Icons:"
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
                    }
                    p {
                        class: "w-full text-center col-span-1",
                        p {
                            class: "font-semibold",
                            "Tech:"
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
                    }
                }
            },
            div {
                class: "flex flex-col justify-center items-center gap-2 w-full",
                p {
                    class: "w-3/4 text-center",
                    "Made with ❤️ in Romania."
                }
            }
            div {
                class: "flex flex-col absolute bottom-4 w-2/3 gap-4 h-max justify-center place-content-center place-self-center",
                a {
                    class: "flex flex-row gap-2 h-10 items-center w-1/2 place-self-center justify-center",
                    href: "https://github.com/arqalite/rummy-nights",
                    target: "_blank",
                    img {
                        class: "h-8",
                        src: "img/github.svg",
                    }
                }
            }
        }
    ))
}

fn top_bar(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    log!("Rendering nav bar.");
    cx.render(rsx!(
        button {
            class: "absolute top-4 left-4",
            onclick: |_| {
                state.write().screen = Screen::Settings;
            },
            img {
                class: "h-12 scale-x-[-1]",
                src: "img/back.svg",
            }
        },
    ))
}
