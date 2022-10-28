pub mod data;
pub mod screens;

pub mod prelude {
    pub use crate::data::model::Screen;
    pub use crate::screens::*;
    pub use crate::data::model::Model;
    pub use crate::data::model::STATE;
    pub use crate::data::model::GameStatus;
    pub use crate::data::model::Player;
}