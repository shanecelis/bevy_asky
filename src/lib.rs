#![doc(html_root_url = "https://docs.rs/bevy_asky/0.2.0")]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
#![allow(clippy::type_complexity)]
use bevy::{app::PluginGroupBuilder, prelude::*};

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

/// Splat import, e.g., `use bevy_asky::prelude::*`
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
        AskyPlugin, AskySet, Dest, Error, Submit, Submitter,
    };
}

/// Asky plugin
///
/// If using "async" features, [bevy_defer]'s `AsyncPlugin` is also required.
/// Consider adding it or use [AskyPlugins] which will include it.
pub struct AskyPlugin;

/// Asky plugins
///
/// Includes [bevy_defer::AsyncPlugin] with default settings if "async" feature
/// is present.
pub struct AskyPlugins;

impl PluginGroup for AskyPlugins {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>().add(AskyPlugin);
        #[cfg(feature = "async")]
        let group = group.add(bevy_defer::AsyncPlugin::default_settings());
        group
    }
}

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
#[derive(Event, Debug, Clone)]
pub enum Submit<T> {
    /// Submit has not been handled yet.
    Unhandled(Result<T, Error>),
    /// Submit has been handled.
    Handled,
}

impl<T> Submit<T> {
    /// Create a new submission event.
    pub fn new(r: Result<T, Error>) -> Self {
        Self::Unhandled(r)
    }

    /// Unwrap the result assuming it hasn't been taken already.
    pub fn take_result(&mut self) -> Result<T, Error> {
        match std::mem::replace(self, Submit::Handled) {
            Submit::Unhandled(res) => res,
            Submit::Handled => Err(Error::SubmitHandled),
        }
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
    /// User canceled
    #[error("canceled")]
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
    /// Submit handled already failure
    #[error("submit handled")]
    SubmitHandled,
    /// Channel canceled
    #[cfg(feature = "async")]
    #[error("channel cancel {0}")]
    Channel(#[from] oneshot::Canceled),
}
