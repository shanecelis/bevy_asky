#[cfg(feature = "focus")]
use bevy_alt_ui_navigation_lite::{prelude::*, systems::InputMapping};

#[cfg(feature = "focus")]
mod ui_navigation {
use bevy::{
    ecs::{
        system::{
            SystemParam,
        }
    },
    prelude::*,
};
    #[derive(SystemParam)]
    pub struct Focus<'w, 's> {
        focus: Query<'w, 's, &'static mut Focusable>,
        requests: EventWriter<NavRequest>,
        input_mapping: ResMut<InputMapping>,
    }

    impl<'w, 's> Focus<'w, 's> {
        pub fn is_focused(&self, id: Entity) -> bool {
            self.focus.get(id).map(|focusable| FocusState::Focused == focusable.state()).unwrap_or(true)
        }

        pub fn move_focus(&mut self, _id: Entity)  {
            self.requests.send(NavRequest::Move(NavDirection::South));
        }

        pub fn set_keyboard_nav(&mut self, on: bool) {
            self.input_mapping.keyboard_navigation = on;
        }

        pub fn block(&mut self, id: Entity) {
            self.focus.get_mut(id).map(|focusable| *focusable = Focusable::new().blocked());
            self.move_focus(id);
        }
    }
}

#[cfg(not(feature = "focus"))]
mod ui_navigation {
    use bevy::{
        ecs::{
            system::{
                SystemParam,
            }
        },
        prelude::*,
    };
    use crate::AskyState;
    pub type Focusable = AskyState;

    #[derive(SystemParam)]
    pub struct Focus<'w, 's> {
        query: Query<'w, 's, &'static mut AskyState>,
        focus_maybe: Option<ResMut<'w, bevy::a11y::Focus>>,
    }

    impl<'w, 's> Focus<'w, 's> {
        pub fn is_focused(&self, id: Entity) -> bool {
            self.query.get(id).map(|asky_state| matches!(asky_state, AskyState::Reading)).unwrap_or(true)
        }

        pub fn unfocus(&mut self, id: Entity, is_complete: bool)  {
            self.query.get_mut(id).map(|mut asky_state| *asky_state = if is_complete {
                AskyState::Complete
            } else {
                AskyState::Error
            });
        }

        pub fn move_focus(&mut self, id: Entity)  {
            self.query.get_mut(id).map(|mut asky_state| *asky_state = AskyState::Complete);
            // self.requests.send(NavRequest::Move(NavDirection::South));
        }

        pub fn set_keyboard_nav(&mut self, _on: bool) {
        }

        pub fn block(&mut self, id: Entity) {
            todo!();
        }
    }
}

pub use ui_navigation::*;
