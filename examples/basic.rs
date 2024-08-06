use bevy::prelude::*;
use bevy_asky::{construct::*, prompt::*, view::*, *};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AskyPlugin))
        .add_plugins(view::ascii::plugin)
        .add_plugins(view::color::plugin)
        .add_plugins(view::button::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (text_color_system, read_keys))
        .run();
}

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

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

    commands.entity(column)
            .with_children(|parent| {
        parent
            .construct::<Confirm>("Do you like ascii?")
            .construct::<ascii::View>(())
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

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in &mut query {
        let seconds = time.elapsed_seconds();

        // Update the color of the first and only section.
        text.sections[0].style.color = Color::srgb(
            (1.25 * seconds).sin() / 2.0 + 0.5,
            (0.75 * seconds).sin() / 2.0 + 0.5,
            (0.50 * seconds).sin() / 2.0 + 0.5,
        );
    }
}

fn read_keys(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut AskyState>) {
    if input.just_pressed(KeyCode::KeyR) {
        for mut state in query.iter_mut() {
            *state = AskyState::Reading;
        }
    }
}
