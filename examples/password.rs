use bevy::prelude::*;
use bevy_asky::prelude::*;

#[path = "common/lib.rs"]
mod common;
use common::View;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AskyPlugin))
        .add_plugins(common::views)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());

    commands
        .column()
        .with_children(|parent| {
            parent
                .construct::<View>(())
                .construct::<Password>("Password: ")
                .observe(move |trigger: Trigger<AskyEvent<String>>| {
                    eprintln!("trigger {:?}", trigger.event());
                });
        });
}
