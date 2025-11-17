use bevy::{
    prelude::*,
input_focus::{
        directional_navigation::{
            DirectionalNavigation, DirectionalNavigationMap, DirectionalNavigationPlugin,
        },
        InputDispatchPlugin, InputFocus, InputFocusVisible,
    },
};
use bevy_asky::prelude::*;

#[path = "common/lib.rs"]
mod common;
use common::View;
const NORMAL_BUTTON: Srgba = bevy::color::palettes::tailwind::BLUE_400;
const PRESSED_BUTTON: Srgba = bevy::color::palettes::tailwind::BLUE_500;
const FOCUSED_BORDER: Srgba = bevy::color::palettes::tailwind::BLUE_50;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,
                      AskyPlugin,
                      InputDispatchPlugin,
                      DirectionalNavigationPlugin,
        ))
        .insert_resource(InputFocusVisible(true))
        .add_plugins(common::views)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                // We need to show which button is currently focused
                highlight_focused_element,
            ))
        .run();
}

fn highlight_focused_element(
    input_focus: Res<InputFocus>,
    // While this isn't strictly needed for the example,
    // we're demonstrating how to be a good citizen by respecting the `InputFocusVisible` resource.
    input_focus_visible: Res<InputFocusVisible>,
    mut query: Query<(Entity, &mut BorderColor)>,
) {
    for (entity, mut border_color) in query.iter_mut() {
        if input_focus.0 == Some(entity) && input_focus_visible.0 {
            // Don't change the border size / radius here,
            // as it would result in wiggling buttons when they are focused
            *border_color = BorderColor::all(FOCUSED_BORDER);
        } else {
            *border_color = BorderColor::DEFAULT;
        }
    }
}

fn setup(mut commands: Commands, mut input_focus: ResMut<InputFocus>) {
    // UI camera
    commands.spawn(Camera2d);

    let column = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .id();
    commands.entity(column).with_children(|parent| {
        let question = parent
            .construct::<View>(())
            .construct::<Confirm>("Do you like ascii?")
            .observe(
                move |mut trigger: Trigger<Submit<bool>>, mut commands: Commands| {
                    eprintln!("trigger {:?}", trigger.event());
                    let answer = trigger.event_mut().take_result().unwrap_or(false);
                    commands.entity(column).with_children(|parent| {
                        parent.spawn(Text::new(if answer {
                            "Me too."
                        } else {
                            "We have other options."
                        }));
                        parent
                            .construct::<View>(())
                            .construct::<Confirm>("Do you prefer color?");
                    });
                    commands.entity(trigger.target()).despawn();
                },
            ).id();
        input_focus.set(question);

    });
}
