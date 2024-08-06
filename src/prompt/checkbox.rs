use crate::construct::*;
use crate::{AskyEvent, AskyState, Error};
use super::{Prompt, Feedback};
use bevy::{
    prelude::*
};
use std::borrow::Cow;
use bevy_ui_navigation::prelude::*;

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
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(Focusable::default())
            .insert(Prompt(props.clone()));

        context.world.flush();
        Ok(Checkbox {
            message: props,
            checked: false,
        })
    }
}

fn checkbox_controller(
    mut query: Query<(&mut Checkbox, &Focusable)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (mut checkbox, focusable) in query.iter_mut() {
        if FocusState::Focused != focusable.state() {
            continue;
        }
            if input.any_just_pressed([
                KeyCode::Space,
                KeyCode::KeyH,
                KeyCode::KeyL,
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
