#![allow(clippy::type_complexity)]
use super::*;
use crate::{construct::*, view::*};
use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_defer::AsyncWorld;
use futures::{channel::oneshot, Future};
use std::fmt::Debug;

#[derive(Clone, SystemParam)]
pub struct Asky;

impl Asky {
    /// Prompt the user with `T`, rendering in element `dest`.
    pub fn prompt<T: Construct + Component + Submitter>(
        &mut self,
        props: impl Into<T::Props>,
        dest: impl Into<Option<Entity>>,
    ) -> impl Future<Output = Result<T::Out, Error>>
    where
        <T as Construct>::Props: Send,
        <T as Submitter>::Out: Clone + Debug + Send + Sync,
    {
        let (sender, receiver) = oneshot::channel::<Result<T::Out, Error>>();
        let p = props.into();
        let d = dest.into();

        // let async_world = AsyncWorld::new();
        // async_world.spawn_task(|| async move {

        // });
        let mut send_once = Some(sender);
        async move {
            let async_world = AsyncWorld::new();

            async_world.apply_command(move |world: &mut World| {
                let mut commands = world.commands();
                let mut entity_commands = match d {
                    Some(id) => commands.entity(id),
                    None => commands.spawn_empty(),
                };
                entity_commands
                    .construct::<T>(p)
                    .construct::<ascii::View>(())
                    .observe(
                        move |trigger: Trigger<AskyEvent<T::Out>>, mut commands: Commands| {
                            if let Some(sender) = send_once.take() {
                                sender.send(trigger.event().0.clone()).expect("send");
                                // sender.send("bye".into());
                            }
                            commands.entity(trigger.entity()).despawn_recursive();
                        },
                    );
            });
            // Ok("hi".into())
            receiver.await?
            // match receiver.await {
            //     Ok(x) => x,
            //     Err(e) => warn!("One shot channel canceled: {e}")
            // }
        }
        // receiver
    }
}
