// #![feature(round_char_boundary)]
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
pub mod prelude {
    #[cfg(feature = "async")]
    pub use super::r#async::*;
    pub use super::{
        construct::*, focus::*, num_like::NumLike, prompt::*, view::{*, widget::Widgets}, AskyEvent,
        AskyPlugin, Error, Submitter, AskySet, Dest};
}

/// The Asky plugin. If using "async" features, [bevy_defer]'s `AsyncPlugin` is
/// also required.
pub struct AskyPlugin;

/// Asky runs in the Update schedule of sets in this order where
/// necessary:
///
/// - Controller, process inputs and modify models.
/// - PreReplaceView, empty, can be used to pre-empt default replace view.
/// - ReplaceView, look for `NeedsView` components and add current ones.
/// - View, construct or update associated view components.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AskySet {
    Controller,
    View,
}

impl Plugin for AskyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(prompt::plugin)
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


#[derive(Event, Deref, DerefMut, Debug, Clone)]
pub struct AskyEvent<T>(pub Result<T, Error>);

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

/// A part of a group.
pub trait Part {
    type Group: Default + Component;
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
    /// Invalid number.
    #[error("invalid number")]
    InvalidNumber,
    /// Validation failed.
    #[error("validation fail")]
    ValidationFail,

    #[cfg(feature = "async")]
    #[error("channel cancel {0}")]
    Channel(#[from] oneshot::Canceled),
}
