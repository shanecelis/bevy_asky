use bevy::prelude::*;
use bevy_asky::{construct::*, prompt::*, view::{*, widget::Widgets}, *, };
use bevy_alt_ui_navigation_lite::prelude::*;

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
    let mut next_group = None;
    let root = commands
        .column()
        .insert(MenuBuilder::Root)
        .insert(Focusable::default())
        .id();
    commands.entity(root)
        .with_children(|parent| {

            next_group = Some(parent
                              .construct::<Prompt>("checkbox group 0")
                              .insert(Focusable::default())

                              .construct::<ascii::View>(())
                              .id());
            parent
                .construct::<ascii::View>(())
                .construct::<CheckboxGroup>(vec!["Money".into(), "Time".into(), "Power".into()])
                .insert(MenuBuilder::from(next_group))
                .observe(
                    move |trigger: Trigger<AskyEvent<Vec<bool>>>, mut commands: Commands| {
                        eprintln!("trigger {:?}", trigger.event());
                    },
                );

            parent
                .construct::<Prompt>("checkbox group 1")
                .construct::<ascii::View>(())
                .construct::<CheckboxGroup>(vec!["Money".into(), "Time".into(), "Power".into()])
                // .construct::<CheckboxGroup>(vec![])
                .insert(MenuBuilder::from(next_group))
                // .insert(MenuBuilder::EntityParent(root))
                .observe(
                    move |trigger: Trigger<AskyEvent<Vec<bool>>>, mut commands: Commands| {
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
