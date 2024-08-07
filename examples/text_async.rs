use bevy::prelude::*;
use bevy_asky::{construct::*, prompt::*, view::*, *};
use bevy_defer::{AsyncPlugin, AsyncCommandsExtension};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AskyPlugin, AsyncPlugin::default_settings()))
        .add_plugins(view::ascii::plugin)
        .add_plugins(view::color::plugin)
        .add_plugins(view::button::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (read_keys))
        .run();
}

fn setup(mut commands: Commands, mut asky: Asky) {
    // UI camera
    commands.spawn(Camera2dBundle::default());

    let id = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }).id();
    commands.spawn_task(move || async move {
        let response = asky.prompt("What up?", id).await;
        dbg!(response);
        Ok(())
    });

}

fn read_keys(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut AskyState>) {
    if input.just_pressed(KeyCode::KeyR) {
        for mut state in query.iter_mut() {
            *state = AskyState::Reading;
        }
    }
}
