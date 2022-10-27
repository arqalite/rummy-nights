//! Data structures, custom types and functions for Rummy Nights.
/// Here we should only have structs, enums, functions that deal with data, and Tailwind CSS classes.
use dioxus::prelude::*;

pub mod data;
pub mod screens;

// Preludes generate a lot of mixed opinions,
// but in my opinion having a decent code editor solves most of the issues it generates.
// Having frequently-used types and functions ready without importing them makes the code so much cleaner and readable.
pub mod prelude {
    //All the screens reference other screens so this is a must.
    pub use crate::data::model::Screen;
    pub use crate::screens::*;

    //This is a must as well, the state is used everywhere.
    pub use crate::data::model::Model;
    pub use crate::data::model::STATE;

    //Exposing the main data types.
    pub use crate::data::model::GameStatus;
    pub use crate::data::model::Player;
}

pub fn print_version_number(cx: Scope) -> Element {
    let version = env!("BUILD_VERSION");
    cx.render(rsx!("{version}"))
}
