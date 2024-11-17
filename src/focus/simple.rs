use bevy::{ecs::system::SystemParam, prelude::*};

pub mod private {
    use bevy::prelude::*;

    #[derive(Resource, Default, Debug, Reflect)]
    #[reflect(Resource)]
    pub struct Focus(pub Option<Entity>);

    impl Focus {
        pub fn is_focused(&self, id: Entity) -> bool {
            self.0.map(|f| f == id).unwrap_or(false)
        }
    }
}

#[derive(SystemParam)]
pub struct Focus<'w> {
    focus: Res<'w, private::Focus>,
}

impl<'w> Focus<'w> {
    pub fn is_focused(&self, id: Entity) -> bool {
        self.focus.is_focused(id)
    }
}

#[derive(Resource, Default, Debug)]
pub struct KeyboardNav(bool);

#[derive(Component, Clone, Default, Reflect)]
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

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<private::Focus>()
        .register_type::<Focusable>()
        .insert_resource(private::Focus(None))
        .insert_resource(KeyboardNav(true))
        .add_systems(Update, (focus_on_tab, reset_focus));
}

// pub type Focusable = AskyState;

#[derive(SystemParam)]
pub struct FocusParam<'w, 's> {
    query: Query<'w, 's, (Entity, &'static mut Focusable)>,
    focus: ResMut<'w, private::Focus>,
    keyboard_nav: ResMut<'w, KeyboardNav>,
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

    pub fn move_focus(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(focus_id) = id_maybe.into().or(self.focus.0) {
            // We're moving from a definite id.
            let mut seen_id = false;
            let mut result = None;
            for (id, focusable) in &self.query {
                if seen_id {
                    result = Some(id);
                    break;
                }
                if focus_id == id {
                    seen_id = true;
                } else if !focusable.block && result.is_none() {
                    result = Some(id);
                }
            }
            if let Some(id) = result {
                let (_, mut focusable) = self.query.get_mut(id).unwrap();
                focusable.touch();
            }
            self.focus.0 = result;
        } else {
            // We're moving to any available id.
            self.focus.0 = self
                .query
                .iter_mut()
                .find(|(_, focusable)| !focusable.block)
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
        self.block(id);
        self.move_focus(id);
    }

    pub fn is_blocked(&self, id: Entity) -> bool {
        self.query
            .get(id)
            .map(|(_, focusable)| focusable.block)
            .unwrap_or(true)
    }

    pub fn block(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(id) = id_maybe.into().or(self.focus.0) {
            // self.commands.entity(id).insert(Blocked);
            self.query
                .get_mut(id)
                .map(|(_, mut focus)| focus.block = true)
                .expect("no Focusable");
        } else {
            warn!("No id to block");
        }
    }

    pub fn unblock(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(id) = id_maybe.into().or(self.focus.0) {
            // self.commands.entity(id).remove::<Blocked>();
            self.query
                .get_mut(id)
                .map(|(_, mut focus)| focus.block = false)
                .expect("no Focusable");
        } else {
            warn!("No id to unblock");
        }
    }
}

fn focus_on_tab(input: Res<ButtonInput<KeyCode>>, mut focus: FocusParam) {
    if input.just_pressed(KeyCode::Tab) {
        focus.move_focus(None);
    }
}

/// Reset focus if None or focus is blocked.
fn reset_focus(mut focus: FocusParam) {
    match focus.focus.0 {
        None => focus.move_focus(None),
        Some(id) => {
            if focus.is_blocked(id) {
                focus.move_focus(None)
            }
        }
    }
}
