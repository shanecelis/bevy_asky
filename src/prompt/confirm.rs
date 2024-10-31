use super::{Feedback, Prompt};
use crate::construct::*;
use crate::{AskyEvent, Error, AskyChange};
use bevy::prelude::*;
use bevy_alt_ui_navigation_lite::{
    events::Direction as NavDirection,
    prelude::*,
};
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
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(Focusable::default())
            .insert(Prompt(props.clone()));

        context.world.flush();
        Ok(Confirm {
            message: props,
            yes: false,
        })
    }
}

fn confirm_controller(
    mut query: Query<(Entity, &mut Confirm, &mut Focusable)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut requests: EventWriter<NavRequest>,
) {
    for (id, mut confirm, focusable) in query.iter_mut() {
        if FocusState::Focused != focusable.state() {
            continue;
        }
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
                commands.trigger_targets(AskyChange(true), id);
            }
            if input.any_just_pressed([KeyCode::KeyN, KeyCode::KeyH]) {
                confirm.yes = false;
                commands.trigger_targets(AskyChange(false), id);
            }
            if input.just_pressed(KeyCode::Enter) {
                let yes = confirm.yes;
                requests.send(NavRequest::Move(NavDirection::South));
                // I had tried using triggers in bevy_ui_navigation to fix my issues.
                // commands.trigger(NavRequest::Move(NavDirection::South));
                commands.trigger_targets(AskyEvent::<bool>(Ok(yes)), id);
                // commands
                //     .entity(id)
                //     .insert(Feedback::info(if yes { "Yes" } else { "No" }));
            }
            if input.just_pressed(KeyCode::Escape) {
                commands.trigger_targets(AskyEvent::<bool>(Err(Error::Cancel)), id);
                commands.entity(id).insert(Feedback::error("canceled"));
            }
        }
    }
}
