// #![feature(round_char_boundary)]
use crate::construct::*;
use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use std::borrow::Cow;
pub mod construct;
pub mod prompt;
pub mod view;

pub struct AskyPlugin;

/// AskySet defines when the input events are emitted.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AskySet {
    /// Run before any input events are emitted.
    Pre,
    /// Process the input.
    ProcessInput,
    /// Render views if necessary.
    ConstructView,
    /// Run after all input events are emitted.
    Post,
}

impl Plugin for AskyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(prompt::plugin);
        // .configure_sets(
        //     Update,
        //     (
        //         AskySet::Pre,
        //         AskySet::ProcessInput,
        //         AskySet::ConstructView,
        //         AskySet::Post,
        //     )
        //         .chain(),
        // );
    }
}

#[derive(Event, Deref, Debug)]
pub struct AskyEvent<T>(pub Result<T, Error>);

#[derive(Debug, Component, Default, Clone)]
pub enum AskyState {
    Frozen, // XXX: Drop frozen
    #[default]
    Uninit,
    //Construct,
    Reading,
    Complete,
    Error,
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
    /// Invalid count with expected and actual.
    #[error("invalid count, expected {expected} actual {actual}")]
    InvalidCount {
        /// Expected count
        expected: usize,
        /// Actual count
        actual: usize,
    },
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
