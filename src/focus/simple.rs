use bevy::{ecs::system::SystemParam, math::CompassQuadrant, prelude::*};
use std::fmt::Debug;

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
    focus: ResMut<'w, private::Focus>,
}

impl<'w> Focus<'w> {
    pub fn is_focused(&self, id: Entity) -> bool {
        self.focus.is_focused(id)
    }
    pub fn focus_on(&mut self, id: Entity) {
        self.focus.0 = Some(id);
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
        .add_systems(PreUpdate, focus_keys)
        .add_systems(Update, reset_focus);
}

fn to_dir(dir: CompassQuadrant) -> Dir2 {
    use CompassQuadrant::*;
    match dir {
        // NOTE: I think the Y axis is inverted for UI coordinates.
        North => Dir2::NEG_Y,
        South => Dir2::Y,

        East => Dir2::X,
        West => Dir2::NEG_X,
    }
}

// pub type Focusable = AskyState;

#[derive(SystemParam)]
pub struct FocusParam<'w, 's> {
    query: Query<'w, 's, (Entity, &'static mut Focusable, &'static GlobalTransform)>,
    // nodes: Query<'w, 's, (Entity, &'static Node)>,
    focus: ResMut<'w, private::Focus>,
    keyboard_nav: ResMut<'w, KeyboardNav>,
}

impl<'w, 's> FocusParam<'w, 's> {
    pub fn is_focused(&self, id: Entity) -> bool {
        self.focus.is_focused(id)
    }

    pub fn move_focus(&mut self, dir: CompassQuadrant) {
        let (old_id, old_pos) = if let Some(old_focus) = self.focus.0 {
            if let Ok((id, _, transform)) = self.query.get_mut(old_focus) {
                (id, transform.translation())
            } else {
                self.move_focus_from(None);
                return;
            }
        } else {
            self.move_focus_from(None);
            return;
        };
        let dir: Dir2 = to_dir(dir);
        if let Some((min_id, _min_dist)) = focus_next_wrap(dir, (old_id, old_pos.xy()), || {
            self.query
                .iter()
                .map(|(id, _, transform)| (id, transform.translation().xy()))
        }) {
            info!("focus to {min_id}");
            self.move_focus_to(min_id);
        } else {
            warn!("no focus found");
        }
    }

    pub fn move_focus_to(&mut self, id: Entity) {
        if let Some(old_focus) = self.focus.0.take() {
            if let Ok((_, mut focusable, _)) = self.query.get_mut(old_focus) {
                // Touch the old one so it knows it's no longer the focus.
                focusable.touch()
            }
        }
        self.focus.0 = Some(id);

        if let Ok((_, mut focusable, _)) = self.query.get_mut(id) {
            // Touch the old one so it knows it's no longer the focus.
            focusable.touch()
        }
    }

    pub fn move_focus_from(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(focus_id) = id_maybe.into().or(self.focus.0) {
            // We're moving from a definite id.
            let mut seen_id = false;
            let mut result = None;
            for (id, focusable, _) in &self.query {
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
                let (_, mut focusable, _) = self.query.get_mut(id).unwrap();
                focusable.touch();
            }
            self.focus.0 = result;
        } else {
            // We're moving to any available id.
            self.focus.0 = self
                .query
                .iter_mut()
                .find(|(_, focusable, _)| !focusable.block)
                .map(|(id, mut focusable, _)| {
                    focusable.touch();
                    id
                });
        }
    }

    pub fn keyboard_nav(&self) -> bool {
        self.keyboard_nav.0
    }

    pub fn set_keyboard_nav(&mut self, on: bool) {
        self.keyboard_nav.0 = on;
    }

    pub fn block_and_move(&mut self, id_maybe: impl Into<Option<Entity>>) {
        let id = id_maybe.into();
        self.block(id);
        self.move_focus_from(id);
    }

    pub fn is_blocked(&self, id: Entity) -> bool {
        self.query
            .get(id)
            .map(|(_, focusable, _)| focusable.block)
            .unwrap_or(true)
    }

