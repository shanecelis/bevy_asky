use crate::construct::*;
use crate::prelude::*;
use bevy::prelude::*;
use std::fmt::Write;

#[derive(Component)]
pub struct View;

#[repr(u8)]
enum ViewPart {
    Focus = 0,
    Header = 1,
    PreQuestion = 2,
    Question = 3,
    Answer = 4,
    Options = 5,
    Feedback = 6,
}

impl Construct for View {
    type Props = ();

    fn construct(
        context: &mut ConstructContext,
        _props: Self::Props,
    ) -> Result<Self, ConstructError> {
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(NodeBundle::default())
            .insert(Focusable::default())
            .with_children(|parent| {
                parent.spawn(TextBundle::default()); // Focus
                parent.spawn(TextBundle::default()); // Header
                parent.spawn(TextBundle::default()); // PreQuestion
                parent.spawn(TextBundle::default()); // Question
                parent.spawn(TextBundle::default()); // Answer
                parent
                    .spawn(TextBundle::default()) // Options
                    .with_children(|parent| {
                        parent.spawn(TextBundle::default());
                        parent.spawn(TextBundle::default());
                        parent.spawn(TextBundle::default());
                    });
                parent.spawn(TextBundle::default()); // Feedback
            });

        context.world.flush();

        Ok(View)
    }
}

#[derive(Debug, Resource, Component)]
struct Palette {
    text_color: Srgba,
    background: Option<Srgba>,
    highlight: Srgba,
    complete: Srgba,
    answer: Srgba,
    lowlight: Srgba,
}

impl Default for Palette {
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
    app.add_systems(
        Update,
        (
            header_view,
            radio_view,
            checkbox_view,
            focus_view,
            prompt_view,
            confirm_view,
            feedback_view,
            toggle_view,
            text_view,
            password_view,
            clear_feedback::<StringCursor>,
            clear_feedback::<Toggle>,
        ),
    )
    .insert_resource(Palette::default());
}

pub(crate) fn prompt_view(
    mut query: Query<(&Children, &Prompt), (With<View>, Changed<Prompt>)>,
    mut texts: Query<&mut Text>,
) {
    for (children, prompt) in query.iter_mut() {
        let mut text = texts
            .get_mut(children[ViewPart::Question as usize])
            .expect("prompt");
        replace_or_insert(&mut text, 0, prompt);
    }
}

pub(crate) fn feedback_view(
    mut texts: Query<&mut Text>,
    mut query: Query<(&Children, &Feedback), (With<View>, Changed<Feedback>)>,
) {
    for (children, feedback) in query.iter_mut() {
        let mut text = texts
            .get_mut(children[ViewPart::Feedback as usize])
            .expect("feedback");
        replace_or_insert(&mut text, 0, &format!(" {}", feedback.message));
    }
}

pub(crate) fn clear_feedback<T: Component>(
    mut query: Query<&mut Feedback, (With<View>, Changed<T>)>,
) {
    for mut feedback in query.iter_mut() {
        feedback.clear();
    }
}

pub(crate) fn focus_view(
    mut texts: Query<&mut Text>,
    mut query: Query<(&Children, &Focusable), (With<View>, Changed<Focusable>)>,
    palette: Res<Palette>,
) {
    for (children, focusable) in query.iter_mut() {
        let mut text = texts
            .get_mut(children[ViewPart::Focus as usize])
            .expect("focus");
        replace_or_insert(
            &mut text,
            0,
            match focusable.state() {
                FocusState::Focused => "> ",
                _ => "  ",
            },
        );
        text.sections[0].style.color = palette.highlight.into();
    }
}

pub(crate) fn header_view(
    mut texts: Query<&mut Text>,
    mut query: Query<(&Children, &AskyState), (With<View>, Changed<AskyState>)>,
) {
    for (children, asky_state) in query.iter_mut() {
        let mut text = texts
            .get_mut(children[ViewPart::Header as usize])
            .expect("header");
        replace_or_insert(
            &mut text,
            0,
            match asky_state {
                AskyState::Reading => "[ ] ",
                AskyState::Complete => "[x] ",
                AskyState::Error => "[!] ",
            },
        );
    }
}

