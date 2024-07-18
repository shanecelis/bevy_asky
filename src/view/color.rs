use crate::{AskyState, Confirm, ConfirmState};
use bevy::prelude::*;

pub struct ColorViewPlugin;

#[derive(Debug, Resource, Component)]
struct ColorView {
    text_color: Srgba,
    background: Option<Srgba>,
    highlight: Srgba,
    complete: Srgba,
    answer: Srgba,
    lowlight: Srgba,
}

impl Default for ColorView {
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

impl Plugin for ColorViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, confirm_view)
            .insert_resource(ColorView::default());
    }
}
fn confirm_view(
    mut query: Query<
        (Entity, &mut AskyState, &Confirm, &ConfirmState),
        Or<(Changed<AskyState>, Changed<ConfirmState>)>,
    >,
    color_view: Res<ColorView>,
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
                    let (bg_no, bg_yes) = if confirm_state.yes.unwrap_or(false) {
                        (color_view.lowlight, color_view.highlight)
                    } else {
                        (color_view.highlight, color_view.lowlight)
                    };

                    bundles.push(
                        TextBundle::from_section(" No ", TextStyle::default())
                            .with_background_color(bg_no.into()),
                    );
                    bundles.push(TextBundle::from_section(" ", TextStyle::default()));
                    bundles.push(
                        TextBundle::from_section(" Yes ", TextStyle::default())
                            .with_background_color(bg_yes.into()),
                    );
                }
                let new_child = commands
                    .spawn(NodeBundle::default())
                    .with_children(|parent| {
                        for b in bundles {
                            parent.spawn(b);
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
