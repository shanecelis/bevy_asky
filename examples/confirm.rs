use bevy::prelude::*;
use bevy_asky::{construct::*, prompt::*, view::*, *};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AskyPlugin))
        .add_plugins(view::ascii::plugin)
        .add_plugins(view::color::plugin)
        .add_plugins(view::button::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, read_keys)
        .run();
}

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
        .construct::<Confirm>("Do you like soda?")
        .construct::<ascii::View>(())
        .observe(
            move |trigger: Trigger<AskyEvent<bool>>, commands: Commands| {
                eprintln!("trigger {:?}", trigger.event());
                // commands.entity(trigger.entity()).remove::<Focusable>();
            },
        );

    parent
        .construct::<Confirm>("Do you like coke?")
        .construct::<ascii::View>(())
        .observe(
            move |trigger: Trigger<AskyEvent<bool>>, commands: Commands| {
                eprintln!("trigger {:?}", trigger.event());
                // commands.entity(trigger.entity()).remove::<Focusable>();
            },
        );

    parent
        .construct::<Confirm>("Do you like pepsi?")
        .construct::<ascii::View>(())
        .observe(
            move |trigger: Trigger<AskyEvent<bool>>, commands: Commands| {
                eprintln!("trigger {:?}", trigger.event());
                // commands.entity(trigger.entity()).remove::<Focusable>();
            },
        );
        });
}

fn read_keys(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut AskyState>) {
    if input.just_pressed(KeyCode::KeyR) {
        for mut state in query.iter_mut() {
            *state = AskyState::Reading;
        }
    }
}