    pub fn block(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(id) = id_maybe.into().or(self.focus.0) {
            // self.commands.entity(id).insert(Blocked);
            self.query
                .get_mut(id)
                .map(|(_, mut focus, _)| focus.block = true)
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
                .map(|(_, mut focus, _)| focus.block = false)
                .expect("no Focusable");
        } else {
            warn!("No id to unblock");
        }
    }
}

fn focus_keys(input: Res<ButtonInput<KeyCode>>, mut focus: FocusParam) {
    if !focus.keyboard_nav()
        || !input.any_just_pressed([
            KeyCode::ArrowUp,
            KeyCode::ArrowDown,
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
        ])
    {
        return;
    }

    if input.just_pressed(KeyCode::ArrowUp) {
        focus.move_focus(CompassQuadrant::North);
    } else if input.just_pressed(KeyCode::ArrowDown) {
        focus.move_focus(CompassQuadrant::South);
    } else if input.just_pressed(KeyCode::ArrowLeft) {
        focus.move_focus(CompassQuadrant::West);
    } else if input.just_pressed(KeyCode::ArrowRight) {
        focus.move_focus(CompassQuadrant::East);
    }
}

#[allow(dead_code)]
fn focus_on_tab(input: Res<ButtonInput<KeyCode>>, mut focus: FocusParam) {
    if input.just_pressed(KeyCode::Tab) {
        focus.move_focus_from(None);
    }
}

/// Reset focus if None or focus is blocked.
fn reset_focus(mut focus: FocusParam) {
    match focus.focus.0 {
        None => focus.move_focus_from(None),
        Some(id) => {
            if focus.is_blocked(id) {
                focus.move_focus_from(None)
            }
        }
    }
}

fn focus_next_rev<T>(
    dir: Dir2,
    curr: (T, Vec2),
    elements: impl Iterator<Item = (T, Vec2)>,
) -> Option<(T, f32)>
where
    T: PartialEq + Copy + Debug,
{
    let (curr_id, curr_pos) = curr;
    elements
        .filter_map(|(id, pos)| {
            if id == curr_id {
                None
            } else {
                let delta = pos - curr_pos;
                let dirdist = delta.dot(*dir);
                (dirdist > 0.0).then_some((id, dirdist))
            }
        })
        .max_by(|a, b| a.1.total_cmp(&b.1))
}

fn focus_next_wrap<T, I>(dir: Dir2, curr: (T, Vec2), elements: impl Fn() -> I) -> Option<(T, f32)>
where
    T: PartialEq + Copy + Debug,
    I: Iterator<Item = (T, Vec2)>,
{
    focus_next(dir, curr, elements()).or_else(|| focus_next_rev(-dir, curr, elements()))
}

fn focus_next<T>(
    dir: Dir2,
    curr: (T, Vec2),
    elements: impl Iterator<Item = (T, Vec2)>,
) -> Option<(T, f32)>
where
    T: PartialEq + Copy + Debug,
{
    let (curr_id, curr_pos) = curr;
    elements
        .filter_map(|(id, pos)| {
            // dbg!(id, pos);
            if id == curr_id {
                None
            } else {
                let delta = pos - curr_pos;
                let dirdist = delta.dot(*dir);
                (dirdist > 0.0).then_some((id, dirdist))
            }
        })
        .min_by(|a, b| a.1.total_cmp(&b.1))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_right() {
        let elements = [(0, Vec2::ZERO), (1, Vec2::X)];
        assert_eq!(
            focus_next(
                to_dir(CompassQuadrant::East),
                elements[0],
                elements.into_iter()
            ),
            Some((1, 1.0))
        );
    }

    #[test]
    fn two_right() {
        let elements = [(0, Vec2::ZERO), (1, Vec2::X), (2, 2.0 * Vec2::X)];
        assert_eq!(
            focus_next(
                to_dir(CompassQuadrant::East),
                elements[0],
                elements.into_iter()
            ),
            Some((1, 1.0))
        );
    }

    #[test]
    fn none_right() {
        let elements = [(0, Vec2::ZERO), (1, Vec2::NEG_X)];
        assert_eq!(
            focus_next(
                to_dir(CompassQuadrant::East),
                elements[0],
                elements.into_iter()
            ),
            None
        );
    }

    #[test]
    fn none_right_wrap() {
        let elements = [(0, Vec2::ZERO), (1, Vec2::NEG_X)];
        assert_eq!(
            focus_next_wrap(to_dir(CompassQuadrant::East), elements[0], || elements
                .into_iter())
            .map(|x| x.0),
            Some(1)
        );
    }

    #[test]
    fn two_left_wrap() {
        let elements = [(0, Vec2::ZERO), (1, Vec2::NEG_X), (2, 2.0 * Vec2::NEG_X)];
        assert_eq!(
            focus_next_wrap(to_dir(CompassQuadrant::East), elements[0], || elements
                .into_iter())
            .map(|x| x.0),
            Some(2)
        );
    }

    #[test]
    fn checkbox_group() {
        let elements = [
            (4, Vec2::new(258.0, 12.0)),
            (5, Vec2::new(384.0, 12.0)),
            (6, Vec2::new(510.0, 12.0)),
            (8, Vec2::new(288.0, 60.0)),
            (9, Vec2::new(288.0, 84.0)),
            (10, Vec2::new(288.0, 108.0)),
        ];

        assert_eq!(
            focus_next_wrap(to_dir(CompassQuadrant::East), elements[0], || elements
                .into_iter())
            .map(|x| x.0),
            Some(8)
        );
    }
}
