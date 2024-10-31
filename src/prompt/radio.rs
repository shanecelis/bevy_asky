use super::Prompt;
use crate::construct::*;
use bevy::{
    a11y::{accesskit::*, *},
    prelude::*,
};
use bevy_alt_ui_navigation_lite::prelude::*;
use std::borrow::Cow;

#[derive(Component)]
pub struct Radio {
    /// Message used to display in the prompt.
    pub message: Cow<'static, str>,
    /// Initial radio of the prompt.
    pub checked: bool,
}

impl From<Cow<'static, str>> for Radio {
    fn from(message: Cow<'static, str>) -> Self {
        Radio {
            message,
            checked: false,
        }
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, radio_controller);
}

impl Construct for Radio {
    type Props = Cow<'static, str>;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(Focusable::default())
            .insert(Prompt(props.clone()))
            .insert(AccessibilityNode(NodeBuilder::new(Role::RadioButton)));

        context.world.flush();
        Ok(Radio {
            message: props,
            checked: false,
        })
    }
}

fn radio_controller(
    mut query: Query<(Entity, &mut Radio, Option<&Parent>, &Focusable)>,
    child_query: Query<&Children>,
    input: Res<ButtonInput<KeyCode>>,
    mut toggled: Local<Vec<(Entity, Entity)>>,
) {
    toggled.clear();
    for (id, mut radio, parent, focusable) in query.iter_mut() {
        if FocusState::Focused != focusable.state() {
            continue;
        }
        if input.any_just_pressed([
            KeyCode::Space,
            KeyCode::KeyH,
            KeyCode::KeyL,
            KeyCode::Enter,
            KeyCode::Escape,
        ]) {
            let was_checked = radio.checked;

            if input.just_pressed(KeyCode::Space) {
                radio.checked = true;
            }
            if input.any_just_pressed([KeyCode::KeyL]) {
                radio.checked = true;
            }
            if input.any_just_pressed([KeyCode::KeyH]) {
                radio.checked = false;
            }
            if radio.checked && !was_checked {
                // We've been checked and weren't checked before.
                if let Some(p) = parent {
                    toggled.push((id, **p));
                }
            }
        }
    }
    for (toggled_child, parent) in toggled.drain(..) {
        for child in child_query.get(parent).unwrap() {
            if *child == toggled_child {
                continue;
            }
            if let Ok((_, mut radio, _, _)) = query.get_mut(*child) {
                radio.checked = false;
            }
        }
    }
}
