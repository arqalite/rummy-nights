pub mod assets;
pub mod backend;
pub mod frontend;

pub mod prelude {
    pub use crate::assets;
    pub use crate::backend::prelude::*;
    pub use crate::frontend::*;
    pub use gloo_console::log;
}
