use bevy::prelude::*;
use bevy_asky::{construct::*, prompt::*, view::*, *};

fn views(app: &mut App) {
    app
        .add_plugins(view::ascii::plugin)
        .add_plugins(view::color::plugin);

    #[cfg(feature = "button")]
    app
        .add_plugins(view::button::plugin);
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AskyPlugin))
        .add_plugins(views)
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
    // commands
    //     .construct::<Confirm>("What up?")
    //     .construct::<ascii::View>(());

    commands.entity(column).with_children(|parent| {
        parent
            .construct::<Confirm>("Do you like ascii?")
            .construct::<ascii::View>(())
            // .construct::<color::View>(())
            // .construct::<button::View>(())
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

                        parent
                            .construct::<Confirm>("Do you prefer color?")
                            // .construct::<color::View>(());
                            .construct::<ascii::View>(());
                    });
                },
            );
    });
}
