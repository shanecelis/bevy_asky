#![allow(clippy::type_complexity)]
use bevy::{
    prelude::*,
    ecs::system::SystemParam,

};
use futures::{
    channel::oneshot,
    Future,
};
use bevy_defer::AsyncWorld;
use bevy_ui_navigation::{prelude::*, systems::InputMapping};
use super::*;
use crate::{prompt::*, view::*, construct::*};
use std::fmt::Debug;

#[derive(Clone, SystemParam)]
pub struct Asky;

impl Asky {

    /// Prompt the user with `T`, rendering in element `dest`.
    pub fn prompt<T: Construct + Component + Submitter>(
        &mut self,
        props: impl Into<T::Props>,
        dest: Entity,
    ) -> impl Future<Output = Result<T::Out, Error>>
    where <T as Construct>::Props: Send,
    <T as Submitter>::Out: Clone + Debug + Send + Sync
    {

        let (sender, receiver) = oneshot::channel::<Result<T::Out, Error>>();
        let p = props.into();
        // let async_world = AsyncWorld::new();
        // async_world.spawn_task(|| async move {

        // });
        let mut send_once = Some(sender);
        async move {
            let async_world = AsyncWorld::new();

            async_world.apply_command(move |world: &mut World| {
                let mut commands = world.commands();
                commands.entity(dest)
                    .construct::<T>(p)

                    .construct::<ascii::View>(())
                    .observe(move |trigger: Trigger<AskyEvent<Result<T::Out, Error>>>| {
                        if let Some(sender) = send_once.take() {
                            sender.send(trigger.event().0.as_ref().expect("out").clone()).expect("send");
                            // sender.send("bye".into());
                        }
                    })
                    ;
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
