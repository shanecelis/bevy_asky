use bevy::prelude::*;
use bevy_asky::{
    construct::*,
    prompt::*,
    view::{widget::Widgets, *},
    *,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AskyPlugin))
        .add_plugins(view::ascii::plugin)
        .add_plugins(view::color::plugin)
        .add_plugins(view::button::plugin)
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, read_keys)
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    let root = commands.column().with_children(|parent| {
        parent.spawn(TextBundle::from("checkbox group 0"));

        parent
            .construct::<color::View>(())
            .construct::<CheckboxGroup>(vec!["Money".into(), "Time".into(), "Power".into()])
            .observe(
                move |trigger: Trigger<AskyEvent<Vec<bool>>>, commands: Commands| {
                    eprintln!("trigger {:?}", trigger.event());
                },
            );

        parent.spawn(TextBundle::from("checkbox group 1"));
        parent
            .construct::<ascii::View>(())
            .construct::<CheckboxGroup>(vec!["Money".into(), "Time".into(), "Power".into()])
            .observe(
                move |trigger: Trigger<AskyEvent<Vec<bool>>>, commands: Commands| {
                    eprintln!("trigger {:?}", trigger.event());
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
