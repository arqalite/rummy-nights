pub mod backend;
pub mod frontend;

pub mod prelude {
    pub use crate::backend::prelude::*;
    pub use crate::frontend::*;
    pub use gloo_console::log;
}
