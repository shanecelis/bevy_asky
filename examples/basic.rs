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

    let column = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .id();
    commands.entity(column).with_children(|parent| {
        parent
            .construct::<View>(())
            .construct::<Confirm>("Do you like ascii?")
            .observe(
                move |trigger: Trigger<AskyEvent<bool>>, mut commands: Commands| {
                    eprintln!("trigger {:?}", trigger.event());
                    let answer = trigger.event().as_ref().unwrap_or(&false);
                    commands.entity(column).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            if *answer {
                                "Me too."
                            } else {
                                "We have other options."
                            },
                            TextStyle::default(),
                        ));

                        parent.construct::<Confirm>("Do you prefer color?");
                    });
                },
            );
    });
}
