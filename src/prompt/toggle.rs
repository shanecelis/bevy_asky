use super::{Feedback, Prompt};
use crate::construct::*;
use crate::{AskyEvent, AskyState, Error};
use bevy::prelude::*;
use bevy_alt_ui_navigation_lite::prelude::*;
use std::borrow::Cow;

#[derive(Component, Clone)]
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
        let state: AskyState = context.construct(AskyState::default())?;
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(Prompt(props.message.clone()))
            .insert(Focusable::default())
            .insert(state);

        context.world.flush();
        Ok(props)
    }
}

fn toggle_controller(
    mut query: Query<(Entity, &mut AskyState, &mut Toggle, &Focusable)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (id, mut state, mut toggle, focusable) in query.iter_mut() {
        if FocusState::Focused != focusable.state() {
            continue;
        }
        if let AskyState::Reading = *state {
            if input.any_just_pressed([
                KeyCode::KeyH,
                KeyCode::KeyL,
                KeyCode::Enter,
                KeyCode::Escape,
            ]) {
                if input.just_pressed(KeyCode::KeyH) {
                    toggle.index = 0;
                }
                if input.just_pressed(KeyCode::KeyL) {
                    toggle.index = 1;
                }
                if input.just_pressed(KeyCode::Enter) {
                    commands.trigger_targets(AskyEvent(Ok(toggle.index)), id);
                    *state = AskyState::Complete;
                }

                if input.just_pressed(KeyCode::Escape) {
                    commands.trigger_targets(AskyEvent::<bool>(Err(Error::Cancel)), id);
                    *state = AskyState::Error;
                    commands.entity(id).insert(Feedback::error("canceled"));
                }
            }
        }
    }
}
