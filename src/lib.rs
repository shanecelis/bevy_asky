// #![feature(round_char_boundary)]
#![allow(clippy::type_complexity)]
use bevy::prelude::*;

use futures::channel::oneshot;
mod focus;
use std::borrow::Cow;

#[cfg(feature = "async")]
mod r#async;
pub mod construct;
mod num_like;
pub mod prompt;
mod string_cursor;
pub mod view;

pub use focus::*;
pub use num_like::*;
#[cfg(feature = "async")]
pub use r#async::*;
pub use string_cursor::*;

pub mod prelude {
    pub use super::*;
    pub use prompt::*;
    pub use view::*;
}

pub struct AskyPlugin;

impl Plugin for AskyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(prompt::plugin)
            .add_plugins(focus::plugin);
    }
}


#[derive(Event, Deref, DerefMut, Debug, Clone)]
pub struct AskyEvent<T>(pub Result<T, Error>);

#[derive(Event, Deref, Debug)]
pub struct AskyChange<T>(T);

#[derive(Debug, Component, Default, Clone)]
pub enum AskyState {
    #[default]
    Reading,
    Complete,
    Error,
}

// #[derive(Event, Debug)]
// pub enum AskyEvent<T> {
//     Change(T),
//     Submit(T)
// }

// #[derive(Event, Deref, Debug)]
// pub struct SubmitEvent<T>(pub T);

#[derive(Debug, Component, Default, Clone)]
pub enum Submit {
    #[default]
    Repeat,
    Once,
}

/// This is a commitment to fire a `Trigger<Result<Self::Out, Error>>`.
pub unsafe trait Submitter {
    /// Output of submitter.
    type Out;
    // fn submit(&self) -> Result<Self::Out, Error>;
}

impl AskyState {
    fn is_done(&self) -> bool {
        matches!(self, AskyState::Complete | AskyState::Error)
    }
}

/// Asky errors
#[derive(Debug, thiserror::Error, Clone)]
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
    // #[error("io error {0}")]
    // Io(#[from] std::io::Error),
    #[error("channel cancel {0}")]
    Channel(#[from] oneshot::Canceled),
    // Async error
    // #[error("async error {0}")]
    // Async(#[from] bevy_defer::AccessError),
    // Promise error
    // #[error("promise error {0}")]
    // Promise(#[from] promise_out::Error),
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
