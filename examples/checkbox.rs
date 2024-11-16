use bevy::prelude::*;
use bevy_asky::{construct::*, prompt::*, view::*, *};

fn views(app: &mut App) {
    app.add_plugins(view::ascii::plugin)
        .add_plugins(view::color::plugin);

    #[cfg(feature = "button")]
    app.add_plugins(view::button::plugin);
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
                .construct::<Checkbox>("Money?")
                .construct::<ascii::View>(())
                .observe(move |trigger: Trigger<AskyEvent<bool>>| {
                    eprintln!("trigger {:?}", trigger.event());
                });

            parent
                .construct::<Checkbox>("Time?")
                .construct::<ascii::View>(())
                .observe(move |trigger: Trigger<AskyEvent<bool>>| {
                    eprintln!("trigger {:?}", trigger.event());
                });
        });
}
