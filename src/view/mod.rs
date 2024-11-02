use bevy::prelude::*;

pub mod ascii;
pub mod button;
pub mod click;
pub mod color;
// pub(crate) mod interaction;
pub mod widget;

#[derive(Component)]
pub struct Question;

#[derive(Component)]
// pub struct Answer<T>(T);
pub enum Answer<T> {
    Selection(T),
    Final, //(Option<T>)
}

pub(crate) fn replace_or_insert(text: &mut Text, index: usize, replacement: &str) {
    let len = text.sections.len();
    if len <= index {
        for i in len.saturating_sub(1)..index {
            text.sections.push(TextSection::default());
        }
        text.sections.push(TextSection::from(replacement));
    } else {
        text.sections[index].value.replace_range(.., replacement);
    }
}

pub(crate) fn replace_or_insert_rep(text: &mut Text, index: usize, replacement: &str, repetition: usize) {
    let len = text.sections.len();
    if len <= index {
        for i in len.saturating_sub(1)..index {
            text.sections.push(TextSection::default());
        }
        // This allocates a string, which is fine because TextSection needs one.
        text.sections
            .push(TextSection::from(replacement.repeat(repetition)));
    } else {
        text.sections[index].value.clear();
        for _ in 0..repetition {
            // This doesn't allocate a string.
            text.sections[index].value.push_str(replacement);
        }
    }
}
