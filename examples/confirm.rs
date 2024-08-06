use bevy::prelude::*;
use bevy_asky::{construct::*, prompt::*, view::*, *};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AskyPlugin))
        .add_plugins(view::ascii::plugin)
        .add_plugins(view::color::plugin)
        .add_plugins(view::button::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (read_keys))
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());

    commands
        .construct::<Confirm>("What up?")
        .construct::<ascii::View>(())
        .observe(
            move |trigger: Trigger<AskyEvent<bool>>, mut commands: Commands| {
                eprintln!("trigger {:?}", trigger.event());
            },
        );
}

fn read_keys(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut AskyState>) {
    if input.just_pressed(KeyCode::KeyR) {
        for mut state in query.iter_mut() {
            *state = AskyState::Reading;
        }
    }
}
