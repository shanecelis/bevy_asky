use bevy::{
    ecs::system::SystemParam,
    input_focus::{FocusCause, InputFocus},
    math::CompassQuadrant,
    prelude::*,
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
        self.focus.set(id, FocusCause::Navigated);
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
            created: FOCUSABLE_COUNTER.fetch_add(1, Ordering::SeqCst),
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
    app.register_type::<Focusable>()
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
        if let Some(old_id) = *last_focus
            && let Ok((_, mut focusable)) = focusables.get_mut(old_id)
        {
            focusable.touch();
        }

        // Touch the new focused entity
        if let Some(new_id) = current_focus
            && let Ok((_, mut focusable)) = focusables.get_mut(new_id)
        {
            focusable.touch();
        }

        *last_focus = current_focus;
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
    fn current_focus(&self) -> Option<Entity> {
        self.focus.get()
    }

    fn set_focus(&mut self, id: Option<Entity>) {
        if let Some(id) = id {
            self.focus.set(id, FocusCause::Navigated);
        } else {
            self.focus.clear();
        }
    }

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
        let Some(current_focus) = self.current_focus() else {
            self.move_focus_from(None);
            return;
        };

        let old_created = if let Ok((_, focusable)) = self.query.get(current_focus) {
            focusable.created
        } else {
            self.move_focus_from(None);
            return;
        };

        use CompassQuadrant::*;
        let candidates: Vec<_> = self
            .query
            .iter()
            .filter(|(id, focusable)| *id != current_focus && !focusable.block)
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
                    candidates
                        .iter()
                        .max_by_key(|(_, created)| *created)
                        .map(|(id, _)| *id)
                }
                South | East => {
                    // Wrap: find lowest created ID overall
                    candidates
                        .iter()
                        .min_by_key(|(_, created)| *created)
                        .map(|(id, _)| *id)
                }
            }
        });

        if let Some(id) = result {
            self.move_focus_to(id);
        }
    }

    /// Move focus to an entity.
    pub fn move_focus_to(&mut self, id: Entity) {
        self.set_focus(Some(id));
    }

    /// Move focus away from an entity.
    ///
    /// Uses creation order: moves to the next unblocked entity after the current one.
    pub fn move_focus_from(&mut self, id_maybe: impl Into<Option<Entity>>) {
        if let Some(focus_id) = id_maybe.into().or(self.current_focus()) {
            // Get the creation order of the current focus
            let current_created = self
                .query
                .get(focus_id)
                .map(|(_, focusable)| focusable.created);

            // Find the next unblocked entity with higher creation order
            let mut candidates: Vec<_> = self
                .query
                .iter()
                .filter(|(id, focusable)| *id != focus_id && !focusable.block)
                .map(|(id, focusable)| (id, focusable.created))
                .collect();

            // Sort by creation order
            candidates.sort_by_key(|(_, created)| *created);

            // Find next after current, or wrap to first
            let result = candidates
                .iter()
                .find_map(|(id, created)| {
                    current_created
                        .map(|id| *created > id)
                        .unwrap_or(true)
                        .then_some(*id)
                })
                .or_else(|| candidates.first().map(|(id, _)| *id));

            self.set_focus(result);
        } else {
            // We're moving to any available id - pick the first (lowest created ID).
            let mut candidates: Vec<_> = self
                .query
                .iter()
                .filter(|(_, focusable)| !focusable.block)
                .map(|(id, focusable)| (id, focusable.created))
                .collect();

            candidates.sort_by_key(|(_, created)| *created);
            self.set_focus(candidates.first().map(|(id, _)| *id));
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
        if let Some(id) = id_maybe.into().or(self.current_focus()) {
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
        if let Some(id) = id_maybe.into().or(self.current_focus()) {
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
    match focus.current_focus() {
        None => focus.move_focus_from(None),
        Some(id) => {
            if focus.is_blocked(id) {
                focus.move_focus_from(None)
            }
        }
    }
}
