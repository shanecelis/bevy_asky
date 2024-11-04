use crate::construct::*;
use crate::prelude::*;
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

pub(crate) fn add_view_to_checkbox<V>(
    checkboxes: Query<(Entity, &Parent), Added<Checkbox>>,
    group: Query<&CheckboxGroup, With<V>>,
    mut commands: Commands,
) where
    V: Construct<Props = ()> + Component + Send,
{
    for (id, parent) in &checkboxes {
        if group.get(parent.get()).is_ok() {
            commands.entity(id).construct::<V>(());
        }
    }
}

// pub(crate) fn add_view_to_checkbox<V>(
//     group: Query<&Children, (Added<CheckboxGroup>, With<V>)>,
//     checkboxes: Query<Entity, With<Checkbox>>,
//     mut commands: Commands,
// ) where
//     V: Construct<Props = ()> + Component + Send,
// {
//     for children in &group {
//         for id in checkboxes.iter_many(children) {
//             commands.entity(id).construct::<V>(());
//         }
//     }
// }

pub(crate) fn add_view_to_radio<V>(
    radios: Query<(Entity, &Parent), Added<Radio>>,
    group: Query<&RadioGroup, With<V>>,
    mut commands: Commands,
) where
    V: Construct<Props = ()> + Component + Send,
{
    for (id, parent) in &radios {
        if group.get(parent.get()).is_ok() {
            commands.entity(id).construct::<V>(());
        }
    }
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

pub(crate) fn replace_or_insert_rep(
    text: &mut Text,
    index: usize,
    replacement: &str,
    repetition: usize,
) {
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
