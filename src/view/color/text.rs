use super::*;
use crate::construct::*;
use crate::{
    view::*,
    prompt::{Input, InputState},
    AskyEvent, AskyState,
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, text_view);
}

pub(crate) fn text_view(
    mut query: Query<
        (&AskyState, &InputState, &Children),
        (
            With<View<Input>>,
            Or<(Changed<AskyState>, Changed<InputState>)>,
        ),
    >,
    mut text_query: Query<(&mut Text, &mut Visibility)>,
    color_view: Res<ColorView>,
) {
    for (mut state, text_state, children) in query.iter_mut() {
        match *state {
            AskyState::Frozen | AskyState::Uninit => (),
            ref asky_state => {
                eprint!(".");

                for (i, child) in children.into_iter().enumerate() {
                    let mut vis = true;
                    if let Ok((mut text, mut visibility)) = text_query.get_mut(*child) {
                    match i {
                        0 => {
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
                            vis = true;
                        }
                        1 => {
                                vis = asky_state.is_done();
                                text.sections[0].value.replace_range(
                                    ..,
                                    if vis {
                                        &text_state.value
                                    } else {
                                        ""
                                    },
                                )
                        }
                        2 => {
                            // pre cursor
                                vis = !asky_state.is_done();
                                text.sections[0].value.replace_range(
                                    ..,
                                    if vis {
                                        &text_state.value[0..text_state.index]
                                    } else {
                                        ""
                                    },
                                )
                        }
                        3 => {
                            // cursor
                                vis = !asky_state.is_done();
                                if text_state.next_index() >= text_state.value.len() {
                                    text.sections[0].value.replace_range(.., " ");
                                } else {
                                    text.sections[0].value.replace_range(
                                        ..,
                                        if vis {
                                            &text_state.value[text_state.index..text_state.next_index()]
                                        } else {
                                            ""
                                        },
                                    );
                                }
                        }
                        4 => {
                            // post cursor
                                vis = !asky_state.is_done();
                                text.sections[0].value.replace_range(
                                    ..,
                                    if vis {
                                        &text_state.value[text_state.next_index()..]
                                    } else {
                                        ""
                                    },
                                );
                        }
                        _ => ()
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

#[derive(Component)]
enum CursorPart {
    Pre,
    Cursor,
    Post
}

impl Construct for View<Input> {
    type Props = <Input as Construct>::Props;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let confirm: Input = context.construct(props)?;
        // let answer_color = context.world.get_resource::<ColorView>()?;
        let color_view =
            context
                .world
                .get_resource::<ColorView>()
                .ok_or(ConstructError::MissingResource {
                    message: "No ColorView".into(),
                })?;
        let answer_color = color_view.answer;

        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(NodeBundle::default())
            .with_children(|parent| {
                parent.spawn(( // 0
                    Question,
                    TextBundle {
                        text: Text::from_sections([
                            "[_] ".into(),                      // 0
                            confirm.message.to_string().into(), // 1
                        ]),
                        ..default()
                    },
                ));

                parent.spawn(( // 1
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
                parent.spawn(( // 2
                    CursorPart::Pre,
                    TextBundle::from_section("Pre", TextStyle::default()),
                ));
                parent.spawn((CursorPart::Cursor, // 3
                              TextBundle::from_section("", TextStyle {
                                  color: Color::BLACK,
                                  ..default()
                              })
                              .with_background_color(Color::WHITE.into())));
                parent.spawn(( // 4
                    CursorPart::Post,
                    TextBundle::from_section("Post", TextStyle::default()),
                ));
            });
        context.world.flush();

        Ok(View(confirm))
    }
}
