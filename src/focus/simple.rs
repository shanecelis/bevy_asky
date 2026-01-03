use bevy::{ecs::system::SystemParam, math::CompassQuadrant, prelude::*,
input_focus::{
        InputDispatchPlugin, InputFocus,
    },
};
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A rudimentary focus parameter
///
/// This is only used to test whether an entity is focused.
#[derive(SystemParam)]
pub struct Focus<'w> {
    focus: ResMut<'w, InputFocus>,
}

impl Focus<'_> {
    /// Is entity focused?
    pub fn is_focused(&self, id: Entity) -> bool {
        self.focus.get() == Some(id)
    }

    /// Focus on given entity.
    pub fn focus_on(&mut self, id: Entity) {
        self.focus.0 = Some(id);
    }
}

/// Turn on or off keyboard navigation for focus.
#[derive(Resource, Default, Debug)]
pub struct KeyboardNav(bool);

/// Atomic counter for generating unique creation IDs for Focusable components.
static FOCUSABLE_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Marker for Focusable components
#[derive(Component, Clone, Reflect)]
pub struct Focusable {
    version: usize,
    block: bool,
    /// Creation order ID, used for ordering focusable entities.
    pub created: usize,
}

impl Default for Focusable {
    fn default() -> Self {
        Self {
            version: 0,
            block: false,
            created: FOCUSABLE_COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }
}

impl Focusable {
    /// Modify the focusable.
    ///
    /// Useful for view that can filter by `Changed<Focusable>`.
    fn touch(&mut self) {
        self.version += 1;
    }
}

// #[derive(Component, Default, Debug)]
// pub struct Blocked;

pub(crate) fn plugin(app: &mut App) {
    // InputDispatchPlugin is needed for InputFocus to work in Bevy 0.17,
    // but it requires message types that aren't available in test mode.
    // Only add it when not in test configuration.
    #[cfg(not(test))]
    app.add_plugins(InputDispatchPlugin);
    
    app
        .register_type::<Focusable>()
        .insert_resource(KeyboardNav(true))
        .add_systems(PreUpdate, (sync_focus_to_focusable, focus_keys))
        .add_systems(Update, reset_focus);
}

/// Sync InputFocus changes to Focusable components for change detection.
fn sync_focus_to_focusable(
    input_focus: Res<InputFocus>,
    mut focusables: Query<(Entity, &mut Focusable)>,
    mut last_focus: Local<Option<Entity>>,
) {
    let current_focus = input_focus.get();
    
    // If focus changed, touch the old and new focusable
    if *last_focus != current_focus {
        // Touch the old focused entity
        if let Some(old_id) = *last_focus {
            if let Ok((_, mut focusable)) = focusables.get_mut(old_id) {
                focusable.touch();
            }
        }
        
        // Touch the new focused entity
        if let Some(new_id) = current_focus {
            if let Ok((_, mut focusable)) = focusables.get_mut(new_id) {
                focusable.touch();
            }
        }
        
        *last_focus = current_focus;
    }
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

/// A rich focus parameter
#[derive(SystemParam)]
pub struct FocusParam<'w, 's> {
    query: Query<'w, 's, (Entity, &'static mut Focusable)>,
    // nodes: Query<'w, 's, (Entity, &'static Node)>,
    focus: ResMut<'w, InputFocus>,
    keyboard_nav: ResMut<'w, KeyboardNav>,
}

impl FocusParam<'_, '_> {
    /// Is entity focused?
    pub fn is_focused(&self, id: Entity) -> bool {
        self.focus.get() == Some(id)
    }

    /// Move the focus in a direction if possible.
    /// 
    /// For directional navigation, we use creation order:
    /// - North/Up: previous (lower created ID)
    /// - South/Down: next (higher created ID)
    /// - East/Right: next (higher created ID)
    /// - West/Left: previous (lower created ID)
    pub fn move_focus(&mut self, dir: CompassQuadrant) {
        let old_created = if let Some(old_focus) = self.focus.0 {
            if let Ok((_, focusable)) = self.query.get(old_focus) {
                focusable.created
            } else {
                self.move_focus_from(None);
                return;
            }
        } else {
            self.move_focus_from(None);
            return;
        };
        
        use CompassQuadrant::*;
        let candidates: Vec<_> = self.query
            .iter()
            .filter(|(id, focusable)| {
                *id != self.focus.0.unwrap() && !focusable.block
            })
            .map(|(id, focusable)| (id, focusable.created))
            .collect();
        
        let result = match dir {
            North | West => {
                // Previous: find highest created ID that is less than current
                candidates
                    .iter()
                    .filter(|(_, created)| *created < old_created)
                    .max_by_key(|(_, created)| *created)
                    .map(|(id, _)| *id)
            }
            South | East => {
                // Next: find lowest created ID that is greater than current
                candidates
                    .iter()
                    .filter(|(_, created)| *created > old_created)
                    .min_by_key(|(_, created)| *created)
                    .map(|(id, _)| *id)
            }
        };
        
        // If no result in direction, wrap around
        let result = result.or_else(|| {
            match dir {
                North | West => {
                    // Wrap: find highest created ID overall
                    candidates.iter().max_by_key(|(_, created)| *created).map(|(id, _)| *id)
                }
                South | East => {
                    // Wrap: find lowest created ID overall
                    candidates.iter().min_by_key(|(_, created)| *created).map(|(id, _)| *id)
                }
            }
        });
        
        if let Some(id) = result {
            self.move_focus_to(id);
        }
    }

    /// Move focus to an entity.
    pub fn move_focus_to(&mut self, id: Entity) {
        self.focus.0 = Some(id);
    }

    /// Move focus away from an entity.
    /// 
    /// Uses creation order: moves to the next unblocked entity after the current one.
    pub fn move_focus_from(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(focus_id) = id_maybe.into().or(self.focus.0) {
            // Get the creation order of the current focus
            let current_created = self.query
                .get(focus_id)
                .map(|(_, focusable)| focusable.created)
                .unwrap_or(0);
            
            // Find the next unblocked entity with higher creation order
            let mut candidates: Vec<_> = self.query
                .iter()
                .filter(|(id, focusable)| {
                    *id != focus_id && !focusable.block
                })
                .map(|(id, focusable)| (id, focusable.created))
                .collect();
            
            // Sort by creation order
            candidates.sort_by_key(|(_, created)| *created);
            
            // Find next after current, or wrap to first
            let result = candidates
                .iter()
                .find(|(_, created)| *created > current_created)
                .map(|(id, _)| *id)
                .or_else(|| candidates.first().map(|(id, _)| *id));
            
            self.focus.0 = result;
        } else {
            // We're moving to any available id - pick the first (lowest created ID).
            let mut candidates: Vec<_> = self.query
                .iter()
                .filter(|(_, focusable)| !focusable.block)
                .map(|(id, focusable)| (id, focusable.created))
                .collect();
            
            candidates.sort_by_key(|(_, created)| *created);
            self.focus.0 = candidates.first().map(|(id, _)| *id);
        }
    }

    /// Is keyboard navigation on?
    pub fn keyboard_nav(&self) -> bool {
        self.keyboard_nav.0
    }

    /// Set keyboard navigation.
    pub fn set_keyboard_nav(&mut self, on: bool) {
        self.keyboard_nav.0 = on;
    }

    /// Block focus and move to.
    pub fn block_and_move(&mut self, id_maybe: impl Into<Option<Entity>>) {
        let id = id_maybe.into();
        self.block(id);
        self.move_focus_from(id);
    }

    /// Is entity blocked?
    pub fn is_blocked(&self, id: Entity) -> bool {
        self.query
            .get(id)
            .map(|(_, focusable)| focusable.block)
            .unwrap_or(true)
    }

    /// Block focus on current or given entity.
    pub fn block(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(id) = id_maybe.into().or(self.focus.0) {
            if let Ok((_, mut focus)) = self.query.get_mut(id) {
                focus.block = true;
            } else {
                // Entity doesn't have Focusable component or was despawned
                // This can happen if the entity was despawned after submit
                warn!("Cannot block entity {:?}: no Focusable component", id);
            }
        } else {
            warn!("No id to block");
        }
    }

    /// Unblock focus on current or given entity.
    pub fn unblock(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(id) = id_maybe.into().or(self.focus.0) {
            if let Ok((_, mut focus)) = self.query.get_mut(id) {
                focus.block = false;
            } else {
                // Entity doesn't have Focusable component or was despawned
                warn!("Cannot unblock entity {:?}: no Focusable component", id);
            }
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
