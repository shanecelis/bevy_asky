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
        .add_event::<BlockRequest>()
        .add_systems(Startup, setup)
        .add_systems(Update, handle_block_requests.after(NavRequestSystem))
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
    blocks: EventWriter<'w, BlockRequest>,
    input_mapping: ResMut<'w, InputMapping>,
}

#[derive(Event, Debug)]
struct BlockRequest(Entity);

fn handle_block_requests(mut blocks: EventReader<BlockRequest>,
                         mut focusables: Query<&mut Focusable>) {
    for request in blocks.read() {
        if let Ok(mut focusable) = focusables.get_mut(request.0) {
            warn!("handle block");
            if !focusable.block() {
                warn!("Unable to block focusable. Is it the only one?");
            }
        }
    }
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
            self.blocks.send(BlockRequest(id));
            // self.move_focus(id);
            // self.focus.get_mut(id).map(|mut focusable| {
            //     if !focusable.block() {
            //         warn!("Unable to block focusable. Is it the only one?");
            //     }
            // }).unwrap();
        } else {
            todo!();
        }
    }

    pub fn block_and_move(&mut self, id_maybe: impl Into<Option<Entity>>) {
        let id = id_maybe.into();
        self.move_focus(id.clone());
        self.block(id);
    }
}
