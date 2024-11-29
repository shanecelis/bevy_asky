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
        .construct::<View>(())
        .construct::<Confirm>("Do you like cats?")
        .observe(
            move |mut trigger: Trigger<Submit<bool>>, mut commands: Commands| {
                if let Ok(yes) = trigger.event_mut().take().unwrap() {
                    commands
                        .entity(trigger.entity())
                        .construct::<Feedback>(Feedback::info(if yes { "Me too!" } else { "Ok." }));
                }
            },
        );
}
