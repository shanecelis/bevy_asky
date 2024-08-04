use crate::construct::*;
use crate::{AskyEvent, AskyState, Error};
use super::{Prompt, Feedback};
use bevy::{
    a11y::Focus,
    prelude::*
};
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
        options: [T; 2]
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
            .insert(state);

        context.world.flush();
        Ok(props)
    }
}

fn toggle_controller(
    mut query: Query<(Entity, &mut AskyState, &mut Toggle)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    focus: Option<Res<Focus>>,
) {
    let focused = focus.map(|res| res.0).unwrap_or(None);
    for (id, mut state, mut toggle) in query.iter_mut() {
        if focused.map(|x| x != id).unwrap_or(false) {
            continue;
        }
        match *state {
            AskyState::Reading => {
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
            _ => (),
        }
    }
}

// impl Component for Toggle {
//     const STORAGE_TYPE: StorageType = StorageType::Table;

//     fn register_component_hooks(hooks: &mut ComponentHooks) {
//         hooks.on_add(|mut world, targeted_entity, _component_id| {
//             if world.get::<ConfirmState>(targeted_entity).is_none() {
//                 let confirm_init = world.get::<Toggle>(targeted_entity).unwrap().init;
//                 let mut commands = world.commands();
//                 commands
//                     .entity(targeted_entity)
//                     .insert(ConfirmState { yes: confirm_init });
//             }
//         });
//     }
// }
