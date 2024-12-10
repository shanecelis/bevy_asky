//! Use ascii text
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
        commands
            .entity(context.id)
            .insert(Text::default())
            .with_children(|parent| {
                parent.spawn(TextSpan::default());
                parent.spawn(TextSpan::default());
                parent.spawn(TextSpan::default());
                parent.spawn(TextSpan::default());
                parent.spawn(TextSpan::default());
                parent.spawn(TextSpan::default());
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
        (Entity, &Confirm),
        (
            With<View>,
            With<Text>,
            Or<(Changed<Focusable>, Changed<Confirm>)>,
        ),
    >,
    mut writer: TextUiWriter,
    focus: Focus,
) {
    for (id, confirm) in query.iter_mut() {
        writer.text(id, ViewPart::Options as usize).replace_range(
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
    mut query: Query<(Entity, &Checkbox), (With<View>, With<Text>, Changed<Checkbox>)>,
    mut writer: TextUiWriter,
) {
    for (id, checkbox) in query.iter_mut() {
        writer
            .text(id, ViewPart::PreQuestion as usize)
            .replace_range(.., if checkbox.checked { "[x] " } else { "[ ] " });
    }
}

pub(crate) fn focus_view(
    mut query: Query<Entity, (With<View>, With<Text>, Changed<Focusable>)>,
    focus: Focus,
    mut writer: TextUiWriter,
) {
    for id in query.iter_mut() {
        writer
            .text(id, ViewPart::Focus as usize)
            .replace_range(.., if focus.is_focused(id) { "> " } else { "  " });
    }
}

pub(crate) fn feedback_view(
    mut query: Query<(Entity, &Feedback), (With<View>, With<Text>, Changed<Feedback>)>,
    mut writer: TextUiWriter,
) {
    for (id, feedback) in query.iter_mut() {
        let mut text = writer.text(id, ViewPart::Feedback as usize);
        text.clear();
        let _ = write!(text, " {}", &feedback);
    }
}

pub(crate) fn clear_feedback<T: Component>(
    mut query: Query<&mut Feedback, (With<View>, Changed<T>)>,
) {
    for mut feedback in query.iter_mut() {
        feedback.clear();
    }
}

pub(crate) fn prompt_view(
    mut query: Query<(Entity, &Prompt), (With<View>, With<Text>, Changed<Prompt>)>,
    mut writer: TextUiWriter,
) {
    for (id, prompt) in query.iter_mut() {
        writer
            .text(id, ViewPart::Question as usize)
            .replace_range(.., prompt);
    }
}

pub(crate) fn radio_view(
    mut query: Query<(Entity, &Radio), (With<View>, With<Text>, Changed<Radio>)>,
    mut writer: TextUiWriter,
) {
    for (id, radio) in query.iter_mut() {
        writer
            .text(id, ViewPart::PreQuestion as usize)
            .replace_range(.., if radio.checked { "(o) " } else { "( ) " });
    }
}

pub(crate) fn toggle_view(
    mut query: Query<
        (Entity, &Toggle),
        (
            With<View>,
            With<Text>,
            Or<(Changed<Focusable>, Changed<Toggle>)>,
        ),
    >,
    focus: Focus,
    mut writer: TextUiWriter,
) {
    for (id, toggle) in query.iter_mut() {
        let mut text = writer.text(id, ViewPart::Options as usize);
        text.clear();
        if focus.is_focused(id) {
            if toggle.index == 0 {
                let _ = write!(text, " [{}] _{}_", toggle.options[0], toggle.options[1]);
            } else {
                let _ = write!(text, " _{}_ [{}]", toggle.options[0], toggle.options[1]);
            }
        } else {
            let _ = write!(text, " {}", toggle.options[toggle.index]);
        }
    }
}

pub(crate) fn text_view(
    mut query: Query<
        (Entity, &StringCursor, Option<&Placeholder>),
        (
            With<View>,
            With<Text>,
            Without<Password>,
            Changed<StringCursor>,
        ),
    >,
    mut writer: TextUiWriter,
) {
    for (id, text_state, placeholder) in query.iter_mut() {
        let mut text = writer.text(id, ViewPart::Answer as usize);
        if text_state.value.is_empty() && placeholder.is_some() {
            text.clear();
            let _ = write!(text, "[{}]", &placeholder.map(|x| x.as_ref()).unwrap());
        } else {
            text.replace_range(.., &text_state.value);
        }
    }
}

pub(crate) fn password_view(
    mut query: Query<
        (Entity, &StringCursor, Option<&Placeholder>),
        (
            With<View>,
            With<Text>,
            With<Password>,
            Changed<StringCursor>,
        ),
    >,
    mut writer: TextUiWriter,
) {
    for (id, text_state, placeholder) in query.iter_mut() {
        let mut text = writer.text(id, ViewPart::Answer as usize);
        if text_state.value.is_empty() && placeholder.is_some() {
            text.clear();
            let _ = write!(text, "[{}]", &placeholder.map(|x| x.as_ref()).unwrap());
        } else {
            let replacement = "*";
            text.clear();
            for _ in 0..text_state.value.len() {
                text.push_str(replacement);
            }
        }
    }
}
