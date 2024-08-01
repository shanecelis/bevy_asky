use super::*;
use crate::construct::*;
use crate::{
    view::*,
    prompt::{Confirm, ConfirmState},
    AskyEvent, AskyState,
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, confirm_view);
}

pub(crate) fn confirm_view(
    mut query: Query<
        (&AskyState, &ConfirmState, &Children),
        (
            With<View<Confirm>>,
            Or<(Changed<AskyState>, Changed<ConfirmState>)>,
        ),
    >,
    mut question: Query<&mut Text, With<Question>>,
    mut answers: Query<
        (
            &mut Text,
            &mut BackgroundColor,
            &mut Visibility,
            &Answer<bool>,
        ),
        Without<Question>,
    >,
    color_view: Res<ColorView>,
) {
    for (mut state, confirm_state, children) in query.iter_mut() {
        match *state {
            AskyState::Frozen | AskyState::Uninit => (),
            ref asky_state => {
                eprint!(".");

                for child in children {
                    if let Ok(mut text) = question.get_mut(*child) {
                        let highlight = TextStyle {
                            color: if matches!(asky_state, AskyState::Reading) {
                                color_view.highlight.into()
                            } else {
                                color_view.complete.into()
                            },
                            ..default()
                        };
                        text.sections[0].value.replace_range(
                            1..=1,
                            match asky_state {
                                AskyState::Reading => " ",
                                AskyState::Complete => "x",
                                AskyState::Error => "!",
                                _ => unreachable!(),
                            },
                        );
                        text.sections[0].style = highlight;
                    }
                    // for (mut background, mut visibility) in answers.iter_many_mut(children) {
                    if let Ok((mut text, mut background, mut visibility, answer)) =
                        answers.get_mut(*child)
                    {
                        let mut vis;
                        match answer {
                            Answer::Final => {
                                vis = matches!(asky_state, AskyState::Complete);
                                text.sections[0].value.replace_range(
                                    ..,
                                    if vis {
                                        match confirm_state.yes {
                                            Some(true) => "Yes",
                                            Some(false) => "No",
                                            None => "N/A",
                                        }
                                    } else {
                                        ""
                                    },
                                )
                            }
                            Answer::Selection(yes) => {
                                vis = !matches!(asky_state, AskyState::Complete);
                                if vis {
                                    *background =
                                        if confirm_state.yes.map(|x| x == *yes).unwrap_or(false) {
                                            color_view.highlight
                                        } else {
                                            color_view.lowlight
                                        }
                                        .into();
                                }
                            }
                        }
                        *visibility = if vis {
                            Visibility::Visible
                        } else {
                            Visibility::Hidden
                        };
                    }
                }
            }
        }
    }
}


impl Construct for View<Confirm> {
    type Props = <Confirm as Construct>::Props;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let confirm: Confirm = context.construct(props)?;
        // let answer_color = context.world.get_resource::<ColorView>()?;
        let color_view =
            context
                .world
                .get_resource::<ColorView>()
                .ok_or(ConstructError::MissingResource {
                    message: "No ColorView".into(),
                })?;
        let (bg_no, bg_yes) = (color_view.highlight, color_view.lowlight);
        let answer_color = color_view.answer;

        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(NodeBundle::default())
            .with_children(|parent| {
                parent.spawn((
                    Question,
                    TextBundle {
                        text: Text::from_sections([
                            "[_] ".into(),                      // 0
                            confirm.message.to_string().into(), // 1
                            " ".into(),                         // 2
                        ]),
                        ..default()
                    },
                ));

                parent.spawn((
                    Answer::<bool>::Final,
                    TextBundle {
                        text: Text::from_sections([TextSection::new(
                            "",
                            TextStyle {
                                color: answer_color.into(),
                                ..default()
                            },
                        )]),
                        ..default()
                    },
                ));
                parent.spawn((
                    Answer::Selection(false),
                    TextBundle::from_section(" No ", TextStyle::default())
                        .with_background_color(bg_no.into()),
                ));
                parent.spawn(TextBundle::from_section(" ", TextStyle::default()));
                parent.spawn((
                    Answer::Selection(true),
                    TextBundle::from_section(" Yes ", TextStyle::default())
                        .with_background_color(bg_yes.into()),
                ));
            });
        context.world.flush();

        Ok(View(confirm))
    }
}
