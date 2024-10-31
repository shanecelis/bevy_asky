use bevy::prelude::*;
use bevy_asky::{construct::*, prompt::*, view::*, *};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AskyPlugin))
        .add_plugins(view::ascii::plugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    commands
        .construct::<Confirm>("Do you like cats?")
        .construct::<ascii::View>(())
        .observe(
            move |trigger: Trigger<AskyEvent<bool>>, mut commands: Commands| {
                if let AskyEvent(Ok(yes)) = trigger.event() {
                    commands.entity(trigger.entity())
                            .construct::<Feedback>(Feedback::info(if *yes {
                                "\nMe too!"
                            } else {
                                "\nOk."
                            }));
                }
            },
        );
}
