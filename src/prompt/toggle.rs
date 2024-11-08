use super::{Feedback, Prompt};
use crate::construct::*;
use crate::{AskyEvent, Error, Focusable, FocusParam};
use bevy::prelude::*;
#[cfg(feature = "focus")]
use bevy_alt_ui_navigation_lite::prelude::*;
use std::borrow::Cow;

#[derive(Component, Clone, Reflect)]
pub struct Toggle {
    pub message: Cow<'static, str>,
    pub options: [Cow<'static, str>; 2],
    /// Initial toggle of the prompt.
    pub index: usize,
}

impl Toggle {
    pub fn new<T: Into<Cow<'static, str>>>(
        message: impl Into<Cow<'static, str>>,
        options: [T; 2],
    ) -> Self {
        let mut iter = options.into_iter();
        Toggle {
            message: message.into(),
            options: [iter.next().unwrap().into(), iter.next().unwrap().into()],
            index: 0,
        }
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, toggle_controller);
}

impl Construct for Toggle {
    type Props = Toggle;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(NodeBundle::default())
            .insert(Prompt(props.message.clone()))
            .insert(Focusable::default());

        context.world.flush();
        Ok(props)
    }
}

fn toggle_controller(
    mut query: Query<(Entity, &mut Toggle)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut focus: FocusParam,
) {
    for (id, mut toggle) in query.iter_mut() {
        if !focus.is_focused(id) {
            continue;
        }
            if input.any_just_pressed([
                KeyCode::KeyH,
                KeyCode::ArrowLeft,
                KeyCode::KeyL,
                KeyCode::ArrowRight,
                KeyCode::Enter,
                KeyCode::Escape,
            ]) {
                if input.any_just_pressed([KeyCode::KeyH, KeyCode::ArrowLeft]) {
                    toggle.index = 0;
                }
                if input.any_just_pressed([KeyCode::KeyL, KeyCode::ArrowRight]) {
                    toggle.index = 1;
                }
                if input.just_pressed(KeyCode::Enter) {
                    commands.trigger_targets(AskyEvent(Ok(toggle.index)), id);
                    focus.block_and_move(id);
                }

                if input.just_pressed(KeyCode::Escape) {
                    commands.trigger_targets(AskyEvent::<bool>(Err(Error::Cancel)), id);
                    focus.move_focus(id);
                    // focus.unfocus(id, false);
                    commands.entity(id).insert(Feedback::error("canceled"));
                }
            }
    }
}
