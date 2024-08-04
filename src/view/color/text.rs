use super::*;
use crate::{
    prompt::{TextField, Prompt}, AskyState, StringCursor
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, text_view);
}

#[repr(u8)]
enum PromptPart {
    Question = 0,
    Answer = 1,
    PreCursor = 2,
    Cursor = 3,
    PostCursor = 4,
}

impl PromptPart {
    pub fn from_usize(v: usize) -> Option<Self> {
        use PromptPart::*;
        match v {
            0 => Some(Question),
            1 => Some(Answer),
            2 => Some(PreCursor),
            3 => Some(Cursor),
            4 => Some(PostCursor),
            _ => None
        }
    }
}

pub(crate) fn text_view(
    mut query: Query<
            (Entity, &AskyState, &StringCursor, Option<&Prompt>, Option<&Children>),
        (
            With<View>, With<TextField>,
            Or<(Changed<AskyState>, Changed<StringCursor>, Changed<Prompt>)>,
        ),
        >,
    mut text_query: Query<(&mut Text, &mut Visibility)>,
    mut commands: Commands,
    color_view: Res<ColorView>,
) {
    for (id, state, text_state, prompt, children_maybe) in query.iter_mut() {
        if let Some(children) = children_maybe {
            match *state {
                AskyState::Frozen | AskyState::Uninit => (),
                ref asky_state => {
                    use PromptPart::*;
                    eprint!(".");

                    for (i, child) in children.into_iter().enumerate() {
                        let vis: bool;
                        if let Ok((mut text, mut visibility)) = text_query.get_mut(*child) {
                            match PromptPart::from_usize(i).expect("prompt part") {
                                Question => {
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
                                    text.sections[1].value.replace_range(.., prompt.map(|x| x.as_ref()).unwrap_or(""));
                                    vis = true;
                                }
                                Answer => {
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
                                PreCursor => {
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
                                Cursor => {
                                    // cursor
                                    vis = !asky_state.is_done();
                                    if text_state.index >= text_state.value.len() {
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
                                PostCursor => {
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
        } else {
            let answer_color = color_view.answer;

            commands
                .entity(id)
                .with_children(|parent| {
                    parent.spawn(( // 0
                        TextBundle {
                            text: Text::from_sections([
                                "[_] ".into(),                      // 0
                                "".into(),                          // 1
                            ]),
                            ..default()
                        },
                    ));

                    parent.spawn(( // 1
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
                        TextBundle::from_section("Pre", TextStyle::default()),
                    ));
                    parent.spawn(TextBundle::from_section("", TextStyle {
                        color: Color::BLACK,
                        ..default()
                    })
                                 .with_background_color(Color::WHITE));
                    parent.spawn(( // 4
                        TextBundle::from_section("Post", TextStyle::default()),
                    ));
                });
        }

    }
}
