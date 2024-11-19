#![allow(clippy::type_complexity)]
use super::*;
use crate::construct::*;
use bevy::ecs::system::EntityCommands;
use std::fmt::Debug;

pub trait AskyCommands {
    fn prompt<
        T: Construct + Component + Submitter,
    >(
        &mut self,
        props: impl Into<T::Props>,
        dest: impl Into<Dest>,
    ) -> EntityCommands
    where
        <T as Construct>::Props: Send,
        <T as Submitter>::Out: Clone + Debug + Send + Sync;

    fn prompt_group<
        T: Construct + Component + Submitter + Part,
        X>(
        &mut self,
        group_prop: impl Into<<<T as Part>::Group as Construct>::Props>,
        props: impl IntoIterator<Item = X>,
        dest: impl Into<Dest>,
    ) -> EntityCommands
    where
        <T as Construct>::Props: Send,
        <<T as Part>::Group as Construct>::Props: Send,
        X: Into<T::Props>,
        <T as Part>::Group: Component + Construct + Clone + Send + Sync,
        <T as Submitter>::Out: Clone + Debug + Send + Sync;
}

impl<'w, 's> AskyCommands for Commands<'w, 's> {
    fn prompt<
        T: Construct + Component + Submitter,
    >(
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

    fn prompt_group<
        T: Construct + Component + Submitter + Part,
        X>(
        &mut self,
        group_prop: impl Into<<<T as Part>::Group as Construct>::Props>,
        props: impl IntoIterator<Item = X>,
        dest: impl Into<Dest>,
    ) -> EntityCommands
    where
        <T as Construct>::Props: Send,
        <<T as Part>::Group as Construct>::Props: Send,
        X: Into<T::Props>,
        <T as Part>::Group: Component + Construct + Clone + Send + Sync,
        <T as Submitter>::Out: Clone + Debug + Send + Sync {
        let d = dest.into();

        let mut commands = d.entity(self);
        commands
            .construct::<T::Group>(group_prop)
            .with_children(|parent| {
                for prop in props.into_iter() {
                    parent.construct::<T>(prop);
                }
            });
        commands
    }
}
