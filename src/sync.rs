#![allow(clippy::type_complexity)]
use super::*;
use crate::construct::*;
use bevy::ecs::system::EntityCommands;
use std::{marker::PhantomData, fmt::Debug};
use futures::{channel::oneshot, Future, TryFutureExt};

#[derive(Deref, DerefMut)]
pub struct Asyncable<'w,T> {
    #[deref]
    commands: EntityCommands<'w>,
    output: PhantomData<T>,
}

impl<'w, T> Asyncable<'w, T> {
    pub fn as_future(&mut self, sender: oneshot::Sender<T>) -> impl Future<Output = Result<T, Error>>
    where T: Clone + Debug + Send + Sync + 'static {
        let mut send_once = Some(sender);
        self.commands.observe(
            move |trigger: Trigger<AskyEvent<T>>, mut commands: Commands| {
                if let Some(sender) = send_once.take() {
                    sender.send(trigger.event().0.clone()).expect("send");
                }
                // TODO: This should be the result of some policy not de facto.
                // commands.entity(trigger.entity()).despawn_recursive();
            },
        );
        async move {
            receiver.await?
        }
    }
}

pub trait AskyCommands {
    fn prompt<
        T: Construct + Bundle + Submitter,
        V: Construct<Props = ()> + Bundle>(
        &mut self,
        props: impl Into<T::Props>,
        dest: impl Into<Dest>,
    ) -> Asyncable<'_, T::Out>
    where
        <T as Construct>::Props: Send,
        <T as Submitter>::Out: Clone + Debug + Send + Sync;

    fn prompt_group<
        T: Construct + Bundle + Part, V: Construct<Props = ()> + Bundle>(
        &mut self,
        group_prop: impl Into<<<T as Part>::Group as Construct>::Props>,
        props: impl IntoIterator<Item = impl Into<T::Props>>,
        dest: impl Into<Dest>,
    ) -> EntityCommands
    where
        <T as Construct>::Props: Send,
        <<T as Part>::Group as Construct>::Props: Send,
        <T as Part>::Group: Bundle + Construct + Send + Sync,
        <T as Part>::Group: Submitter,
        <<T as Part>::Group as Submitter>::Out: Clone + Debug + Send + Sync;
}

impl<'w, 's> AskyCommands for Commands<'w, 's> {
    fn prompt<
        T: Construct + Bundle + Submitter,
        V: Construct<Props = ()> + Bundle,
    >(
        &mut self,
        props: impl Into<T::Props>,
        dest: impl Into<Dest>,
    ) -> Asyncable<'_, T::Out>
    where
        <T as Construct>::Props: Send,
        <T as Submitter>::Out: Clone + Debug + Send + Sync,
    {
        let p = props.into();
        let d = dest.into();

        let mut commands = d.entity(self);
        commands
            .construct::<V>(())
            .construct::<T>(p);
        Asyncable { commands, output: PhantomData }
    }

    fn prompt_group<T: Construct + Bundle + Part,
                    V: Construct<Props = ()> + Bundle>(
        &mut self,
        group_prop: impl Into<<<T as Part>::Group as Construct>::Props>,
        props: impl IntoIterator<Item = impl Into<T::Props>>,
        dest: impl Into<Dest>,
    ) -> EntityCommands
    where
        <T as Construct>::Props: Send,
        <<T as Part>::Group as Construct>::Props: Send,
        <T as Part>::Group: Bundle + Construct + Send + Sync + Submitter,
        <<T as Part>::Group as Submitter>::Out: Clone + Debug + Send + Sync {
        let d = dest.into();

        let mut commands = d.entity(self);
        commands
            .construct::<V>(())
            .construct::<T::Group>(group_prop)
            .with_children(|parent| {
                for prop in props.into_iter() {
                    parent
                        .construct::<V>(())
                        .construct::<T>(prop);
                }
            });
        commands
    }
}