pub(crate) fn checkbox_view(
    mut query: Query<(&Checkbox, &Children, &Focusable), (With<View>, Changed<Checkbox>)>,
    mut texts: Query<&mut Text>,
    palette: Res<Palette>,
) {
    for (checkbox, children, focusable) in query.iter_mut() {
        let mut text = texts
            .get_mut(children[ViewPart::PreQuestion as usize])
            .expect("header");
        replace_or_insert(&mut text, 0, if checkbox.checked { "[x] " } else { "[ ] " });
        text.sections[0].style.color = if focusable.state() == FocusState::Focused {
            palette.highlight.into()
        } else {
            palette.text_color.into()
        };
    }
}

pub(crate) fn text_view(
    mut query: Query<
        (&StringCursor, &Children, Option<&Placeholder>, &Focusable),
        (
            With<View>,
            Without<Password>,
            Or<(Changed<StringCursor>, Changed<Focusable>)>,
        ),
    >,
    mut texts: Query<&mut Text>, //, &mut BackgroundColor)>,
    mut sections: Query<&Children>,
    palette: Res<Palette>,
    mut commands: Commands,
) {
    for (text_state, children, placeholder, focusable) in query.iter_mut() {
        let id = children[ViewPart::Answer as usize];
        if let Ok(cursor_parts) = sections.get(id) {
            let mut parts = texts.iter_many_mut(cursor_parts);
            if focusable.state() == FocusState::Focused {
                let mut pre_cursor = parts.fetch_next().expect("pre cursor");
                replace_or_insert(&mut pre_cursor, 0, &text_state.value[0..text_state.index]);
                let mut cursor = parts.fetch_next().expect("cursor");
                replace_or_insert(
                    &mut cursor,
                    0,
                    if text_state.index >= text_state.value.len() {
                        " "
                    } else {
                        &text_state.value[text_state.index..text_state.next_index()]
                    },
                );
                let mut post_cursor = parts.fetch_next().expect("post cursor");
                replace_or_insert(
                    &mut post_cursor,
                    0,
                    &text_state.value[text_state.next_index()..],
                );
            } else {
                let mut pre_cursor = parts.fetch_next().expect("pre cursor");
                replace_or_insert(&mut pre_cursor, 0, &text_state.value);
                let mut cursor = parts.fetch_next().expect("cursor");
                replace_or_insert(&mut cursor, 0, "");
                let mut post_cursor = parts.fetch_next().expect("post cursor");
                replace_or_insert(&mut post_cursor, 0, "");
            }
        } else {
            // Make the parts.
            commands.entity(id).with_children(|parent| {
                // pre cursor
                parent.spawn(TextBundle::from_section(
                    &text_state.value[0..text_state.index],
                    TextStyle::default(),
                ));
                // cursor
                parent.spawn(
                    TextBundle::from_section(
                        if text_state.index >= text_state.value.len() {
                            " "
                        } else {
                            &text_state.value[text_state.index..text_state.next_index()]
                        },
                        TextStyle {
                            color: Color::BLACK,
                            ..default()
                        },
                    )
                    .with_background_color(Color::WHITE),
                );
                // post cursor
                parent.spawn(TextBundle::from_section(
                    &text_state.value[text_state.next_index()..],
                    TextStyle::default(),
                ));
            });
        }
    }
}

pub(crate) fn password_view(
    mut query: Query<
        (&StringCursor, &Children, Option<&Placeholder>, &Focusable),
        (
            With<View>,
            With<Password>,
            Or<(Changed<StringCursor>, Changed<Focusable>)>,
        ),
    >,
    mut texts: Query<&mut Text>, //, &mut BackgroundColor)>,
    mut sections: Query<&Children>,
    palette: Res<Palette>,
    mut commands: Commands,
) {
    for (text_state, children, placeholder, focusable) in query.iter_mut() {
        let glyph = "*";
        let id = children[ViewPart::Answer as usize];
        if let Ok(cursor_parts) = sections.get(id) {
            let mut parts = texts.iter_many_mut(cursor_parts);
            if focusable.state() == FocusState::Focused {
                let mut pre_cursor = parts.fetch_next().expect("pre cursor");
                replace_or_insert_rep(&mut pre_cursor, 0, glyph, text_state.index);
                let mut cursor = parts.fetch_next().expect("cursor");
                replace_or_insert_rep(
                    &mut cursor,
                    0,
                    if text_state.index >= text_state.value.len() {
                        " "
                    } else {
                        glyph
                    },
                    1,
                );
                let mut post_cursor = parts.fetch_next().expect("post cursor");
                replace_or_insert_rep(
                    &mut post_cursor,
                    0,
                    glyph,
                    text_state.value.len().saturating_sub(text_state.index + 1),
                );
            } else {
                let mut pre_cursor = parts.fetch_next().expect("pre cursor");
                replace_or_insert(&mut pre_cursor, 0, &text_state.value);
                let mut cursor = parts.fetch_next().expect("cursor");
                replace_or_insert(&mut cursor, 0, "");
                let mut post_cursor = parts.fetch_next().expect("post cursor");
                replace_or_insert(&mut post_cursor, 0, "");
            }
        } else {
            // Make the parts.
            commands.entity(id).with_children(|parent| {
                // pre cursor
                parent.spawn(TextBundle::from_section(
                    glyph.repeat(text_state.index),
                    TextStyle::default(),
                ));
                // cursor
                parent.spawn(
                    TextBundle::from_section(
                        if text_state.index >= text_state.value.len() {
                            " "
                        } else {
                            glyph
                        },
                        TextStyle {
                            color: Color::BLACK,
                            ..default()
                        },
                    )
                    .with_background_color(Color::WHITE),
                );
                // post cursor
                parent.spawn(TextBundle::from_section(
                    glyph.repeat(text_state.value.len().saturating_sub(text_state.index)),
                    TextStyle::default(),
                ));
            });
        }
    }
}

