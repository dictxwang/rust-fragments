pub mod action;
// use action::*;

pub mod plane;
// use plane::*;

pub mod animal;

pub mod prelude {
    pub use super:: {
        action::*, plane::*, animal::*,
    };
}