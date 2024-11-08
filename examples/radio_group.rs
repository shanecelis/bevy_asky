use bevy::prelude::*;
use bevy_asky::{
    construct::*,
    prompt::*,
    view::{widget::Widgets, *},
    *,
};

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
    commands.column().with_children(|parent| {
        parent
            .construct::<Prompt>("radio group 0")
            .construct::<ascii::View>(());
        parent
            .construct::<color::View>(())
            .construct::<RadioGroup>(vec!["Money".into(), "Time".into(), "Power".into()])
            .observe(
                move |trigger: Trigger<AskyEvent<usize>>| {
                    eprintln!("trigger {:?}", trigger.event());
                },
            );

        parent
            .construct::<Prompt>("radio group 1")
            .construct::<ascii::View>(());
        parent
            .construct::<ascii::View>(())
            .construct::<RadioGroup>(vec!["Money".into(), "Time".into(), "Power".into()])
            .observe(
                move |trigger: Trigger<AskyEvent<usize>>| {
                    eprintln!("trigger {:?}", trigger.event());
                },
            );
    });
}
