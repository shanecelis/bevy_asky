use bevy_alt_ui_navigation_lite::{
    prelude::*,
    systems::InputMapping,
    events::Direction as NavDirection
};
use bevy::{
    ecs::{
        system::{
            SystemParam,
        }
    },
    prelude::*,
};

pub fn plugin(app: &mut App) {
    app
        .add_systems(Startup, setup)
        .add_plugins(DefaultNavigationPlugins);
}

fn setup(mut input_mapping: ResMut<InputMapping>) {
    input_mapping.keyboard_navigation = true;
}

pub use bevy_alt_ui_navigation_lite::prelude::Focusable;

#[derive(SystemParam)]
pub struct Focus<'w, 's> {
    query: Query<'w, 's, &'static Focused>,
}

impl<'w, 's> Focus<'w, 's> {
    pub fn is_focused(&self, id: Entity) -> bool {
        self.query.get(id).is_ok()
    }
}

#[derive(SystemParam)]
pub struct FocusParam<'w, 's> {
    focus: Query<'w, 's, &'static mut Focusable>,
    requests: EventWriter<'w, NavRequest>,
    input_mapping: ResMut<'w, InputMapping>,
}

impl<'w, 's> FocusParam<'w, 's> {
    pub fn is_focused(&self, id: Entity) -> bool {
        self.focus.get(id).map(|focusable| FocusState::Focused == focusable.state()).unwrap_or(true)
    }

    pub fn move_focus(&mut self, _id_maybe: impl Into<Option<Entity>>)  {
        self.requests.send(NavRequest::Move(NavDirection::South));
    }

    pub fn set_keyboard_nav(&mut self, on: bool) {
        self.input_mapping.keyboard_navigation = on;
    }

    pub fn block(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(id) = id_maybe.into() {
            self.move_focus(id);
            self.focus.get_mut(id).map(|mut focusable| {
                if !focusable.block() {
                    warn!("Unable to block focusable. Is it the only one?");
                }
            }).unwrap();
        } else {
            todo!();
        }
    }

    pub fn block_and_move(&mut self, id_maybe: impl Into<Option<Entity>>) {
        let id = id_maybe.into();
        self.block(id.clone());
        self.move_focus(id);
    }
}
