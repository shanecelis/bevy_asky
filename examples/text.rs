use bevy::prelude::*;
use bevy_asky::{construct::*, prompt::*, view::*, *};
// use crate::view::ascii::*;
// use bevy_asky::view::button::*;

use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin, AskyPlugin))
        .add_plugins(view::ascii::plugin)
        .add_plugins(view::color::plugin)
        .add_plugins(view::button::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (text_update_system, text_color_system, read_keys))
        .run();
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "hello\nbevy!",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 100.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        ColorText,
    ));

    let column = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .id();
    // commands.construct::<ascii::View<Input>>("Name? ")
    commands.construct::<color::View<Input>>("Name? ")
            .observe(
                move |trigger: Trigger<AskyEvent<String>>, mut commands: Commands| {
                    eprintln!("trigger {:?}", trigger.event());
                });

    // commands.entity(column).with_children(|parent| {
    //     parent
    //         .spawn_empty()
    //         .construct::<ascii::View<Confirm>>("Do you like ascii?")
    //         .observe(
    //             move |trigger: Trigger<AskyEvent<bool>>, mut commands: Commands| {
    //                 eprintln!("trigger {:?}", trigger.event());
    //                 let answer = trigger.event().as_ref().unwrap();
    //                 commands.entity(column).with_children(|parent| {
    //                     parent.spawn(TextBundle::from_section(
    //                         if *answer {
    //                             "Me too."
    //                         } else {
    //                             "We have other options."
    //                         },
    //                         TextStyle::default(),
    //                     ));

    //                 parent
    //                     .spawn_empty()
    //                     .construct::<color::View<Confirm>>("Do you prefer color?");
    //                 });

    //             },
    //         );
    // });
    // commands
    //     .spawn((
    //         NodeBundle { ..default() },
    //         AskyState::default(),
    //         Confirm {
    //             message: "Do thing?".into(),
    //             init: None,
    //         },
    //     ))
    //     .observe(|trigger: Trigger<AskyEvent<bool>>| {
    //         eprintln!("trigger {:?}", trigger.event());
    //     });

    commands.spawn(
        // Here we are able to call the `From` method instead of creating a new `TextSection`.
        // This will use the default font (a minimal subset of FiraMono) and apply the default styling.
        TextBundle::from("From an &str into a TextBundle with the default font!").with_style(
            Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
        ),
    );
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in &mut query {
        let seconds = time.elapsed_seconds();

        // Update the color of the first and only section.
        text.sections[0].style.color = Color::srgb(
            (1.25 * seconds).sin() / 2.0 + 0.5,
            (0.75 * seconds).sin() / 2.0 + 0.5,
            (0.50 * seconds).sin() / 2.0 + 0.5,
        );
    }
}

fn read_keys(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut AskyState>) {
    if input.just_pressed(KeyCode::KeyR) {
        for mut state in query.iter_mut() {
            *state = AskyState::Reading;
        }
    }
}

fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
