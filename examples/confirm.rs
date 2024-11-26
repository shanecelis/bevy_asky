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
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
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
                .construct::<Confirm>("Do you like soda?")
                .observe(move |trigger: Trigger<AskyEvent<bool>>| {
                    eprintln!("trigger {:?}", trigger.event());
                });

            parent
                .construct::<View>(())
                .construct::<Confirm>("Do you like coke?")
                .observe(move |trigger: Trigger<AskyEvent<bool>>| {
                    eprintln!("trigger {:?}", trigger.event());
                });
            parent
                .construct::<View>(())
                .construct::<Confirm>("Do you like pepsi?")
                .observe(move |trigger: Trigger<AskyEvent<bool>>| {
                    eprintln!("trigger {:?}", trigger.event());
                });
        });
}
