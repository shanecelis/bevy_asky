use bevy::{
    ecs::{
        system::SystemParam,
        component::{StorageType, ComponentHooks},
    },
    prelude::*
};

#[derive(Resource, Deref, DerefMut, Default, Debug)]
pub struct Focus(pub Option<Entity>);

impl Focus {
    pub fn is_focused(&self, id: Entity) -> bool {
        self.map(|f| f == id).unwrap_or(false)
    }
}

#[derive(Resource, Deref, DerefMut, Default, Debug)]
pub struct Foci(Vec<Entity>);

#[derive(Resource, Default, Debug)]
pub struct KeyboardNav(bool);

#[derive(Component, Clone, Default)]
pub struct Focusable {
    version: usize,
    block: bool,
}

impl Focusable {
    fn touch(&mut self) {
        self.version += 1;
    }
}

// #[derive(Component, Default, Debug)]
// pub struct Blocked;

pub fn plugin(app: &mut App) {
    app
        .init_resource::<Foci>()
        .insert_resource(Focus(None))
        .insert_resource(KeyboardNav(true))
        .add_systems(Update, (focus_on_tab,
                              reset_focus));
}

// pub type Focusable = AskyState;

#[derive(SystemParam)]
pub struct FocusParam<'w, 's> {
    query: Query<'w, 's, (Entity, &'static mut Focusable)>,
    focus: ResMut<'w, Focus>,
    keyboard_nav: ResMut<'w, KeyboardNav>,
    foci: ResMut<'w, Foci>,
    commands: Commands<'w, 's>,
}

impl<'w, 's> FocusParam<'w, 's> {
    pub fn is_focused(&self, id: Entity) -> bool {
        // self.focus_maybe.and_then(|focus| focus.0.map(|f| f == id)).unwrap_or(false)
        self.focus.is_focused(id)
    }

    // pub fn unfocus(&mut self, id: Entity, is_complete: bool)  {
    //     self.query.get_mut(id).map(|mut asky_state| *asky_state = if is_complete {
    //         AskyState::Complete
    //     } else {
    //         AskyState::Error
    //     });
    // }

    pub fn move_focus(&mut self, id_maybe: impl Into<Option<Entity>>)  {
        if let Some(focus_id) = id_maybe.into().or(self.focus.0) {
            // We're moving from a definite id.
            let mut seen_id = false;
            let mut result = None;
            for (id, focusable) in &self.query {
                if seen_id {
                    result = Some(id);
                    break;
                }
                if !focusable.block {
                    if result.is_none() {
                        result = Some(id);
                    }
                }
                if focus_id == id {
                    seen_id = true;
                }
            }
            if let Some(id) = result {
                let (_, mut focusable) = self.query.get_mut(id).unwrap();
                focusable.touch();
            }
            self.focus.0 = result;
        } else {
            // We're moving to any available id.
            self.focus.0 = self.query.iter_mut()
                                     .skip_while(|(_, focusable)| focusable.block)
                                     .next()
                                     .map(|(id, mut focusable)| {
                                         focusable.touch();
                                         id
                                     });
        }
        // There is a focus resource.
        // if let Some(focus_id) = self.focus.0 {
        //     if let Ok((_, mut focusable)) = self.query.get_mut(focus_id) {
        //         focusable.version += 1;
        //     }
        //     dbg!(focus_id);
        //     if let Some(index) = self.foci.iter().position(|&x| x == focus_id) {
        //         let mut unblocked = self.query.iter_many(self.foci.iter());
        //         let mut first_unblocked = None;
        //         let mut take_next = false;
        //         while let Some((id, focusable)) = unblocked.fetch_next() {
        //             if focus_id == id {
        //                 take_next = true;
        //             }
        //             if !focusable.block {
        //                 if first_unblocked.is_none() {
        //                     first_unblocked = Some(id);
        //                 }
        //                 if take_next {

        //                 }
        //             }
        //         }
        //         self.focus.0 = self.foci.get(index + 1).or(self.foci.first()).cloned();
        //     }
        // } else {
        //     self.focus.0 = self.foci.first().cloned();
        // }
        // if let Some(focus_id) = self.focus.0 {
        //     if let Ok((_, mut focusable)) = self.query.get_mut(focus_id) {
        //         focusable.version += 1;
        //     }
        // }
        // self.query.get_mut(id).map(|mut asky_state| *asky_state = AskyState::Complete);
    }

    pub fn set_keyboard_nav(&mut self, on: bool) {
        self.keyboard_nav.0 = on;
    }

    pub fn block_and_move(&mut self, id_maybe: impl Into<Option<Entity>>) {
        let id = id_maybe.into();
        self.block(id.clone());
        self.move_focus(id);
    }

    pub fn block(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(id) = id_maybe.into().or(self.focus.0) {
            // self.commands.entity(id).insert(Blocked);
            self.query.get_mut(id).map(|(_, mut focus)| focus.block = true).expect("no Focusable");
        } else {
            warn!("No id to block");
        }
    }

    pub fn unblock(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(id) = id_maybe.into().or(self.focus.0) {
            // self.commands.entity(id).remove::<Blocked>();
            self.query.get_mut(id).map(|(_, mut focus)| focus.block = false).expect("no Focusable");
        } else {
            warn!("No id to unblock");
        }
    }


    // pub fn unblock(&mut self, id: Entity) {
    //     self.query.get_mut(id).map(|focus| focus.block = false).expect("no Focusable");
    // }
}

fn focus_on_tab(
    input: Res<ButtonInput<KeyCode>>,
    mut focus: FocusParam,
) {
    if input.just_pressed(KeyCode::Tab) {
        focus.move_focus(None);
    }
}

fn reset_focus(mut focus: FocusParam) {
    if focus.focus.is_none() {
        focus.move_focus(None);
    }
}


// impl Component for Focusable {
//     const STORAGE_TYPE: StorageType = StorageType::Table;

//     fn register_component_hooks(hooks: &mut ComponentHooks) {
//         hooks.on_add(|mut world, targeted_entity, _component_id| {
//             let mut foci = world.get_resource_mut::<Foci>().expect("Foci resource");
//             foci.push(targeted_entity);

//             if let Some(mut focus) = world.get_resource_mut::<a11y::FocusParam>() {
//                 if focus.is_none() {
//                     focus.0 = Some(targeted_entity);
//                 }
//             }
//         });
//         hooks.on_remove(|mut world, targeted_entity, _component_id| {
//             let mut foci = world.get_resource_mut::<Foci>().expect("Foci resource");
//             if let Some(index) = foci.iter().position(|&x| x == targeted_entity) {
//                 foci.remove(index);
//             }
//         });
//     }
// }