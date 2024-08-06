// #![feature(round_char_boundary)]
#![allow(clippy::type_complexity)]
use bevy::{
    prelude::*,
};
use bevy_ui_navigation::{
    prelude::*,
    systems::InputMapping,

};
//mod focus;
use std::borrow::Cow;
pub mod construct;
pub mod prompt;
pub mod view;
mod num_like;
pub use num_like::*;
mod string_cursor;
pub use string_cursor::*;
// pub use focus::*;
pub struct AskyPlugin;

impl Plugin for AskyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(prompt::plugin)
            .add_plugins(DefaultNavigationPlugins)
            .add_systems(Startup, setup);
        // app.add_plugins(focus::plugin);
    }
}

fn setup(mut commands: Commands, mut input_mapping: ResMut<InputMapping>) {
    input_mapping.keyboard_navigation = true;
    // input_mapping.focus_follows_mouse = true;
}

#[derive(Event, Deref, Debug)]
pub struct AskyEvent<T>(pub Result<T, Error>);

#[derive(Debug, Component, Default, Clone)]
pub enum AskyState {
    #[default]
    Reading,
    Complete,
    Error,
}

#[derive(Debug, Component, Default, Clone)]
pub struct Submitter;

pub trait Submit {
    type Out;
    fn submit(&self) -> Result<Self::Out, Error>;
}

impl AskyState {
    fn is_done(&self) -> bool {
        matches!(self, AskyState::Complete | AskyState::Error)
    }
}

/// Asky errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// User cancelled.
    #[error("cancelled")]
    Cancel,
    /// Input was invalid.
    #[error("invalid input")]
    InvalidInput,
    #[error("invalid number")]
    InvalidNumber,
    /// Invalid count with expected and actual.
    // #[error("invalid count, expected {expected} actual {actual}")]
    // InvalidCount {
    //     /// Expected count
    //     expected: usize,
    //     /// Actual count
    //     actual: usize,
    // },
    /// Validation failed.
    #[error("validation fail")]
    ValidationFail,
    /// Message
    #[error("{0}")]
    Message(Cow<'static, str>),
    /// There was an [std::io::Error].
    #[error("io error {0}")]
    Io(#[from] std::io::Error),
    #[cfg(feature = "bevy")]
    /// Async error
    // #[error("async error {0}")]
    // Async(#[from] bevy_defer::AccessError),
    /// Promise error
    #[error("promise error {0}")]
    Promise(#[from] promise_out::Error),
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
