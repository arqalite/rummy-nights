use dioxus::prelude::*;

pub fn okay_button(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            role: "img",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100%",
            height: "100%",
            view_box: "0 0 24 24",
            stroke: "#000000",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            fill: "none",
            color: "#000000",
            polyline {
                points: "7 13 10 16 17 9"
            }
            circle {
                cx: "12",
                cy: "12",
                r: "10"
            }
        }
    ))
}

pub fn gb_flag_icon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            id: "flag-icons-gb",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 640 480",
            height: "100%",
            width: "100%",
            path {
                fill: "#012169",
                d: "M0 0h640v480H0z",
            }
            path {
                fill: "#FFF",
                d: "m75 0 244 181L562 0h78v62L400 241l240 178v61h-80L320 301 81 480H0v-60l239-178L0 64V0h75z",
            }
            path {
                d: "m424 281 216 159v40L369 281h55zm-184 20 6 35L54 480H0l240-179zM640 0v3L391 191l2-44L590 0h50zM0 0l239 176h-60L0 42V0z",
                fill: "#C8102E",
            }
            path {
                fill: "#FFF",
                d: "M241 0v480h160V0H241zM0 160v160h640V160H0z",
            }
            path {
                d: "M0 193v96h640v-96H0zM273 0v480h96V0h-96z",
                fill: "#C8102E",
            }
        }
    ))
}

pub fn romanian_flag_icon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            id: "flag-icons-ro",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 640 480",
            height: "100%",
            width: "100%",
            g {
                fill_rule: "evenodd",
                stroke_width: "1pt",
                path {
                    d: "M0 0h213.3v480H0z",
                    fill: "#00319c",
                }
                path {
                    d: "M213.3 0h213.4v480H213.3z",
                    fill: "#ffde00",
                }
                path {
                    d: "M426.7 0H640v480H426.7z",
                    fill: "#de2110",
                }
            }
        }
    ))
}

pub fn add_button(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            role: "img",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100%",
            height: "100%",
            view_box: "0 0 24 24",
            stroke: "#000000",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            fill: "none",
            color: "#000000",
            path {
                d: "M17 12L7 12M12 17L12 7"
            }
            circle {
                cx: "12",
                cy: "12",
                r: "10"
            }
        }
    ))
}

pub fn arrow_right(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            role: "img",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100%",
            height: "100%",
            view_box: "0 0 24 24",
            stroke: "#000000",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            fill: "none",
            color: "#000000",
            path {
                d: "M13.5 9l3 3-3 3"
            }
            path {
                d: "M7.5 12H15"
             }
            path {
                stroke_linecap: "round",
                d: "M16.5 12H15"
             }
            circle {
                cx: "12",
                cy: "12",
                r: "10"
            }
        }
    ))
}

pub fn back(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            role: "img",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100%",
            height: "100%",
            view_box: "0 0 24 24",
            stroke: "#000000",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            fill: "none",
            color: "#000000",
            path {
                d: "M16 4L20 8L16 12"
            }
            path {
                d: "M20 8H9.5C6.46243 8 4 10.4624 4 13.5V13.5C4 16.5376 6.46243 19 9.5 19H19"
            }
        }
    ))
}

pub fn bin(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            role: "img",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100%",
            height: "100%",
            view_box: "0 0 24 24",
            stroke: "#000000",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            fill: "none",
            color: "#000000",
            path {
                d: "M19 6L5 6M14 5L10 5M6 10L6 20C6 20.6666667 6.33333333 21 7 21 7.66666667 21 11 21 17 21 17.6666667 21 18 20.6666667 18 20 18 19.3333333 18 16 18 10"
            }
        }
    ))
}

pub fn bonus(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            role: "img",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100%",
            height: "100%",
            view_box: "0 0 18 18",
            fill: "#ee609c",
            path {
                d: "M2.7 11l8-11v7h5l-8 11v-7z"
            },
        }
    ))
}

pub fn github(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            stroke: "currentColor",
            width: "100%",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            version: "1.1",
            height: "100%",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 16 16",
            stroke_width: "1.5",
            fill: "none",
            path {
                d: "m5.75 14.25s-.5-2 .5-3c0 0-2 0-3.5-1.5s-1-4.5 0-5.5c-.5-1.5.5-2.5.5-2.5s1.5 0 2.5 1c1-.5 3.5-.5 4.5 0 1-1 2.5-1 2.5-1s1 1 .5 2.5c1 1 1.5 4 0 5.5s-3.5 1.5-3.5 1.5c1 1 .5 3 .5 3",
            }
            path {
                d: "m5.25 13.75c-1.5.5-3-.5-3.5-1",
            }
        }
    ))
}

