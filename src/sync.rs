//! Uses triggers to communicate results
#![allow(clippy::type_complexity)]
use super::*;
use crate::construct::*;
use bevy::ecs::system::EntityCommands;
use std::fmt::Debug;

/// EntityCommands extension for constructing prompts.
pub trait AskyEntityCommands<'w> {
    /// Construct prompt that will issue a trigger.
    fn prompt<T: Construct + Bundle + Submitter>(
        self,
        props: impl Into<T::Props>,
    ) -> EntityCommands<'w>
    where
        <T as Construct>::Props: Send,
        <T as Submitter>::Out: Clone + Debug + Send + Sync;
}

/// Commands extension for constructing prompts.
pub trait AskyCommands {
    /// Construct prompt that will issue a trigger.
    fn prompt<T: Construct + Bundle + Submitter>(
        &mut self,
        props: impl Into<T::Props>,
        dest: impl Into<Dest>,
    ) -> EntityCommands
    where
        <T as Construct>::Props: Send,
        <T as Submitter>::Out: Clone + Debug + Send + Sync;
}

impl<'w> AskyEntityCommands<'w> for EntityCommands<'w> {
    fn prompt<T: Construct + Bundle + Submitter>(
        mut self,
        props: impl Into<T::Props>,
    ) -> EntityCommands<'w>
    where
        <T as Construct>::Props: Send,
        <T as Submitter>::Out: Clone + Debug + Send + Sync,
    {
        let p = props.into();
        self.construct::<T>(p);
        self
    }
}

impl<'w, 's> AskyCommands for Commands<'w, 's> {
    fn prompt<T: Construct + Bundle + Submitter>(
        &mut self,
        props: impl Into<T::Props>,
        dest: impl Into<Dest>,
    ) -> EntityCommands
    where
        <T as Construct>::Props: Send,
        <T as Submitter>::Out: Clone + Debug + Send + Sync,
    {
        let p = props.into();
        let d = dest.into();

        let mut commands = d.entity(self);
        commands.construct::<T>(p);
        commands
    }
}
