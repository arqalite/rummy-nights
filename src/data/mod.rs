pub mod model;
pub mod tailwind_classes;

use dioxus::prelude::*;

pub fn print_version_number(cx: Scope) -> Element {
    let version = env!("BUILD_VERSION");
    cx.render(rsx!("{version}"))
}