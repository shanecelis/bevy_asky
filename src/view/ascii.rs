use std::fmt::Write;
use crate::construct::*;
use crate::{
    Focused,
    Focusable,
    prompt::{Confirm, ConfirmState, Prompt, Feedback, Placeholder, Password, Toggle, Checkbox, Radio},
    StringCursor,
    AskyState,
};
use bevy::{
    a11y::Focus,
    prelude::*
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (confirm_view, text_view, password_view, toggle_view, checkbox_view, radio_view));
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

pub(crate) fn checkbox_view(
    mut query: Query<
        (&AskyState, &Checkbox, &mut Text, Option<&Prompt>, Option<&Feedback>),
        (
            With<View>,

            Or<(Changed<AskyState>, Changed<Checkbox>, Changed<Feedback>, Changed<Prompt>, Changed<Focusable>)>,
        ),
    >,
) {
    for (asky_state, checkbox, mut text, prompt, feedback_maybe) in query.iter_mut() {
        eprint!(".");

        text.sections[0].value.replace_range(
            ..,
            if checkbox.checked {
                "[x] "
            } else {
                "[ ] "
            }
        );
        text.sections[1].value.replace_range(.., prompt.map(|x| x.as_ref()).unwrap_or(""));

        if let Some(ref feedback) = feedback_maybe {
            text.sections[4].value.clear();
            let _ = write!(&mut text.sections[4].value, " {}", &feedback);
        }
    }
}

pub(crate) fn radio_view(
    mut query: Query<
        (Entity, &AskyState, &Radio, &mut Text, Option<&Prompt>, Option<&Feedback>),
        (
            With<View>,
            Or<(Changed<AskyState>, Changed<Radio>, Changed<Feedback>, Changed<Prompt>, Changed<Focusable>)>,
        ),
    >,
    focus: Option<Res<Focus>>,
) {
    for (id, asky_state, radio, mut text, prompt, feedback_maybe) in query.iter_mut() {
        eprint!(".");
        if focus.is_focused(id) {
            text.sections[0].value.replace_range(.., "> ");
        } else {
            text.sections[0].value.replace_range(.., "  ");
        }
        text.sections[1].value.replace_range(
            ..,
            if radio.checked {
                "(o) "
            } else {
                "( ) "
            }
        );
        text.sections[2].value.replace_range(.., prompt.map(|x| x.as_ref()).unwrap_or(""));
        // text.sections[3].value.replace_range(
        //     ..,
        // );

        if let Some(ref feedback) = feedback_maybe {
            text.sections[3].value.clear();
            let _ = write!(&mut text.sections[4].value, " {}", &feedback);
        }
    }
}

pub(crate) fn toggle_view(
    mut query: Query<
        (&AskyState, &Toggle, &mut Text, Option<&Prompt>, Option<&Feedback>),
        (
            With<View>,
            Or<(Changed<AskyState>, Changed<Toggle>, Changed<Feedback>, Changed<Prompt>)>,
        ),
    >,
) {
    for (asky_state, toggle, mut text, prompt, feedback_maybe) in query.iter_mut() {
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

        text.sections[3].value.clear();
        if !matches!(asky_state, AskyState::Complete) {
            if toggle.index == 0 {
                let _ = write!(text.sections[3].value, " [{}] _{}_", toggle.options[0], toggle.options[1]);
            } else {
                let _ = write!(text.sections[3].value, " _{}_ [{}]", toggle.options[0], toggle.options[1]);
            }
        } else {
            let _ = write!(text.sections[3].value, " {}", toggle.options[toggle.index]);
        }

        if let Some(ref feedback) = feedback_maybe {
            text.sections[4].value.clear();
            let _ = write!(&mut text.sections[4].value, " {}", &feedback);
        }
    }
}

pub(crate) fn text_view(
    mut query: Query<
        (&AskyState, &StringCursor, &mut Text, Option<&Prompt>, Option<&Feedback>, Option<&Placeholder>),
        (
            With<View>,
            Without<Password>,
            Or<(Changed<AskyState>, Changed<StringCursor>, Changed<Feedback>, Changed<Prompt>)>,
        ),
    >,
) {
    for (asky_state, text_state, mut text, prompt, feedback_maybe, placeholder) in query.iter_mut() {
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

        if text_state.value.is_empty() && placeholder.is_some() {
            text.sections[2].value.clear();
            let _ = write!(text.sections[2].value, "[{}]", &placeholder.map(|x| x.as_ref()).unwrap());
            // text.sections[2].value.replace_range(.., placeholder.map(|x| x.as_ref()).unwrap_or(""));
        } else {
            text.sections[2].value.replace_range(
                ..,
                &text_state.value
            );
        }

        text.sections[4].value.clear();
        if let Some(ref feedback) = feedback_maybe {
            let _ = write!(&mut text.sections[4].value, " {}", &feedback);
        }
    }
}

pub(crate) fn password_view(
    mut query: Query<
        (&AskyState, &StringCursor, &mut Text, Option<&Prompt>, Option<&Feedback>, Option<&Placeholder>),
        (
            With<View>,
            With<Password>,
            Or<(Changed<AskyState>, Changed<StringCursor>, Changed<Feedback>, Changed<Prompt>)>,
        ),
    >,
) {
    for (asky_state, text_state, mut text, prompt, feedback_maybe, placeholder) in query.iter_mut() {
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

        if text_state.value.is_empty() && placeholder.is_some() {
            text.sections[2].value.clear();
            let _ = write!(text.sections[2].value, "[{}]", &placeholder.map(|x| x.as_ref()).unwrap());
            // text.sections[2].value.replace_range(.., placeholder.map(|x| x.as_ref()).unwrap_or(""));
        } else {
            text.sections[2].value.replace_range(
                ..,
                &"*".repeat(text_state.value.len()), // TODO: This allocates a String. Don't do that.
            );
        }

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
