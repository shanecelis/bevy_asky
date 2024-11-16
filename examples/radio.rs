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
    let mut first = None;
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            first = Some(
                parent
                    .construct::<Radio>("Money")
                    .construct::<ascii::View>(())
                    .id(),
            );

            parent
                .construct::<Radio>("Time")
                .construct::<ascii::View>(());

            parent
                .construct::<Radio>("Power")
                .construct::<ascii::View>(());
        });
    // commands.insert_resource(Focus(first));
}
