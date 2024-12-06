use bevy::prelude::*;
use bevy_asky::prelude::*;

#[path = "../common/lib.rs"]
mod common;
use common::View;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AskyPlugin))
        .add_plugins(common::views)
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    commands.column().with_children(|parent| {
        parent
            .spawn(NodeBundle::default())
            .construct::<CheckboxGroup>("checkbox group 0")
            .construct_children::<Add0<Checkbox, View>>(["Money", "Time", "Power"])
            .observe(move |trigger: Trigger<Submit<Vec<bool>>>| {
                eprintln!("trigger {:?}", trigger.event());
            });

        parent
            .column()
            .construct::<CheckboxGroup>("checkbox group 1")
            .construct_children::<Add0<Checkbox, View>>(["Money", "Time", "Power"])
            .observe(move |trigger: Trigger<Submit<Vec<bool>>>| {
                eprintln!("trigger {:?}", trigger.event());
            });
    });
}
