use bevy::prelude::*;
use bevy_asky::{construct::*, prompt::*, view::*, *};

#[path = "common/lib.rs"]
mod common;

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
                .construct::<Checkbox>("Money?")
                .observe(move |trigger: Trigger<AskyEvent<bool>>| {
                    eprintln!("trigger {:?}", trigger.event());
                });

            parent
                .construct::<Checkbox>("Time?")
                .observe(move |trigger: Trigger<AskyEvent<bool>>| {
                    eprintln!("trigger {:?}", trigger.event());
                });
        });
}
