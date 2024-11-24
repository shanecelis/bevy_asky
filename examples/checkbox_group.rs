use bevy::prelude::*;
use bevy_asky::{
    construct::*,
    prompt::*,
    view::{widget::Widgets, *},
    *,
};

#[path = "common/lib.rs"]
mod common;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AskyPlugin))
        .add_plugins(common::views)
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    commands.column().with_children(|parent| {
        parent.spawn(TextBundle::from("checkbox group 0"));
        parent
            // .column()
            .spawn(NodeBundle::default())
            .construct_children::<Checkbox>(["Money", "Time", "Power"])
            .observe(move |trigger: Trigger<AskyEvent<Vec<bool>>>| {
                eprintln!("trigger {:?}", trigger.event());
            });

        parent.spawn(TextBundle::from("checkbox group 1"));
        parent
            .column()
            .construct_children::<Checkbox>(["Money", "Time", "Power"])
            .observe(move |trigger: Trigger<AskyEvent<Vec<bool>>>| {
                eprintln!("trigger {:?}", trigger.event());
            });
    });
}
