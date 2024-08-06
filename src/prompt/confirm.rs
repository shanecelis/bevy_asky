use crate::construct::*;
use crate::{AskyEvent, AskyState, Error};
use super::{Prompt, Feedback};
use bevy::{
    a11y::Focus,
    prelude::*
};
use std::borrow::Cow;
use bevy_ui_navigation::prelude::*;

#[derive(Component)]
pub struct Confirm {
    /// Message used to display in the prompt.
    pub message: Cow<'static, str>,
    /// Initial confirm_state of the prompt.
    pub init: Option<bool>,
}

impl From<Cow<'static, str>> for Confirm {
    fn from(message: Cow<'static, str>) -> Self {
        Confirm {
            message,
            init: None
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
        let confirm_state = ConfirmState { yes: None };
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(Focusable::default())
            .insert(Prompt(props.clone()))
            .insert(confirm_state)
            .insert(state);

        context.world.flush();
        Ok(Confirm {
            message: props,
            init: None,
        })
    }
}

#[derive(Component)]
pub(crate) struct ConfirmState {
    pub(crate) yes: Option<bool>,
}

impl From<&Confirm> for ConfirmState {
    fn from(confirm: &Confirm) -> Self {
        ConfirmState { yes: confirm.init }
    }
}

fn confirm_controller(
    mut query: Query<(Entity, &mut AskyState, &mut ConfirmState, &Focusable)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (id, mut state, mut confirm_state, focusable) in query.iter_mut() {
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
                        confirm_state.yes = Some(true);
                    }
                    if input.any_just_pressed([KeyCode::KeyN, KeyCode::KeyH]) {
                        confirm_state.yes = Some(false);
                    }
                    if input.just_pressed(KeyCode::Enter) {
                        if let Some(yes) = confirm_state.yes {
                            commands.trigger_targets(AskyEvent(Ok(yes)), id);
                            commands.entity(id).insert(Feedback::info(if yes { "Yes" } else { "No" }));
                            *state = AskyState::Complete;
                        } else {
                            commands.entity(id).insert(Feedback::warn("select an option"));
                        }
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

// impl Component for Confirm {
//     const STORAGE_TYPE: StorageType = StorageType::Table;

//     fn register_component_hooks(hooks: &mut ComponentHooks) {
//         hooks.on_add(|mut world, targeted_entity, _component_id| {
//             if world.get::<ConfirmState>(targeted_entity).is_none() {
//                 let confirm_init = world.get::<Confirm>(targeted_entity).unwrap().init;
//                 let mut commands = world.commands();
//                 commands
//                     .entity(targeted_entity)
//                     .insert(ConfirmState { yes: confirm_init });
//             }
//         });
//     }
// }
