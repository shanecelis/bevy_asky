#![allow(clippy::type_complexity)]
use super::*;
use crate::{construct::*, sync::AskyCommands, view::*};
use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_defer::AsyncWorld;
use futures::{channel::oneshot, Future};
use std::fmt::Debug;

#[derive(Clone, SystemParam, Default)]
pub struct Asky;

impl Asky {
    /// Prompt the user with `T`, rendering in element `dest`.
    pub fn prompt<T: Construct + Component + Submitter>(
        &mut self,
        props: impl Into<T::Props>,
        dest: impl Into<Dest>,
    ) -> impl Future<Output = Result<T::Out, Error>>
    where
        <T as Construct>::Props: Send,
        <T as Submitter>::Out: Clone + Debug + Send + Sync,
    {
        use Dest::*;
        let (sender, receiver) = oneshot::channel::<Result<T::Out, Error>>();
        let p = props.into();
        let d = dest.into();

        let mut send_once = Some(sender);
        async move {
            let async_world = AsyncWorld::new();
            async_world.apply_command(move |world: &mut World| {
                let mut commands = world.commands();
                commands
                    .prompt::<T>(p, d)
                    .observe(
                        move |trigger: Trigger<AskyEvent<T::Out>>, mut commands: Commands| {
                            if let Some(sender) = send_once.take() {
                                sender.send(trigger.event().0.clone()).expect("send");
                            }
                            // TODO: This should be the result of some policy not de facto.
                            // commands.entity(trigger.entity()).despawn_recursive();
                        },
                    );
            });
            receiver.await?
        }
    }
}
