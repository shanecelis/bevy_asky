#![doc(html_root_url = "https://docs.rs/bevy_asky/0.1.0")]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
#![allow(clippy::type_complexity)]
use bevy::prelude::*;

pub mod focus;

#[cfg(feature = "async")]
mod r#async;
pub mod construct;
mod num_like;
pub mod prompt;
pub mod string_cursor;
pub mod view;
#[cfg(feature = "async")]
use futures::channel::oneshot;
#[cfg(feature = "async")]
pub use r#async::*;
mod dest;
pub mod sync;
pub use dest::Dest;

/// Splat import, e.g., `use bevy_asky::prelude::*`.
pub mod prelude {
    #[cfg(feature = "async")]
    pub use super::r#async::*;
    pub use super::{
        construct::*,
        focus::*,
        num_like::NumLike,
        prompt::*,
        sync::{AskyCommands, AskyEntityCommands},
        view::{widget::Widgets, *},
        Submit, AskyPlugin, AskySet, Dest, Error, Submitter,
    };
}

/// The Asky plugin. If using "async" features, [bevy_defer]'s `AsyncPlugin` is
/// also required.
pub struct AskyPlugin;

/// In the Update schedule, AskySet runs the controllers then the views.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AskySet {
    /// Process inputs and modify models
    Controller,
    /// Construct or update view components
    View,
}

impl Plugin for AskyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(prompt::plugin)
            .add_plugins(view::plugin)
            .add_plugins(focus::plugin)
            .configure_sets(Update, (AskySet::Controller, AskySet::View).chain());
        // AsyncPlugin may require a special configuration, so we're not
        // including it ourselves.

        // #[cfg(feature = "async")]
        // app
        //     .add_plugins(bevy_defer::AsyncPlugin::default_settings());
    }
}

/// Prompts trigger an Submit
///
/// [Submitter] trait on prompt defines what output type to expect.
#[derive(Event, Deref, DerefMut, Debug, Clone)]
pub struct Submit<T>(pub Option<Result<T, Error>>);

impl<T> Submit<T> {
    /// Create a new submission event.
    pub fn new(r: Result<T, Error>) -> Self {
        Self(Some(r))
    }

    /// Unwrap the result assuming it hasn't been taken already.
    pub fn take_result(&mut self) -> Result<T, Error> {
        self.0.take().expect("submit has been taken already")
    }
}


// /// Should we have a policy on submission?
// #[derive(Debug, Component, Default, Clone)]
// pub enum Submit {
//     #[default]
//     Repeat,
//     Once,
// }

/// This trait represents a commitment to fire a `Trigger<Result<Self::Out,
/// Error>>`.
///
/// # Safety
///
/// Structs that implement this trait commit to firing
/// `Trigger<Result<Self::Out, Error>>` at some point in their life cycle.
pub unsafe trait Submitter {
    /// Output of submitter.
    type Out;
}

/// A part of a group
pub trait Part {
    /// The type of the group
    type Group: Default + Component;
}

/// Asky errors
#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    /// User cancelled
    #[error("cancelled")]
    Cancel,
    /// Input was invalid
    #[error("invalid input")]
    InvalidInput,
    /// Invalid number
    #[error("invalid number")]
    InvalidNumber,
    /// Validation failed
    #[error("validation fail")]
    ValidationFail,
    /// Channel canceled
    #[cfg(feature = "async")]
    #[error("channel cancel {0}")]
    Channel(#[from] oneshot::Canceled),
}
