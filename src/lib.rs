#![doc(html_root_url = "https://docs.rs/bevy_asky/0.5.0")]
#![doc = include_str!("../README.md")]
// #![forbid(missing_docs)]
#![allow(clippy::type_complexity)]
use bevy::{
    app::PluginGroupBuilder,
    input_focus::{
        FocusCause, InputFocus,
        tab_navigation::{NavAction, TabGroup, TabIndex, TabNavigation, TabNavigationPlugin},
    },
    prelude::*,
};
use std::sync::atomic::{AtomicI32, Ordering};

#[cfg(feature = "async")]
mod r#async;
pub mod construct;
mod num_like;
pub mod prompt;
pub mod string_cursor;
pub mod view;
#[cfg(feature = "async")]
pub use r#async::*;
#[cfg(feature = "async")]
use futures::channel::oneshot;
mod dest;
pub mod sync;
pub use dest::Dest;

/// Splat import, e.g., `use bevy_asky::prelude::*`
pub mod prelude {
    #[cfg(feature = "async")]
    pub use super::r#async::*;
    pub use super::{
        AskyPlugin, AskySet, Dest, Error, Submit, Submitter,
        construct::*,
        next_tab_index,
        num_like::NumLike,
        prompt::*,
        sync::{AskyCommands, AskyEntityCommands},
        view::{widget::Widgets, *},
    };
    pub use bevy::input_focus::{
        FocusCause, InputFocus, InputFocusPlugin, InputFocusVisible,
        tab_navigation::{NavAction, TabGroup, TabIndex, TabNavigation, TabNavigationPlugin},
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

static TAB_INDEX_COUNTER: AtomicI32 = AtomicI32::new(0);

/// Return the next Bevy tab index for an Asky prompt.
pub fn next_tab_index() -> TabIndex {
    TabIndex(TAB_INDEX_COUNTER.fetch_add(1, Ordering::Relaxed))
}

impl Plugin for AskyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TabNavigationPlugin)
            .add_plugins(prompt::plugin)
            .add_plugins(view::plugin)
            .configure_sets(Update, (AskySet::Controller, AskySet::View).chain())
            .add_systems(
                PreUpdate,
                (
                    ensure_tab_groups,
                    initialize_focus,
                    arrow_key_navigation.after(ensure_tab_groups),
                ),
            );
        // AsyncPlugin may require a special configuration, so we're not
        // including it ourselves.

        // #[cfg(feature = "async")]
        // app
        //     .add_plugins(bevy_defer::AsyncPlugin::default_settings());
    }
}

fn ensure_tab_groups(
    mut commands: Commands,
    focusables: Query<&ChildOf, Added<TabIndex>>,
    tab_groups: Query<(), With<TabGroup>>,
) {
    for child_of in &focusables {
        let parent = child_of.parent();
        if !tab_groups.contains(parent) {
            commands.entity(parent).try_insert(TabGroup::new(0));
        }
    }
}

fn initialize_focus(
    mut input_focus: ResMut<InputFocus>,
    focusables: Query<(Entity, &TabIndex), Without<TabGroup>>,
) {
    if input_focus.get().is_some_and(|entity| {
        focusables
            .get(entity)
            .is_ok_and(|(_, tab_index)| tab_index.0 >= 0)
    }) {
        return;
    }

    if let Some((entity, _)) = focusables
        .iter()
        .filter(|(_, tab_index)| tab_index.0 >= 0)
        .min_by_key(|(_, tab_index)| tab_index.0)
    {
        input_focus.set(entity, FocusCause::Navigated);
    }
}

fn arrow_key_navigation(
    input: Res<ButtonInput<KeyCode>>,
    nav: TabNavigation,
    mut input_focus: ResMut<InputFocus>,
    text_inputs: Query<(), With<string_cursor::StringCursor>>,
) {
    let action = if input.any_just_pressed([KeyCode::ArrowDown, KeyCode::ArrowRight]) {
        NavAction::Next
    } else if input.any_just_pressed([KeyCode::ArrowUp, KeyCode::ArrowLeft]) {
        NavAction::Previous
    } else {
        return;
    };

    if input_focus
        .get()
        .is_some_and(|entity| text_inputs.contains(entity))
    {
        return;
    }

    if let Ok(next) = nav.navigate(&input_focus, action) {
        input_focus.set(next, FocusCause::Navigated);
    }
}

pub(crate) fn move_focus_from(nav: &TabNavigation, input_focus: &mut InputFocus) {
    match nav.navigate(input_focus, NavAction::Next) {
        Ok(next) => input_focus.set(next, FocusCause::Navigated),
        Err(_) => input_focus.clear(),
    }
}

pub(crate) fn block_and_move_focus(
    commands: &mut Commands,
    id: Entity,
    nav: &TabNavigation,
    input_focus: &mut InputFocus,
) {
    move_focus_from(nav, input_focus);
    commands.entity(id).try_insert(TabIndex(-1));
}

/// Prompts trigger an Submit
///
/// [Submitter] trait on prompt defines what output type to expect.
#[derive(EntityEvent, Debug, Clone)]
pub struct Submit<T> {
    /// The entity that triggered this submit event.
    pub entity: Entity,
    /// The state of the submit event.
    pub state: SubmitState<T>,
}

#[derive(Debug, Clone)]
pub enum SubmitState<T> {
    /// Submit has not been handled yet.
    Unhandled(Result<T, Error>),
    /// Submit has been handled.
    Handled,
}

impl<T> Submit<T> {
    /// Create a new submission event.
    pub fn new(entity: Entity, r: Result<T, Error>) -> Self {
        Self {
            entity,
            state: SubmitState::Unhandled(r),
        }
    }

    /// Unwrap the result assuming it hasn't been taken already.
    pub fn take_result(&mut self) -> Result<T, Error> {
        match std::mem::replace(&mut self.state, SubmitState::Handled) {
            SubmitState::Unhandled(res) => res,
            SubmitState::Handled => Err(Error::SubmitHandled),
        }
    }

    /// Get the event (for observers).
    pub fn event(&self) -> &SubmitState<T> {
        &self.state
    }

    /// Get the event mutably (for observers).
    pub fn event_mut(&mut self) -> &mut SubmitState<T> {
        &mut self.state
    }
}

// /// Should we have a policy on submission?
// #[derive(Debug, Component, Default, Clone)]
// pub enum Submit {
//     #[default]
//     Repeat,
//     Once,
// }

/// This trait represents a commitment to fire a `On<Result<Self::Out,
/// Error>>`.
///
/// # Safety
///
/// Structs that implement this trait commit to firing
/// `On<Result<Self::Out, Error>>` at some point in their life cycle.
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
