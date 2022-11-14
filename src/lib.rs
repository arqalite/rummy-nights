//! # Rummy Nights
//! Rummy Nights is a cross-platform rummy score counter app.

//#![warn(missing_docs)]

pub mod data;
pub mod screens;
pub mod prelude {
    //! The prelude re-exports commonly used data types and functions
    //! for easy access throughout the codebase.
    //!
    //! It is usually imported via `use rummy_nights::prelude::*;`.

    pub use crate::data::GameStatus;
    pub use crate::data::Model;
    pub use crate::data::Player;
    pub use crate::data::Screen;
    pub use crate::data::STATE;
    pub use crate::data::SETTINGS;
    pub use crate::screens::*;

    //This is added for logging throughout the app.
    pub use gloo_console::log;
}
