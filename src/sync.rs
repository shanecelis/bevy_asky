#![allow(clippy::type_complexity)]
use super::*;
use crate::{construct::*, view::*};
use bevy::{ecs::system::{EntityCommands, SystemParam}, prelude::*};
use bevy_defer::AsyncWorld;
use futures::{channel::oneshot, Future};
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
        use Dest::*;
        let p = props.into();
        let d = dest.into();

        let mut commands = d.entity_commands(self);
            commands.construct::<V>(())
            .construct::<T>(p);
        commands
    }
}

// impl Asky {
//     /// Prompt the user with `T`, rendering in element `dest`.
//     pub fn prompt<T: Construct + Component + Submitter, V: Construct<Props = ()> + Component + Default>(
//         &mut self,
//         props: impl Into<T::Props>,
//         dest: impl Into<Dest>,
//     ) -> EntityCommands
//     where
//         <T as Construct>::Props: Send,
//         <T as Submitter>::Out: Clone + Debug + Send + Sync,
//     {
//         use Dest::*;
//         let p = props.into();
//         let d = dest.into();

//             async_world.apply_command(move |world: &mut World| {
//                 let mut commands = world.commands();
//                 d.entity_commands(&mut commands)
//                     .construct::<V>(())
//                     .construct::<T>(p)
//                     .observe(
//                         move |trigger: Trigger<AskyEvent<T::Out>>, mut commands: Commands| {
//                             if let Some(sender) = send_once.take() {
//                                 sender.send(trigger.event().0.clone()).expect("send");
//                             }

//                             // TODO: This should be the result of some policy not de facto.
//                             // commands.entity(trigger.entity()).despawn_recursive();
//                         },
//                     );
//             });
//             receiver.await?
//         }
//     }
// }
