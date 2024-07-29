use super::{
    click::{self, Click},
    widget::*,
};
use crate::{AskyEvent, AskyState, Confirm, ConfirmState};
use bevy::{color::palettes::basic::*, prelude::*};
use std::collections::HashMap;

pub struct ButtonViewPlugin;

#[derive(Debug, Resource, Component)]
struct ButtonView {
    text_color: Srgba,
    background: Option<Srgba>,
    highlight: Srgba,
    complete: Srgba,
    answer: Srgba,
    lowlight: Srgba,
}

impl Default for ButtonView {
    fn default() -> Self {
        Self {
            text_color: Srgba::WHITE,
            background: None,
            highlight: Srgba::hex("80ADFA").unwrap(),
            complete: Srgba::hex("94DD8D").unwrap(),
            answer: Srgba::hex("FFB9E8").unwrap(),
            lowlight: Srgba::hex("5A607A").unwrap(),
        }
    }
}

impl Plugin for ButtonViewPlugin {
    fn build(&self, app: &mut App) {
        click::plugin(app);
        app.add_systems(Update, (button_interaction, confirm_view))
            .insert_resource(ButtonView::default());
    }
}

fn setup_view(
    mut query: Query<
        (Entity, &mut AskyState, &Confirm, &ConfirmState),
        Without<Children>>,
    color_view: Res<ButtonView>,
    mut text: Query<&mut Text>,
    mut commands: Commands,
) {
    for (id, mut state, confirm, confirm_state) in query.iter_mut() {
        match *state {
            AskyState::Frozen | AskyState::Uninit => (),
            ref asky_state => {
                eprint!(".");
                let highlight = TextStyle {
                    color: if matches!(asky_state, AskyState::Reading) {
                        color_view.highlight.into()
                    } else {
                        color_view.complete.into()
                    },
                    ..default()
                };

                let mut bundles = vec![];
                bundles.push(
                    commands
                        .spawn(TextBundle::from_sections([
                            TextSection::new(
                                format!(
                                    "[{}] ",
                                    match asky_state {
                                        AskyState::Reading => " ",
                                        AskyState::Complete => "x",
                                        AskyState::Error => "!",
                                        _ => unreachable!(),
                                    }
                                ),
                                highlight.clone(),
                            ),
                            TextSection::new(confirm.message.as_ref(), TextStyle::default()),
                        ]))
                        .id(),
                );

                if matches!(asky_state, AskyState::Complete) {
                    let id = commands
                        .spawn(TextBundle::from_section(
                            match confirm_state.yes {
                                Some(true) => " Yes",
                                Some(false) => " No",
                                None => unreachable!(),
                            },
                            TextStyle {
                                color: color_view.answer.into(),
                                ..default()
                            },
                        ))
                        .id();
                    bundles.push(id);
                } else {
                    // bundles.push(commands.spawn(TextBundle::from_section(" ", TextStyle::default())).id());
                    // let (bg_no, bg_yes) = if confirm_state.yes.unwrap_or(false) {
                    //     (color_view.lowlight, color_view.highlight)
                    // } else {
                    //     (color_view.highlight, color_view.lowlight)
                    // };

                    // bundles.push(
                    //     TextBundle::from_section(" No ", TextStyle::default())
                    //         .with_background_color(bg_no.into()),
                    // );
                    // bundles.push(TextBundle::from_section(" ", TextStyle::default()));
                    // bundles.push(
                    //     TextBundle::from_section(" Yes ", TextStyle::default())
                    //         .with_background_color(bg_yes.into()),
                    // );
                }
                if !matches!(asky_state, AskyState::Complete | AskyState::Error) {
                    bundles.push(
                        commands
                            .button(" No ", &Palette::default())
                            .insert(ConfirmRef(id, false))
                            .observe(
                                move |trigger: Trigger<Click>,
                                      mut query: Query<(&mut AskyState, &mut ConfirmState)>,
                                      mut commands: Commands| {
                                    let (mut asky_state, mut confirm_state) =
                                        query.get_mut(id).unwrap();
                                    *asky_state = AskyState::Complete;
                                    confirm_state.yes = Some(false);
                                    commands.trigger_targets(AskyEvent(Ok(false)), id);
                                },
                            )
                            .id(),
                    );
                    bundles.push(
                        commands
                            .button(" Yes ", &Palette::default())
                            .insert(ConfirmRef(id, false))
                            .observe(
                                move |trigger: Trigger<Click>,
                                      mut query: Query<(&mut AskyState, &mut ConfirmState)>,
                                      mut commands: Commands| {
                                    let (mut asky_state, mut confirm_state) =
                                        query.get_mut(id).unwrap();
                                    *asky_state = AskyState::Complete;
                                    confirm_state.yes = Some(true);
                                    commands.trigger_targets(AskyEvent(Ok(true)), id);
                                },
                            )
                            .id(),
                    );
                    // add_button(parent, " No ", ConfirmRef(id, false));
                    // add_button(parent, " Yes ", ConfirmRef(id, true));
                }
                commands
                    .entity(id)
                    .despawn_descendants()
                    .replace_children(&bundles);
            }
        }
    }
}
fn confirm_view(
    mut query: Query<
        (Entity, &mut AskyState, &Confirm, &ConfirmState),
        // Or<(Changed<AskyState>, Changed<ConfirmState>)>,
        Changed<AskyState>,
    >,
    color_view: Res<ButtonView>,
    mut text: Query<&mut Text>,
    mut commands: Commands,
) {
    for (id, mut state, confirm, confirm_state) in query.iter_mut() {
        match *state {
            AskyState::Frozen | AskyState::Uninit => (),
            ref asky_state => {
                eprint!(".");
                let highlight = TextStyle {
                    color: if matches!(asky_state, AskyState::Reading) {
                        color_view.highlight.into()
                    } else {
                        color_view.complete.into()
                    },
                    ..default()
                };

                let mut bundles = vec![];
                bundles.push(
                    commands
                        .spawn(TextBundle::from_sections([
                            TextSection::new(
                                format!(
                                    "[{}] ",
                                    match asky_state {
                                        AskyState::Reading => " ",
                                        AskyState::Complete => "x",
                                        AskyState::Error => "!",
                                        _ => unreachable!(),
                                    }
                                ),
                                highlight.clone(),
                            ),
                            TextSection::new(confirm.message.as_ref(), TextStyle::default()),
                        ]))
                        .id(),
                );

                if matches!(asky_state, AskyState::Complete) {
                    let id = commands
                        .spawn(TextBundle::from_section(
                            match confirm_state.yes {
                                Some(true) => " Yes",
                                Some(false) => " No",
                                None => unreachable!(),
                            },
                            TextStyle {
                                color: color_view.answer.into(),
                                ..default()
                            },
                        ))
                        .id();
                    bundles.push(id);
                } else {
                    // bundles.push(commands.spawn(TextBundle::from_section(" ", TextStyle::default())).id());
                    // let (bg_no, bg_yes) = if confirm_state.yes.unwrap_or(false) {
                    //     (color_view.lowlight, color_view.highlight)
                    // } else {
                    //     (color_view.highlight, color_view.lowlight)
                    // };

                    // bundles.push(
                    //     TextBundle::from_section(" No ", TextStyle::default())
                    //         .with_background_color(bg_no.into()),
                    // );
                    // bundles.push(TextBundle::from_section(" ", TextStyle::default()));
                    // bundles.push(
                    //     TextBundle::from_section(" Yes ", TextStyle::default())
                    //         .with_background_color(bg_yes.into()),
                    // );
                }
                if !matches!(asky_state, AskyState::Complete | AskyState::Error) {
                    bundles.push(
                        commands
                            .button(" No ", &Palette::default())
                            .insert(ConfirmRef(id, false))
                            .observe(
                                move |trigger: Trigger<Click>,
                                      mut query: Query<(&mut AskyState, &mut ConfirmState)>,
                                      mut commands: Commands| {
                                    let (mut asky_state, mut confirm_state) =
                                        query.get_mut(id).unwrap();
                                    *asky_state = AskyState::Complete;
                                    confirm_state.yes = Some(false);
                                    commands.trigger_targets(AskyEvent(Ok(false)), id);
                                },
                            )
                            .id(),
                    );
                    bundles.push(
                        commands
                            .button(" Yes ", &Palette::default())
                            .insert(ConfirmRef(id, false))
                            .observe(
                                move |trigger: Trigger<Click>,
                                      mut query: Query<(&mut AskyState, &mut ConfirmState)>,
                                      mut commands: Commands| {
                                    let (mut asky_state, mut confirm_state) =
                                        query.get_mut(id).unwrap();
                                    *asky_state = AskyState::Complete;
                                    confirm_state.yes = Some(true);
                                    commands.trigger_targets(AskyEvent(Ok(true)), id);
                                },
                            )
                            .id(),
                    );
                    // add_button(parent, " No ", ConfirmRef(id, false));
                    // add_button(parent, " Yes ", ConfirmRef(id, true));
                }
                commands
                    .entity(id)
                    .despawn_descendants()
                    .replace_children(&bundles);
            }
        }
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct ConfirmRef(Entity, bool);

fn button_interaction(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &ConfirmRef,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut state_query: Query<(&mut ConfirmState, &mut AskyState)>,
    mut commands: Commands,
    mut last_state: Local<HashMap<Entity, Interaction>>,
) {
    for (id, interaction, mut color, mut border_color, children, confirm_ref) in
        &mut interaction_query
    {
        let (mut confirm_state, mut asky_state) = state_query.get_mut(confirm_ref.0).unwrap();
        let last = last_state.get(&id);
        dbg!(id.index(), *interaction);
        match *interaction {
            Interaction::Pressed => {
                // confirm_state.yes = Some(confirm_ref.1);
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                // if matches!(last, Some(Interaction::Pressed)) {
                //     commands.trigger_targets(AskyEvent(Ok(confirm_state.yes.unwrap())), confirm_ref.0);
                //     *asky_state = AskyState::Complete;
                // }
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = match confirm_state.yes {
                    None => Color::BLACK,
                    Some(yes) => {
                        if yes == confirm_ref.1 {
                            GREEN.into()
                        } else {
                            Color::BLACK
                        }
                    }
                }
            }
        }
        last_state.insert(id, *interaction);
    }
}

// fn button_view(
//     mut interaction_query: Query<
//         (
//             &mut BackgroundColor,
//             &mut BorderColor,
//             &ConfirmRef,
//         ),
//         (Changed<Interaction>, With<Button>),
//     >,
//     mut state_query: Query<(&mut ConfirmState, &mut AskyState)>,
//     mut commands: Commands,
// ) {
//     for (interaction, mut color, mut border_color, children, confirm_ref) in &mut interaction_query {
//         let (mut confirm_state, mut asky_state) = state_query.get_mut(confirm_ref.0).unwrap();
//         match *interaction {
//             Interaction::Pressed => {
//                 confirm_state.yes = Some(confirm_ref.1);
//                 commands.trigger_targets(AskyEvent(Ok(confirm_state.yes.unwrap())), confirm_ref.0);
//                 *asky_state = AskyState::Complete;
//                 *color = PRESSED_BUTTON.into();
//                 border_color.0 = RED.into();
//             }
//             Interaction::Hovered => {
//                 *color = HOVERED_BUTTON.into();
//                 border_color.0 = Color::WHITE;
//             }
//             Interaction::None => {
//                 *color = NORMAL_BUTTON.into();
//                 border_color.0 = match confirm_state.yes {
//                     None => Color::BLACK,
//                     Some(yes) => if yes == confirm_ref.1 {
//                         GREEN.into()
//                     } else {
//                         Color::BLACK
//                     }
//                 }
//             }
//         }
//     }
// }

fn add_button(mut parent: &mut ChildBuilder<'_>, text: &str, confirm_ref: ConfirmRef) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    // width: Val::Px(150.0),
                    // height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(2.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                // border_radius: BorderRadius::MAX,
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            confirm_ref,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    // font_size: 40.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
}
