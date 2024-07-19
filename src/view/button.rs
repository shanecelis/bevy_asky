use crate::{AskyState, Confirm, ConfirmState};
use bevy::{
    color::palettes::basic::*,
    prelude::*
};

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
        app.add_systems(Update, (button_system, confirm_view))
            .insert_resource(ButtonView::default());
    }
}
fn confirm_view(
    mut query: Query<
        (Entity, &mut AskyState, &Confirm, &ConfirmState),
        Or<(Changed<AskyState>, Changed<ConfirmState>)>,
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

                let mut bundles = vec![TextBundle::from_sections([
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
                ])];

                if matches!(asky_state, AskyState::Complete) {
                    bundles.push(TextBundle::from_section(
                        match confirm_state.yes {
                            Some(true) => " Yes",
                            Some(false) => " No",
                            None => unreachable!(),
                        },
                        TextStyle {
                            color: color_view.answer.into(),
                            ..default()
                        },
                    ));
                } else {
                    bundles.push(TextBundle::from_section(" ", TextStyle::default()));
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
                let new_child = commands
                    .spawn(NodeBundle::default())
                    .with_children(|parent| {
                        for b in bundles {
                            parent.spawn(b);
                        }
                        if !matches!(asky_state, AskyState::Complete) {
                            parent.spawn(NodeBundle {

                            }).with_children(|parent| {
                                add_button(parent, " No ");
                                add_button(parent, " Yes ");
                            });
                        }
                    })
                    .id();
                commands
                    .entity(id)
                    .despawn_descendants()
                    .replace_children(&[new_child]);
            }
        }
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct ButtonIndex(usize);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn add_button(mut parent: &mut ChildBuilder<'_>, text: &str, index: usize) {
    parent
        .spawn((ButtonBundle {
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
        }, ButtonIndex(index)))
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
