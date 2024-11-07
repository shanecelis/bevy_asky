use bevy::{
    ecs::{
        system::SystemParam,
        component::{StorageType, ComponentHooks},
    },
    prelude::*
};
// I'm a cheat.
mod a11y {
    use bevy::prelude::*;
    #[derive(Resource, Deref, DerefMut, Default, Debug)]
    pub struct Focus(pub Option<Entity>);
}
use std::sync::atomic::{AtomicUsize, Ordering};

static AUTOINC: AtomicUsize = AtomicUsize::new(0);

#[derive(Resource, Deref, DerefMut, Default, Debug)]
pub struct Foci(Vec<Entity>);

#[derive(Resource, Default, Debug)]
pub struct KeyboardNav(bool);

#[derive(Clone)]
pub struct Focusable {
    version: usize,
    sort: usize,
}

impl Default for Focusable {
    fn default() -> Self {
        Focusable {
            version: 0,
            sort: AUTOINC.fetch_add(1, Ordering::AcqRel)
        }
    }
}

#[derive(Component, Default, Debug)]
pub struct Blocked;

pub fn plugin(app: &mut App) {
    app
        .init_resource::<Foci>()
        .insert_resource(a11y::Focus(None))
        .insert_resource(KeyboardNav(true))
        .add_systems(Update, focus_controller);
}

// pub type Focusable = AskyState;

#[derive(SystemParam)]
pub struct Focus<'w, 's> {
    query: Query<'w, 's, (Entity, &'static mut Focusable), Without<Blocked>>,
    focus: ResMut<'w, a11y::Focus>,
    keyboard_nav: ResMut<'w, KeyboardNav>,
    foci: ResMut<'w, Foci>,
    commands: Commands<'w, 's>,
}

impl<'w, 's> Focus<'w, 's> {
    pub fn is_focused(&self, id: Entity) -> bool {
        // self.focus_maybe.and_then(|focus| focus.0.map(|f| f == id)).unwrap_or(false)
        self.focus.0.map(|f| f == id).unwrap_or(false)
    }

    // pub fn unfocus(&mut self, id: Entity, is_complete: bool)  {
    //     self.query.get_mut(id).map(|mut asky_state| *asky_state = if is_complete {
    //         AskyState::Complete
    //     } else {
    //         AskyState::Error
    //     });
    // }

    pub fn move_focus(&mut self)  {
        // There is a focus resource.
        if let Some(focus_id) = self.focus.0 {
            if let Ok((_, mut focusable)) = self.query.get_mut(focus_id) {
                focusable.version += 1;
            }
            dbg!(focus_id);
            if let Some(index) = self.foci.iter().position(|&x| x == focus_id) {
                let unblocked = self.query.iter_many(self.foci.iter());
                self.focus.0 = self.foci.get(index + 1).or(self.foci.first()).cloned();
            }
        } else {
            self.focus.0 = self.foci.first().cloned();
        }
        if let Some(focus_id) = self.focus.0 {
            if let Ok((_, mut focusable)) = self.query.get_mut(focus_id) {
                focusable.version += 1;
            }
        }
        // self.query.get_mut(id).map(|mut asky_state| *asky_state = AskyState::Complete);
    }

    pub fn set_keyboard_nav(&mut self, on: bool) {
        self.keyboard_nav.0 = on;
    }

    pub fn block_and_move(&mut self, id_maybe: impl Into<Option<Entity>>) {
        self.block(id_maybe);
        self.move_focus();
    }

    pub fn block(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(id) = id_maybe.into().or(self.focus.0) {
            self.commands.entity(id).insert(Blocked);
            // self.query.get_mut(id).map(|mut focus| focus.block = true).expect("no Focusable");
        } else {
            warn!("No id to block");
        }
    }

    pub fn unblock(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(id) = id_maybe.into().or(self.focus.0) {
            self.commands.entity(id).remove::<Blocked>();
            // self.query.get_mut(id).map(|mut focus| focus.block = false).expect("no Focusable");
        } else {
            warn!("No id to unblock");
        }
    }


    // pub fn unblock(&mut self, id: Entity) {
    //     self.query.get_mut(id).map(|focus| focus.block = false).expect("no Focusable");
    // }
}

fn focus_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut focus: Focus,
) {
    if input.just_pressed(KeyCode::Tab) {
        focus.move_focus();
    }
}


impl Component for Focusable {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, targeted_entity, _component_id| {
            let mut foci = world.get_resource_mut::<Foci>().expect("Foci resource");
            foci.push(targeted_entity);

            if let Some(mut focus) = world.get_resource_mut::<a11y::Focus>() {
                if focus.is_none() {
                    focus.0 = Some(targeted_entity);
                }
            }
        });
        hooks.on_remove(|mut world, targeted_entity, _component_id| {
            let mut foci = world.get_resource_mut::<Foci>().expect("Foci resource");
            if let Some(index) = foci.iter().position(|&x| x == targeted_entity) {
                foci.remove(index);
            }
        });
    }
}
