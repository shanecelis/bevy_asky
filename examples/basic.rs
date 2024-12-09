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
    commands.spawn(Camera2d);

    let column = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .id();
    commands.entity(column).with_children(|parent| {
        parent
            .construct::<View>(())
            .construct::<Confirm>("Do you like ascii?")
            .observe(
                move |mut trigger: Trigger<Submit<bool>>, mut commands: Commands| {
                    eprintln!("trigger {:?}", trigger.event());
                    let answer = trigger.event_mut().take_result().unwrap_or(false);
                    commands.entity(column).with_children(|parent| {
                        parent.spawn(Text::new(if answer {
                            "Me too."
                        } else {
                            "We have other options."
                        }));
                        parent
                            .construct::<View>(())
                            .construct::<Confirm>("Do you prefer color?");
                    });
                    commands.entity(trigger.entity()).despawn();
                },
            );
    });
}
