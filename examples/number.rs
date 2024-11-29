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
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .construct::<View>(())
                .construct::<Number<i8>>("Age? ")
                .construct::<Placeholder>("333")
                .observe(move |trigger: Trigger<Submit<i8>>| {
                    eprintln!("trigger {:?}", trigger.event());
                });

            parent
                .construct::<View>(())
                .construct::<Number<i32>>("Phone number ? ")
                .construct::<Placeholder>("123-4567")
                .observe(move |trigger: Trigger<Submit<i8>>| {
                    eprintln!("trigger {:?}", trigger.event());
                });
        });
}