pub(crate) fn toggle_view(
    mut query: Query<
        (&AskyState, &Toggle, &Children),
        (With<View>, Or<(Changed<AskyState>, Changed<Toggle>)>),
    >,
    mut sections: Query<&Children>,
    mut texts: Query<(&mut Text, &mut BackgroundColor)>,
    palette: Res<Palette>,
) {
    for (asky_state, toggle, children) in query.iter_mut() {
        let options: &Children = sections
            .get(children[ViewPart::Options as usize])
            .expect("options");
        // text.sections[ViewPart::Options as usize]
        if let Ok((mut text, mut color)) = texts.get_mut(options[0]) {
            if text.sections.len() == 0 {
                text.sections
                    .push(format!(" {} ", toggle.options[0]).into());
            }
            *color = if toggle.index == 0 {
                palette.highlight.into()
            } else {
                palette.lowlight.into()
            }
        }

        if let Ok((mut text, mut color)) = texts.get_mut(options[1]) {
            if text.sections.len() == 0 {
                text.sections.push(" ".into());
            }
        }

        if let Ok((mut text, mut color)) = texts.get_mut(options[2]) {
            if text.sections.len() == 0 {
                text.sections
                    .push(format!(" {} ", toggle.options[1]).into());
            }
            *color = if toggle.index == 1 {
                palette.highlight.into()
            } else {
                palette.lowlight.into()
            }
        }
    }
}

pub(crate) fn radio_view(
    mut texts: Query<&mut Text>,
    mut query: Query<
        (&Radio, &Children, &Focusable),
        (With<View>, Or<(Changed<Radio>, Changed<Focusable>)>),
    >,
    palette: Res<Palette>,
) {
    for (radio, children, focusable) in query.iter_mut() {
        let mut text = texts
            .get_mut(children[ViewPart::PreQuestion as usize])
            .expect("header");
        replace_or_insert(&mut text, 0, if radio.checked { "(x) " } else { "( ) " });
        text.sections[0].style.color = if focusable.state() == FocusState::Focused {
            palette.highlight.into()
        } else {
            palette.text_color.into()
        };
    }
}

pub(crate) fn confirm_view(
    mut query: Query<
        (&Confirm, &Children, &Focusable),
        (With<View>, Or<(Changed<Focusable>, Changed<Confirm>)>),
    >,
    mut sections: Query<&Children>,
    mut texts: Query<(&mut Text, &mut BackgroundColor)>,
    palette: Res<Palette>,
) {
    for (confirm, children, focusable) in query.iter_mut() {
        let options: &Children = sections
            .get(children[ViewPart::Options as usize])
            .expect("options");
        if let Ok((mut text, mut color)) = texts.get_mut(options[0]) {
            if text.sections.len() == 0 {
                text.sections.push(" No ".into());
            }
            *color = if confirm.yes {
                palette.lowlight.into()
            } else {
                palette.highlight.into()
            }
        }

        if let Ok((mut text, mut color)) = texts.get_mut(options[1]) {
            if text.sections.len() == 0 {
                text.sections.push(" ".into());
            }
        }

        if let Ok((mut text, mut color)) = texts.get_mut(options[2]) {
            if text.sections.len() == 0 {
                text.sections.push(" Yes ".into());
            }
            *color = if confirm.yes {
                palette.highlight.into()
            } else {
                palette.lowlight.into()
            }
        }
    }
}
