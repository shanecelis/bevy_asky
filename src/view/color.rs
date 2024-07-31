use crate::{AskyState, Confirm, ConfirmState};
use bevy::prelude::*;
use crate::construct::*;

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


pub (crate) fn confirm_view(
    mut query: Query<
        (&AskyState, &ConfirmState, &Children),
        (With<View<Confirm>>, Or<(Changed<AskyState>, Changed<ConfirmState>)>)
    >,
    mut question: Query<&mut Text, With<Question>>,
    mut answers: Query<(&mut BackgroundColor, &mut Visibility, &Answer<bool>)>,
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
                text.sections[0].value.replace_range(1..=1,
                    match asky_state {
                        AskyState::Reading => " ",
                        AskyState::Complete => "x",
                        AskyState::Error => "!",
                        _ => unreachable!(),
                    });
                text.sections[0].style = highlight;

                if matches!(asky_state, AskyState::Complete) {
                    text.sections[3].value.replace_range(..,
                        match confirm_state.yes {
                            Some(true) => " Yes",
                            Some(false) => " No",
                            None => unreachable!(),
                        });
                    text.sections[3].style.color = color_view.answer.into();
                } else {
                    let (bg_no, bg_yes) = if confirm_state.yes.unwrap_or(false) {
                        (color_view.lowlight, color_view.highlight)
                    } else {
                        (color_view.highlight, color_view.lowlight)
                    };

                }

                if matches!(asky_state, AskyState::Complete) {
                    text.sections[3].value.replace_range(..,
                                                         match confirm_state.yes {
                                                             Some(true) => "Yes",
                                                             Some(false) => "No",
                                                             None => unreachable!(),
                                                         })
                }

                    }
                // for (mut background, mut visibility) in answers.iter_many_mut(children) {
                    if let Ok((mut background, mut visibility, answer)) = answers.get_mut(*child) {
                        if matches!(asky_state, AskyState::Complete) {
                            *visibility = Visibility::Hidden;
                        } else {
                            let (bg_no, bg_yes) = if confirm_state.yes.unwrap_or(false) {
                                (color_view.lowlight, color_view.highlight)
                            } else {
                                (color_view.highlight, color_view.lowlight)
                            };
                            if answer.0 {
                                *background = bg_no.into();
                            } else {
                                *background = bg_yes.into();
                            }
                        }

                    }


                // }
                }

            }
        }
    }
}

// fn confirm_view2(
//     mut query: Query<
//         (Entity, &mut AskyState, &Confirm, &ConfirmState),
//         Or<(Changed<AskyState>, Changed<ConfirmState>)>,
//     >,
//     color_view: Res<ColorView>,
//     mut text: Query<&mut Text>,
//     mut commands: Commands,
// ) {
//     for (id, mut state, confirm, confirm_state) in query.iter_mut() {
//         match *state {
//             AskyState::Frozen | AskyState::Uninit => (),
//             ref asky_state => {
//                 eprint!(".");
//                 let highlight = TextStyle {
//                     color: if matches!(asky_state, AskyState::Reading) {
//                         color_view.highlight.into()
//                     } else {
//                         color_view.complete.into()
//                     },
//                     ..default()
//                 };

//                 let mut bundles = vec![TextBundle::from_sections([
//                     TextSection::new(
//                         format!(
//                             "[{}] ",
//                             match asky_state {
//                                 AskyState::Reading => " ",
//                                 AskyState::Complete => "x",
//                                 AskyState::Error => "!",
//                                 _ => unreachable!(),
//                             }
//                         ),
//                         highlight.clone(),
//                     ),
//                     TextSection::new(confirm.message.as_ref(), TextStyle::default()),
//                 ])];

//                 if matches!(asky_state, AskyState::Complete) {
//                     bundles.push(TextBundle::from_section(
//                         match confirm_state.yes {
//                             Some(true) => " Yes",
//                             Some(false) => " No",
//                             None => unreachable!(),
//                         },
//                     ));


//                         TextStyle {
//                             color: color_view.answer.into(),
//                             ..default()
//                         },
//                 } else {
//                     bundles.push(TextBundle::from_section(" ", TextStyle::default()));
//                     let (bg_no, bg_yes) = if confirm_state.yes.unwrap_or(false) {
//                         (color_view.lowlight, color_view.highlight)
//                     } else {
//                         (color_view.highlight, color_view.lowlight)
//                     };

//                     bundles.push(
//                         TextBundle::from_section(" No ", TextStyle::default())
//                             .with_background_color(bg_no.into()),
//                     );
//                     bundles.push(TextBundle::from_section(" ", TextStyle::default()));
//                     bundles.push(
//                         TextBundle::from_section(" Yes ", TextStyle::default())
//                             .with_background_color(bg_yes.into()),
//                     );
//                 }
//                 let new_child = commands
//                     .spawn(NodeBundle::default())
//                     .with_children(|parent| {
//                         for b in bundles {
//                             parent.spawn(b);
//                         }
//                     })
//                     .id();
//                 commands
//                     .entity(id)
//                     .despawn_descendants()
//                     .replace_children(&[new_child]);
//             }
//         }
//     }
// }

#[derive(Component)]
pub struct View<T>(T);

#[derive(Component)]
pub struct Answer<T>(T);

#[derive(Component)]
pub struct Question;

impl Construct for View<Confirm> {
    type Props = <Confirm as Construct>::Props;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let confirm: Confirm = context.construct(props)?;
        // let answer_color = context.world.get_resource::<ColorView>()?;
        let color_view = context.world.get_resource::<ColorView>().ok_or(ConstructError::MissingResource { message: "No ColorView".into() })?;
        let (bg_no, bg_yes) = (color_view.highlight, color_view.lowlight);
        let answer_color = color_view.answer;

        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(NodeBundle::default())
            .with_children(|parent| {
                parent.spawn((Question,
                              TextBundle {
                                  text: Text::from_sections(
                                      ["[_] ".into(), // 0
                                       confirm.message.to_string().into(), // 1
                                       " ".into(), // 2
                                       TextSection::new("",
                                                        TextStyle {
                                                            color: answer_color.into(),
                                                            ..default()
                                                        }) // 3
                                      ]),
                                  ..default()
                              }));
                parent.spawn((Answer(false), TextBundle::from_section(" No ", TextStyle::default())
                             .with_background_color(bg_no.into())));
                parent.spawn(TextBundle::from_section(" ", TextStyle::default()));
                parent.spawn((Answer(true), TextBundle::from_section(" Yes ", TextStyle::default())
                             .with_background_color(bg_yes.into())));

            });
        context.world.flush();

        Ok(View(confirm))
    }
}
