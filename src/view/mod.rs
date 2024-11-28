//! ascii, color, or button views
use crate::construct::*;
use crate::prelude::*;
use bevy::prelude::*;

#[cfg(feature = "ascii")]
pub mod ascii;
// pub mod button;
pub mod click;
#[cfg(feature = "color")]
pub mod color;
// pub(crate) mod interaction;
pub mod widget;

pub(crate) fn plugin(_app: &mut App) {}

/// Replace or insert a [TextSection] at a particular index.
pub fn replace_or_insert(text: &mut Text, index: usize, replacement: &str) {
    let len = text.sections.len();
    if len <= index {
        for _ in len.saturating_sub(1)..index {
            text.sections.push(TextSection::default());
        }
        text.sections.push(TextSection::from(replacement));
    } else {
        text.sections[index].value.replace_range(.., replacement);
    }
}

/// Replace or insert a [TextSection] at a particular index with a repeating string.
pub fn replace_or_insert_rep(text: &mut Text, index: usize, replacement: &str, repetition: usize) {
    let len = text.sections.len();
    if len <= index {
        for _ in len.saturating_sub(1)..index {
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