pub fn home(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            role: "img",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100%",
            height: "100%",
            view_box: "0 0 24 24",
            stroke: "#000000",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            fill: "none",
            color: "#000000",
            path {
                d: "M2 12L5 9.3M22 12L19 9.3M19 9.3L12 3L5 9.3M19 9.3V21H5V9.3",
            }
            rect {
                width: "6",
                height: "8",
                x: "9",
                y: "13",
            }
        }
    ))
}

pub fn info(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            role: "img",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100%",
            height: "100%",
            view_box: "0 0 24 24",
            stroke: "#000000",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            fill: "none",
            color: "#000000",
            path {
                d: "M12,12 L12,15",
            }
            line {
                x2: "12",
                y1: "9",
                y2: "9",
                x1: "12",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
        }
    ))
}

pub fn new_game(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            stroke_linecap: "round",
            fill: "none",
            width: "100%",
            stroke_width: "1.5",
            xmlns: "http://www.w3.org/2000/svg",
            color: "#ffffff",
            stroke: "#ffffff",
            role: "img",
            view_box: "0 0 24 24",
            stroke_linejoin: "round",
            height: "100%",
            title {
                id: "videoIconTitle",
                "Video"
            }
            polygon {
                fill: "white",
                points: "18 12 9 16.9 9 7",
            }
        }
    ))
}

pub fn pushpin(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            fill: "none",
            height: "100%",
            stroke_linecap: "round",
            version: "1.1",
            view_box: "0 0 16 16",
            stroke_linejoin: "round",
            stroke_width: "1.5",
            stroke: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100%",
            path {
                d: "m10.25 10.25 4 4m-12.5-7.5 5-5s1 2 2 3 4.5 2 4.5 2l-6.5 6.5s-1-3.5-2-4.5-3-2-3-2z",
            }
        }
    ))
}

pub fn remove(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            role: "img",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100%",
            height: "100%",
            view_box: "0 0 24 24",
            stroke: "#000000",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            fill: "none",
            color: "#000000",
            path {
                d: "M15.5355339 15.5355339L8.46446609 8.46446609M15.5355339 8.46446609L8.46446609 15.5355339",
            }
            path {
                d: "M4.92893219,19.0710678 C1.02368927,15.1658249 1.02368927,8.83417511 4.92893219,4.92893219 C8.83417511,1.02368927 15.1658249,1.02368927 19.0710678,4.92893219 C22.9763107,8.83417511 22.9763107,15.1658249 19.0710678,19.0710678 C15.1658249,22.9763107 8.83417511,22.9763107 4.92893219,19.0710678 Z",
            }
        }
    ))
}

pub fn replay_icon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            stroke_width: "1.5",
            stroke_linejoin: "round",
            height: "100%",
            width: "100%",
            view_box: "0 0 24 24",
            role: "img",
            stroke: "#000",
            stroke_linecap: "round",
            fill: "none",
            color: "#000",
            title {
                id: "rotateIconTitle",
                "Rotate"
            }
            path {
                d: "M22 12l-3 3-3-3",
            }
            path {
                d: "M2 12l3-3 3 3",
            }
            path {
                d: "M19.016 14v-1.95A7.05 7.05 0 0 0 8 6.22",
            }
            path {
                d: "M16.016 17.845A7.05 7.05 0 0 1 5 12.015V10",
            }
            path {
                d: "M5 10V9",
                stroke_linecap: "round",
            }
            path {
                d: "M19 15v-1",
                stroke_linecap: "round",
            }
        }
    ))
}

pub fn restart_icon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            stroke_width: "1.5",
            stroke_linejoin: "round",
            height: "100%",
            width: "100%",
            view_box: "0 0 24 24",
            role: "img",
            stroke: "#000",
            stroke_linecap: "round",
            fill: "none",
            color: "#000",
            polyline {
                points: "22 12 19 15 16 12",
            }
            path {
                d: "M11,20 C6.581722,20 3,16.418278 3,12 C3,7.581722 6.581722,4 11,4 C15.418278,4 19,7.581722 19,12 L19,14",
            }
        }
    ))
}
pub fn resume_icon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: "100%",
            width: "100%",
            view_box: "0 0 24 24",
            role: "img",
            fill: "none",
            color: "#000",
            g {
                title {
                    "Layer 1"
                }
                path {
                    id: "svg_1",
                    transform: "rotate(-180 12.9075 12.5735)",
                    fill: "white",
                    d: "m14.54095,14.36372l4.57352,2.78331l0,-9.14704l-4.57352,2.78331l0,-2.78331l-7.84032,4.57352l7.84032,4.57352l0,-2.78331z",
                    stroke: "null",
                }
            }
        }
    ))
}

