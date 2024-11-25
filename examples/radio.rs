//! [Radio] don't submit values. Use [RadioGroup] to submit.
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
                .construct::<Radio>("Money");
            parent
                .construct::<View>(())
                .construct::<Radio>("Time");
            parent
                .construct::<View>(())
                .construct::<Radio>("Power");
        });
}
