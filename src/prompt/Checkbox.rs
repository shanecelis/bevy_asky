use crate::construct::*;
use crate::{AskyEvent, AskyState, Error};
use super::{Prompt, Feedback};
use bevy::{
    a11y::Focus,
    prelude::*
};
use std::borrow::Cow;

#[derive(Component)]
pub struct Checkbox {
    /// Message used to display in the prompt.
    pub message: Cow<'static, str>,
    /// Initial checkbox of the prompt.
    pub checked: bool,
}

impl From<Cow<'static, str>> for Checkbox {
    fn from(message: Cow<'static, str>) -> Self {
        Checkbox {
            message,
            checked: false
        }
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, checkbox_controller);
}


impl Construct for Checkbox {
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
            .insert(Prompt(props.clone()))
            .insert(state);

        context.world.flush();
        Ok(Checkbox {
            message: props,
            checked: false,
        })
    }
}

fn checkbox_controller(
    mut query: Query<(Entity, &mut AskyState, &mut Checkbox)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    focus: Option<Res<Focus>>,
) {
    let focused = focus.map(|res| res.0).unwrap_or(None);
    for (id, mut state, mut checkbox) in query.iter_mut() {
        if focused.map(|x| x != id).unwrap_or(false) {
            continue;
        }
        if matches!(*state, AskyState::Reading) {
            if input.any_just_pressed([
                KeyCode::Space,
                KeyCode::KeyH,
                KeyCode::KeyL,
                KeyCode::Enter,
                KeyCode::Escape,
            ]) {

                if input.just_pressed(KeyCode::Space) {
                    checkbox.checked = !checkbox.checked;
                }
                if input.any_just_pressed([KeyCode::KeyL]) {
                    checkbox.checked = true;
                }
                if input.any_just_pressed([KeyCode::KeyH]) {
                    checkbox.checked = false;
                }
                if input.just_pressed(KeyCode::Enter) {
                    commands.trigger_targets(AskyEvent(Ok(checkbox.checked)), id);
                    *state = AskyState::Complete;
                }
                if input.just_pressed(KeyCode::Escape) {
                    commands.trigger_targets(AskyEvent::<bool>(Err(Error::Cancel)), id);
                    *state = AskyState::Error;
                }
            }
        }
    }
}

// impl Component for Checkbox {
//     const STORAGE_TYPE: StorageType = StorageType::Table;

//     fn register_component_hooks(hooks: &mut ComponentHooks) {
//         hooks.on_add(|mut world, targeted_entity, _component_id| {
//             if world.get::<ConfirmState>(targeted_entity).is_none() {
//                 let confirm_init = world.get::<Checkbox>(targeted_entity).unwrap().init;
//                 let mut commands = world.commands();
//                 commands
//                     .entity(targeted_entity)
//                     .insert(ConfirmState { yes: confirm_init });
//             }
//         });
//     }
// }
