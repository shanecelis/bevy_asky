//! Use ascii text
use super::replace_or_insert_rep;
use crate::{prelude::*, string_cursor::*};
use bevy::prelude::*;
use std::fmt::Write;

#[repr(u8)]
enum ViewPart {
    Focus = 0,
    PreQuestion = 1,
    Question = 2,
    Answer = 3,
    Options = 4,
    Feedback = 5,
}

/// Marker for ascii views
#[derive(Component, Default)]
pub struct View;

impl Construct for View {
    type Props = ();

    fn construct(
        context: &mut ConstructContext,
        _props: Self::Props,
    ) -> Result<Self, ConstructError> {
        let mut commands = context.world.commands();
        commands.entity(context.id).insert(TextBundle {
            text: Text::from_sections([
                "".into(), // 0
                "".into(), // 1
                "".into(), // 2
                "".into(), // 3
                "".into(), // 4
                "".into(), // 5
            ]),
            ..default()
        });
        // context.world.flush();
        Ok(View)
    }
}

/// Add ascii views handlers.
pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            focus_view,
            checkbox_view,
            radio_view,
            prompt_view,
            confirm_view,
            text_view,
            password_view,
            toggle_view,
            feedback_view,
            clear_feedback::<StringCursor>,
            clear_feedback::<Toggle>,
        ),
    );
}

pub(crate) fn confirm_view(
    mut query: Query<
        (Entity, &Confirm, &mut Text),
        (With<View>, Or<(Changed<Focusable>, Changed<Confirm>)>),
    >,
    focus: Focus,
) {
    for (id, confirm, mut text) in query.iter_mut() {
        text.sections[ViewPart::Options as usize]
            .value
            .replace_range(
                ..,
                if focus.is_focused(id) {
                    if confirm.yes {
                        " no/YES"
                    } else {
                        " NO/yes"
                    }
                } else if confirm.yes {
                    " Yes"
                } else {
                    " No"
                },
            );
    }
}

pub(crate) fn checkbox_view(
    mut query: Query<(&Checkbox, &mut Text), (With<View>, Changed<Checkbox>)>,
) {
    for (checkbox, mut text) in query.iter_mut() {
        text.sections[ViewPart::PreQuestion as usize]
            .value
            .replace_range(.., if checkbox.checked { "[x] " } else { "[ ] " });
    }
}

pub(crate) fn focus_view(
    mut query: Query<(Entity, &mut Text), (With<View>, Changed<Focusable>)>,
    focus: Focus,
) {
    for (id, mut text) in query.iter_mut() {
        if focus.is_focused(id) {
            text.sections[ViewPart::Focus as usize]
                .value
                .replace_range(.., "> ");
        } else {
            text.sections[ViewPart::Focus as usize]
                .value
                .replace_range(.., "  ");
        }
    }
}

pub(crate) fn feedback_view(
    mut query: Query<(&mut Text, &Feedback), (With<View>, Changed<Feedback>)>,
) {
    for (mut text, feedback) in query.iter_mut() {
        text.sections[ViewPart::Feedback as usize].value.clear();
        let _ = write!(
            &mut text.sections[ViewPart::Feedback as usize].value,
            " {}",
            &feedback
        );
    }
}

pub(crate) fn clear_feedback<T: Component>(
    mut query: Query<&mut Feedback, (With<View>, Changed<T>)>,
) {
    for mut feedback in query.iter_mut() {
        feedback.clear();
    }
}

pub(crate) fn prompt_view(mut query: Query<(&mut Text, &Prompt), (With<View>, Changed<Prompt>)>) {
    for (mut text, prompt) in query.iter_mut() {
        text.sections[ViewPart::Question as usize]
            .value
            .replace_range(.., prompt);
    }
}

pub(crate) fn radio_view(mut query: Query<(&Radio, &mut Text), (With<View>, Changed<Radio>)>) {
    for (radio, mut text) in query.iter_mut() {
        text.sections[ViewPart::PreQuestion as usize]
            .value
            .replace_range(.., if radio.checked { "(o) " } else { "( ) " });
    }
}

pub(crate) fn toggle_view(
    mut query: Query<
        (Entity, &Toggle, &mut Text),
        (With<View>, Or<(Changed<Focusable>, Changed<Toggle>)>),
    >,
    focus: Focus,
) {
    for (id, toggle, mut text) in query.iter_mut() {
        let o = ViewPart::Options as usize;
        text.sections[o].value.clear();
        if focus.is_focused(id) {
            if toggle.index == 0 {
                let _ = write!(
                    text.sections[o].value,
                    " [{}] _{}_",
                    toggle.options[0], toggle.options[1]
                );
            } else {
                let _ = write!(
                    text.sections[o].value,
                    " _{}_ [{}]",
                    toggle.options[0], toggle.options[1]
                );
            }
        } else {
            let _ = write!(text.sections[o].value, " {}", toggle.options[toggle.index]);
        }
    }
}

pub(crate) fn text_view(
    mut query: Query<
        (&StringCursor, &mut Text, Option<&Placeholder>),
        (With<View>, Without<Password>, Changed<StringCursor>),
    >,
) {
    for (text_state, mut text, placeholder) in query.iter_mut() {
        let a = ViewPart::Answer as usize;
        if text_state.value.is_empty() && placeholder.is_some() {
            text.sections[a].value.clear();
            let _ = write!(
                text.sections[a].value,
                "[{}]",
                &placeholder.map(|x| x.as_ref()).unwrap()
            );
        } else {
            text.sections[a].value.replace_range(.., &text_state.value);
        }
    }
}

pub(crate) fn password_view(
    mut query: Query<
        (&StringCursor, &mut Text, Option<&Placeholder>),
        (With<View>, With<Password>, Changed<StringCursor>),
    >,
) {
    for (text_state, mut text, placeholder) in query.iter_mut() {
        let a = ViewPart::Answer as usize;
        if text_state.value.is_empty() && placeholder.is_some() {
            text.sections[a].value.clear();
            let _ = write!(
                text.sections[a].value,
                "[{}]",
                &placeholder.map(|x| x.as_ref()).unwrap()
            );
        } else {
            replace_or_insert_rep(&mut text, a, "*", text_state.value.len());
        }
    }
}
