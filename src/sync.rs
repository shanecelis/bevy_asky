#![allow(clippy::type_complexity)]
use super::*;
use crate::construct::*;
use bevy::ecs::system::EntityCommands;
use std::{marker::PhantomData, fmt::Debug};
#[cfg(feature = "async")]
use futures::{channel::oneshot, Future, TryFutureExt};

// #[derive(Deref, DerefMut)]
// pub struct Asyncable<'w,R> {
//     #[deref]
//     pub commands: EntityCommands<'w>,
//     output: PhantomData<R>,
// }

// impl<'w, R> Asyncable<'w, R> {
//     // pub fn prompt_children<
//     //     T: Construct + Bundle + Part,
//     //     V: Construct<Props = ()> + Bundle>(mut self, props:impl IntoIterator<Item = impl Into<T::Props>>) -> Self
//     //     where <T as Construct>::Props: Send + Sync + 'static
//     // {
//     //     self.commands.construct_children::<Add<T, V>>(props);
//     //     self
//     // }
// }

pub trait AskyEntityCommands<'w> {
    fn prompt<T: Construct + Bundle + Submitter>(
        self,
        props: impl Into<T::Props>,
    ) -> EntityCommands<'w>
    where
        <T as Construct>::Props: Send,
        <T as Submitter>::Out: Clone + Debug + Send + Sync;
}

pub trait AskyCommands {
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
        self
            .construct::<T>(p);
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
        commands
            .construct::<T>(p);
        commands
    }

}