pub fn save_icon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            role: "img",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100%",
            height: "100%",
            view_box: "0 0 24 24",
            stroke: "#000000",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            fill: "none",
            color: "#000000",
            path {
                d: "M17.2928932,3.29289322 L21,7 L21,20 C21,20.5522847 20.5522847,21 20,21 L4,21 C3.44771525,21 3,20.5522847 3,20 L3,4 C3,3.44771525 3.44771525,3 4,3 L16.5857864,3 C16.8510029,3 17.1053568,3.10535684 17.2928932,3.29289322 Z",
            }
            rect {
                height: "8",
                width: "10",
                x: "7",
                y: "13",
            }
            rect {
                height: "5",
                width: "8",
                x: "8",
                y: "3",
            }
        }
    ))
}

pub fn settings_icon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            role: "img",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100%",
            height: "100%",
            view_box: "0 0 24 24",
            stroke: "#000000",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            fill: "none",
            color: "#000000",
            path {
                d: "M5.03506429,12.7050339 C5.01187484,12.4731696 5,12.2379716 5,12 C5,11.7620284 5.01187484,11.5268304 5.03506429,11.2949661 L3.20577137,9.23205081 L5.20577137,5.76794919 L7.9069713,6.32070904 C8.28729123,6.0461342 8.69629298,5.80882212 9.12862533,5.61412402 L10,3 L14,3 L14.8713747,5.61412402 C15.303707,5.80882212 15.7127088,6.0461342 16.0930287,6.32070904 L18.7942286,5.76794919 L20.7942286,9.23205081 L18.9649357,11.2949661 C18.9881252,11.5268304 19,11.7620284 19,12 C19,12.2379716 18.9881252,12.4731696 18.9649357,12.7050339 L20.7942286,14.7679492 L18.7942286,18.2320508 L16.0930287,17.679291 C15.7127088,17.9538658 15.303707,18.1911779 14.8713747,18.385876 L14,21 L10,21 L9.12862533,18.385876 C8.69629298,18.1911779 8.28729123,17.9538658 7.9069713,17.679291 L5.20577137,18.2320508 L3.20577137,14.7679492 L5.03506429,12.7050339 Z",
            }
            circle {
                r: "1",
                cx: "12",
                cy: "12",
            }
        }
    ))
}

pub fn trophy_icon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            color: "#000",
            width: "100%",
            height: "100%",
            stroke_width: "1.5",
            fill: "none",
            stroke: "#fff",
            xmlns: "http://www.w3.org/2000/svg",
            stroke_linecap: "round",
            view_box: "-4 -5 32 32",
            role: "img",
            stroke_linejoin: "round",
            title {
                id: "cupIconTitle",
                "Trophy"
            }
            path {
                d: "M6 2L18 2 18 11C18 14.3137085 15.3137085 17 12 17 8.6862915 17 6 14.3137085 6 11L6 2zM7 21L17 21",
            }
            path {
                d: "M12,17 L12,21",
            }
            path {
                d: "M6 5L6 11 5 11C3.34314575 11 2 9.65685425 2 8 2 6.34314575 3.34314575 5 5 5L6 5zM18 11L18 5 19 5C20.6568542 5 22 6.34314575 22 8 22 9.65685425 20.6568542 11 19 11L18 11z",
            }
        }
    ))
}

pub fn up_icon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            view_box: "0 0 16 12",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            stroke_width: "1.5",
            version: "1.1",
            width: "100%",
            stroke: "currentColor",
            stroke_linecap: "round",
            height: "100%",
            stroke_linejoin: "round",
            path {
                d: "m12.25 10.25-4.25-4.5-4.25 4.5",
            }
        }
    ))
}

pub fn play_icon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            role: "img",
            xmlns: "http://www.w3.org/2000/svg",
            width: "100%",
            height: "100%",
            view_box: "0 0 24 24",
            stroke: "#000000",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            fill: "none",
            color: "#000000",
            polygon {
                points: "18 12 9 16.9 9 7",
            }
            circle {
                cy: "12",
                cx: "12",
                r: "10",
            }
        }
    ))
}
