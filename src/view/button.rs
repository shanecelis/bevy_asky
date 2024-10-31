use super::{
    click::{self, Click},
    widget::*,
    *,
};
use crate::construct::*;
use crate::{
    prompt::{Confirm, Prompt},
    AskyEvent, AskyState,
};
use bevy::color::palettes::basic::*;
use std::collections::HashMap;

#[derive(Debug, Resource, Component)]
pub struct ButtonView {
    pub text_color: Srgba,
    pub background: Option<Srgba>,
    pub highlight: Srgba,
    pub complete: Srgba,
    pub answer: Srgba,
    pub lowlight: Srgba,
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

pub fn plugin(app: &mut App) {
    click::plugin(app);
    app.insert_resource(ButtonView::default())
        .add_systems(Update, (button_interaction, confirm_view));
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn button_interaction(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Parent,
        ),
        (Changed<Interaction>, With<Button>, With<AskyElement>),
    >,
    mut state_query: Query<(&mut Confirm, &mut AskyState)>,
    mut last_state: Local<HashMap<Entity, Interaction>>,
) {
    for (id, interaction, mut color, mut border_color, parent) in &mut interaction_query {
        let (confirm, _asky_state) = state_query.get_mut(parent.get()).unwrap();
        // let last = last_state.get(&id);
        // dbg!(id.index(), *interaction);
        match *interaction {
            Interaction::Pressed => {
                // confirm.yes = Some(confirm_ref.1);
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                // if matches!(last, Some(Interaction::Pressed)) {
                //     commands.trigger_targets(AskyEvent(Ok(confirm.yes.unwrap())), confirm_ref.0);
                //     *asky_state = AskyState::Complete;
                // }
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = if confirm.yes {
                    GREEN.into()
                } else {
                    Color::BLACK
                }
            }
        }
        last_state.insert(id, *interaction);
    }
}

pub(crate) fn confirm_view(
    mut query: Query<
        (&AskyState, &Confirm, Option<&Prompt>, &Children),
        (
            With<View>,
            With<Confirm>,
            Or<(Changed<AskyState>, Changed<Confirm>, Changed<Prompt>)>,
        ),
    >,
    mut question: Query<&mut Text, With<Question>>,
    mut answers: Query<
        (
            Option<&mut Text>,
            &mut BackgroundColor,
            &mut Visibility,
            &Answer<bool>,
        ),
        Without<Question>,
    >,
    color_view: Res<ButtonView>,
) {
    for (asky_state, confirm, prompt, children) in query.iter_mut() {
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
                    },
                );
                text.sections[0].style = highlight;
                text.sections[1]
                    .value
                    .replace_range(.., prompt.map(|x| x.as_ref()).unwrap_or(""));
            }

            // for (mut background, mut visibility) in answers.iter_many_mut(children) {
            if let Ok((text, mut background, mut visibility, answer)) = answers.get_mut(*child) {
                let vis;
                match answer {
                    Answer::Final => {
                        vis = matches!(asky_state, AskyState::Complete);
                        text.unwrap().sections[0].value.replace_range(
                            ..,
                            if vis {
                                if confirm.yes {
                                    "Yes"
                                } else {
                                    "No"
                                }
                            } else {
                                ""
                            },
                        )
                    }
                    Answer::Selection(yes) => {
                        vis = !matches!(asky_state, AskyState::Complete);
                        if vis {
                            *background = if confirm.yes {
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

#[derive(Component)]
pub struct View;

impl Construct for View {
    type Props = ();

    fn construct(
        context: &mut ConstructContext,
        _props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        // let confirm: Confirm = context.construct(props)?;
        let color_view =
            context
                .world
                .get_resource::<ButtonView>()
                .ok_or(ConstructError::MissingResource {
                    message: "No ButtonView".into(),
                })?;
        let answer_color = color_view.answer;

        let id = context.id;
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(NodeBundle::default())
            .with_children(|parent| {
                parent.spawn((
                    Question,
                    TextBundle {
                        text: Text::from_sections([
                            "[_] ".into(), // 0
                            "".into(),     // confirm.message.to_string().into(), // 1
                            " ".into(),    // 2
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
                parent
                    .button(" No ", &Palette::default())
                    .insert(Answer::Selection(false))
                    .observe(
                        move |_trigger: Trigger<Click>,
                              mut query: Query<(&mut AskyState, &mut Confirm)>,
                              mut commands: Commands| {
                            let (mut asky_state, mut confirm) = query.get_mut(id).unwrap();
                            *asky_state = AskyState::Complete;
                            confirm.yes = false;
                            commands.trigger_targets(AskyEvent(Ok(false)), id);
                        },
                    );
                parent.spawn(TextBundle::from_section(" ", TextStyle::default()));

                parent
                    .button(" Yes ", &Palette::default())
                    .insert(Answer::Selection(true))
                    .observe(
                        move |_trigger: Trigger<Click>,
                              mut query: Query<(&mut AskyState, &mut Confirm)>,
                              mut commands: Commands| {
                            let (mut asky_state, mut confirm) = query.get_mut(id).unwrap();
                            *asky_state = AskyState::Complete;
                            confirm.yes = true;
                            commands.trigger_targets(AskyEvent(Ok(true)), id);
                        },
                    );
            });
        context.world.flush();

        Ok(View)
    }
}
