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
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2d::default());
    commands.column().with_children(|parent| {
        parent
            .column()
            .construct::<RadioGroup>("radio group 0")
            .construct_children::<Add0<Radio, View>>(["Money", "Time", "Power"])
            .observe(move |trigger: Trigger<Submit<usize>>| {
                eprintln!("trigger {:?}", trigger.event());
            });
    });
}
