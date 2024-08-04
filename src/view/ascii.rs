use std::fmt::Write;
use crate::construct::*;
use crate::{
    prompt::{Confirm, ConfirmState, Prompt, Feedback},
    StringCursor,
    AskyState,
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, confirm_view);
    app.add_systems(Update, text_view);
}

pub(crate) fn confirm_view(
    mut query: Query<
        (&AskyState, &ConfirmState, &mut Text, Option<&Prompt>, Option<&Feedback>),
        (
            With<View>,
            With<Confirm>,

            Or<(Changed<AskyState>, Changed<ConfirmState>, Changed<Feedback>, Changed<Prompt>)>,
        ),
    >,
) {
    for (asky_state, confirm_state, mut text, prompt, feedback_maybe) in query.iter_mut() {
        eprint!(".");
        text.sections[0].value.replace_range(
            1..=1,
            match asky_state {
                AskyState::Reading => " ",
                AskyState::Complete => "x",
                AskyState::Error => "!",
            },
        );
        text.sections[1].value.replace_range(.., prompt.map(|x| x.as_ref()).unwrap_or(""));
        text.sections[3].value.replace_range(
            ..,
            if !matches!(asky_state, AskyState::Complete) {
                match confirm_state.yes {
                    Some(true) => " Y/n",
                    Some(false) => " y/N",
                    None => " y/n",
                }
            } else {
                " "
            }
        );

        if let Some(ref feedback) = feedback_maybe {
            text.sections[4].value.clear();
            let _ = write!(&mut text.sections[4].value, " {}", &feedback);
        }

    }
}

pub(crate) fn text_view(
    mut query: Query<
        (&AskyState, &StringCursor, &mut Text, Option<&Prompt>, Option<&Feedback>),
        (
            With<View>,
            Or<(Changed<AskyState>, Changed<StringCursor>, Changed<Feedback>, Changed<Prompt>)>,
        ),
    >,
) {
    for (asky_state, text_state, mut text, prompt_maybe, feedback_maybe) in query.iter_mut() {
        eprint!(".");
        text.sections[0].value.replace_range(
            1..=1,
            match asky_state {
                AskyState::Reading => " ",
                AskyState::Complete => "x",
                AskyState::Error => "!",
            },
        );
        text.sections[1].value.clear();
        if let Some(prompt) = prompt_maybe {
            text.sections[1].value.replace_range(..,
                                                 &prompt.0);
        }
        text.sections[2].value.replace_range(
            ..,
            &text_state.value
        );

        text.sections[4].value.clear();
        if let Some(ref feedback) = feedback_maybe {
            let _ = write!(&mut text.sections[4].value, " {}", &feedback);
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
        // let text_input: Input = context.construct(props)?;
        let mut commands = context.world.commands();
        commands.entity(context.id).insert(TextBundle {
            text: Text::from_sections([
                "[_] ".into(),                      // 0
                "".into(), //text_input.message.to_string().into(), // 1
                "".into(),                          // 2
                "".into(),                          // 3
                "".into(),                          // 4
            ]),
            ..default()
        });
        context.world.flush();

        Ok(View)
    }
}
