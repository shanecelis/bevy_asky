use super::{Feedback, Prompt};
use crate::construct::*;
use crate::{AskyEvent, AskyState, Error};
use bevy::{a11y::Focus, prelude::*};
use bevy_ui_navigation::prelude::*;
use std::borrow::Cow;

#[derive(Component)]
pub struct Confirm {
    /// Message used to display in the prompt.
    pub message: Cow<'static, str>,
    pub yes: bool
}

impl From<Cow<'static, str>> for Confirm {
    fn from(message: Cow<'static, str>) -> Self {
        Confirm {
            message,
            yes: false,
        }
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, confirm_controller);
}

impl Construct for Confirm {
    type Props = Cow<'static, str>;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let state: AskyState = context.construct(AskyState::default())?;
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(Focusable::default())
            .insert(Prompt(props.clone()))
            .insert(state);

        context.world.flush();
        Ok(Confirm {
            message: props,
            yes: false,
        })
    }
}

fn confirm_controller(
    mut query: Query<(Entity, &mut AskyState, &mut Confirm, &Focusable)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (id, mut state, mut confirm, focusable) in query.iter_mut() {
        if FocusState::Focused != focusable.state() {
            continue;
        }
        if matches!(*state, AskyState::Reading) {
            if input.any_just_pressed([
                KeyCode::KeyY,
                KeyCode::KeyH,
                KeyCode::KeyL,
                KeyCode::KeyN,
                KeyCode::Enter,
                KeyCode::Escape,
            ]) {
                if input.any_just_pressed([KeyCode::KeyY, KeyCode::KeyL]) {
                    confirm.yes = true;
                }
                if input.any_just_pressed([KeyCode::KeyN, KeyCode::KeyH]) {
                    confirm.yes = false;
                }
                if input.just_pressed(KeyCode::Enter) {
                    let yes = confirm.yes;
                        commands.trigger_targets(AskyEvent(Ok(yes)), id);
                        commands
                            .entity(id)
                            .insert(Feedback::info(if yes { "Yes" } else { "No" }));
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
