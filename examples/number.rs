use bevy::prelude::*;
use bevy_asky::{construct::*, prompt::*, view::*, *};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AskyPlugin))
        .add_plugins(view::ascii::plugin)
        .add_plugins(view::color::plugin)
        .add_plugins(view::button::plugin)
        .add_systems(Startup, (setup, add_marker).chain())
        .add_systems(Update, (text_color_system, read_keys))
        .run();
}
// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .construct::<Number<i8>>("Age? ")
                .construct::<Placeholder>("333")
                .construct::<ascii::View>(())
                .observe(move |trigger: Trigger<AskyEvent<i8>>| {
                    eprintln!("trigger {:?}", trigger.event());
                });

            parent
                .construct::<Number<i32>>("Phone number ? ")
                .construct::<Placeholder>("123-4567")
                .construct::<ascii::View>(())
                .observe(move |trigger: Trigger<AskyEvent<i8>>| {
                    eprintln!("trigger {:?}", trigger.event());
                });
        });
}

fn add_marker(query: Query<&Children, With<StringCursor>>, mut commands: Commands) {
    for children in query.iter() {
        for (i, child) in children.into_iter().enumerate() {
            if i >= 2 {
                commands.entity(*child).insert(ColorText);
            }
        }
    }
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
