#![allow(clippy::type_complexity)]
use super::*;
use crate::{construct::*, view::*};
use bevy::{ecs::system::{EntityCommands, SystemParam}, prelude::*};
use std::fmt::Debug;

pub trait AskyCommands {
    fn prompt<T: Construct + Component + Submitter,
              V: Construct<Props = ()> + Component + Default>(
        &mut self,
        props: impl Into<T::Props>,
        dest: impl Into<Dest>,
    ) -> EntityCommands
    where
        <T as Construct>::Props: Send,
        <T as Submitter>::Out: Clone + Debug + Send + Sync;
}

impl<'w, 's> AskyCommands for Commands<'w, 's> {
    fn prompt<T: Construct + Component + Submitter, V: Construct<Props = ()> + Component + Default>(
        &mut self,
        props: impl Into<T::Props>,
        dest: impl Into<Dest>,
    ) -> EntityCommands
    where
        <T as Construct>::Props: Send,
        <T as Submitter>::Out: Clone + Debug + Send + Sync,
    {
        use super::Dest::*;
        let p = props.into();
        let d = dest.into();

        let mut commands = d.entity_commands(self);
            commands.construct::<V>(())
            .construct::<T>(p);
        commands
    }
}

