use crate::{construct::*, prelude::*};
use bevy::prelude::*;
use std::borrow::Cow;

/// Confirm query
#[derive(Debug, Component, Reflect)]
pub struct Confirm {
    /// Yes or no
    pub yes: bool,
}

unsafe impl Submitter for Confirm {
    type Out = bool;
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, confirm_controller.in_set(AskySet::Controller));
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
        Ok(Confirm { yes: false })
    }
}

fn confirm_controller(
    mut query: Query<(Entity, &mut Confirm)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    focus: FocusParam,
) {
    for (id, mut confirm) in query.iter_mut() {
        if !focus.is_focused(id) {
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
            }
            if input.any_just_pressed([KeyCode::KeyN, KeyCode::KeyH, KeyCode::ArrowLeft]) {
                confirm.yes = false;
            }
            if input.just_pressed(KeyCode::Enter) {
                // Make this not focusable again.
                // I had tried using triggers in bevy_ui_navigation to fix my issues.
                // commands.trigger(NavRequest::Move(NavDirection::South));
                commands.trigger_targets(Submit::<bool>::new(Ok(confirm.yes)), id);
                // focus.block_and_move(id);
            }
            if input.just_pressed(KeyCode::Escape) {
                commands.trigger_targets(Submit::<bool>::new(Err(Error::Cancel)), id);
                // commands.entity(id).insert(Feedback::error("canceled"));
            }
        }
    }
}
