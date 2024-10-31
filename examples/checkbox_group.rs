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
        .add_systems(Update, read_keys)
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    // let mut prev_group = None;
    let root = commands
        .column()
        // .insert((MenuBuilder::Root,
        //          MenuSetting::new().wrapping()))
        // .insert(Focusable::default())
        .id();
    commands.entity(root)
        .with_children(|parent| {

            parent
                .construct::<Prompt>("checkbox group 0")
                .construct::<ascii::View>(());
            let id = parent
                .construct::<ascii::View>(())
                .construct::<CheckboxGroup>(vec!["Money".into(), "Time".into(), "Power".into()])

                // .insert(MenuSetting::new().wrapping().scope())
                // .insert(MenuBuilder::from(prev_group))
                .observe(
                    move |trigger: Trigger<AskyEvent<Vec<bool>>>, commands: Commands| {
                        eprintln!("trigger {:?}", trigger.event());
                    },
                ).id();
            // prev_group = Some(id);

            parent
                .construct::<Prompt>("checkbox group 1")
                .construct::<ascii::View>(());
            parent
                .construct::<ascii::View>(())
                .construct::<CheckboxGroup>(vec!["Money".into(), "Time".into(), "Power".into()])
                // .construct::<CheckboxGroup>(vec![])
                // .insert(MenuBuilder::from(prev_group))
                // .insert(MenuSetting::new().wrapping().scope())
                // .insert(MenuBuilder::EntityParent(root))
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
