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

#[derive(Clone, SystemParam)]
pub struct Asky;

impl Asky {

    /// Prompt the user with `T`, rendering in element `dest`.
    pub fn prompt(
        &mut self,
        prompt: impl Into<String>,
        dest: Entity,
    ) -> impl Future<Output = Result<String, oneshot::Canceled>> {

        let (sender, receiver) = oneshot::channel::<String>();
        let p = prompt.into();
        // let async_world = AsyncWorld::new();
        // async_world.spawn_task(|| async move {

        // });
        let mut send_once = Some(sender);
        async move {
            let async_world = AsyncWorld::new();

            async_world.apply_command(move |world: &mut World| {
                let mut commands = world.commands();
                commands.entity(dest)
                    .construct::<TextField>(p)

                    .construct::<ascii::View>(())
                    .observe(move |trigger: Trigger<AskyEvent<String>>| {
                        if let Some(sender) = send_once.take() {
                            sender.send(trigger.event().0.as_ref().unwrap().to_string()).unwrap();
                            // sender.send("bye".into());
                        }
                    })
                    ;
            });
            // Ok("hi".into())
            receiver.await
        }
        // receiver
    }
}
