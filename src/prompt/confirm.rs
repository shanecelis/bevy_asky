use super::{Feedback, Prompt};
use crate::construct::*;
use crate::{AskyState, AskyChange, AskyEvent, Error};
use bevy::prelude::*;
use bevy_alt_ui_navigation_lite::{events::Direction as NavDirection, prelude::*};
use std::borrow::Cow;

#[derive(Component)]
pub struct Confirm {
    pub yes: bool,
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
            // .insert(AskyState::default())
            .insert(Prompt(props.clone()));

        context.world.flush();
        Ok(Confirm {
            yes: false,
        })
    }
}

fn confirm_controller(
    mut query: Query<(Entity, &mut Confirm, &Focusable)>,// &mut AskyState)>,
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
            KeyCode::ArrowRight,
            KeyCode::ArrowLeft,
            KeyCode::KeyH,
            KeyCode::KeyL,
            KeyCode::KeyN,
            KeyCode::Enter,
            KeyCode::Escape,
        ]) {
            if input.any_just_pressed([KeyCode::KeyY, KeyCode::KeyL, KeyCode::ArrowRight]) {
                confirm.yes = true;
                commands.trigger_targets(AskyChange(true), id);
            }
            if input.any_just_pressed([KeyCode::KeyN, KeyCode::KeyH, KeyCode::ArrowLeft]) {
                confirm.yes = false;
                commands.trigger_targets(AskyChange(false), id);
            }
            if input.just_pressed(KeyCode::Enter) {
                // *state = AskyState::Complete;
                // Make this not focusable again.
                commands.entity(id).insert(Focusable::new().blocked());
                requests.send(NavRequest::Move(NavDirection::South));
                // I had tried using triggers in bevy_ui_navigation to fix my issues.
                // commands.trigger(NavRequest::Move(NavDirection::South));
                commands.trigger_targets(AskyEvent::<bool>(Ok(confirm.yes)), id);
                // commands
                //     .entity(id)
                //     .insert(Feedback::info(if yes { "Yes" } else { "No" }));
            }
            if input.just_pressed(KeyCode::Escape) {
                // *state = AskyState::Error;
                commands.trigger_targets(AskyEvent::<bool>(Err(Error::Cancel)), id);
                commands.entity(id).insert(Feedback::error("canceled"));
            }
        }
    }
}
