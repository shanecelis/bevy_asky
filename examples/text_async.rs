use bevy::prelude::*;
use bevy_asky::prelude::*;
use bevy_defer::{AsyncCommandsExtension, AsyncPlugin};

#[path = "common/lib.rs"]
mod common;
use common::View;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AskyPlugin, AsyncPlugin::default_settings()))
        .add_plugins(common::views)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut asky: Asky) {
    // UI camera
    commands.spawn(Camera2dBundle::default());

    // TODO: This one is still not right. Focus doesn't move down.
    let id = commands
        .column()
        .id();
    commands.spawn_task(move || async move {
        let response: Result<String, Error> = asky
            .prompt::<TextField, View>("What up? ", Dest::Append(id))
            .await;
        dbg!(response);

        let response: Result<String, Error> = asky
            .prompt::<TextField, View>("Really? ", Dest::Append(id))
            .await;
        dbg!(response);
        Ok(())
    });
}
