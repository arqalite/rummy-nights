pub mod backend;
pub mod frontend;
pub mod assets;

pub mod prelude {
    pub use crate::backend::prelude::*;
    pub use crate::frontend::*;
    pub use crate::assets;
    pub use gloo_console::log;
}
