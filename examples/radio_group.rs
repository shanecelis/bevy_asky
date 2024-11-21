use bevy::prelude::*;
use bevy_asky::{
    construct::*,
    prompt::*,
    view::{widget::Widgets, *},
    *,
};

#[path = "common/lib.rs"]
mod common;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AskyPlugin))
        .add_plugins(common::views)
        .add_systems(Startup, setup)
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    commands.column().with_children(|parent| {
        // parent
        //     .construct::<Prompt>("radio group 0")
        //     .construct::<ascii::View>(());
        parent
            .construct::<RadioGroup>("radio group 0")
            .construct_children::<Radio>(["Money", "Time", "Power"])
            // .with_children(|group| {
            //     group.construct::<Radio>("Money")
            //         .construct::<color::View>(());
            //     group.construct::<Radio>("Time")
            //         .construct::<color::View>(());
            //     group.construct::<Radio>("Power")
            //         .construct::<color::View>(());
            // })
            // .construct_children::<Radio>(["Money", "Time", "Power"])
            .observe(move |trigger: Trigger<AskyEvent<usize>>| {
                eprintln!("trigger {:?}", trigger.event());
            });

        // parent
        //     .construct::<Prompt>("radio group 1")
        //     .construct::<ascii::View>(());
        // parent
        //     .construct::<ascii::View>(())
        //     .construct::<RadioGroup>(vec!["Money".into(), "Time".into(), "Power".into()])
        //     .observe(move |trigger: Trigger<AskyEvent<usize>>| {
        //         eprintln!("trigger {:?}", trigger.event());
        //     });
    });
}
