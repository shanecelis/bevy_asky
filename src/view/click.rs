//! Click event for Bevy.
//!
//! [Interaction::Pressed] is not the same as a click, which is a press and
//! release of a mouse button on the same UI object. use bevy::ecs::entity::EntityHashMap;
use bevy::prelude::*;
use bevy::ecs::entity::EntityHashMap;

/// A trigger event that signifies a click on a button, which is a mouse button
/// press and release on the same object.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_asky::view::click::{self, Click};
/// fn setup(mut commands: Commands) {
///     commands.spawn(ButtonBundle::default())
///         .observe(|trigger: Trigger<Click>|
///             eprintln!("Click on {}", trigger.entity()));
/// }
#[derive(Event, Debug)]
pub struct Click;

/// Adds a system that triggers a [Click] event when an entity with an
/// [Interaction] component is pressed and released on that same UI element.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_asky::view::click;
/// let mut app = App::new();
/// app.add_plugins(click::plugin);
/// ```
pub fn plugin(app: &mut App) {
    app.add_systems(Update, button_click);
}
/// This system looks at [Button] [Interaction] changes. If that state changes
/// from [Interaction::Pressed] to [Interaction::Hovered] then it will trigger a
/// [Click] event targeting the Entity with the [Interaction] component.
///
/// TODO: The `Local<EntityHashMap>` should be reset or drop elements that are stale
/// so it doesn't grow unbounded.
fn button_click(
    mut interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
    mut last_state: Local<EntityHashMap<Interaction>>,
    mut commands: Commands,
) {
    for (id, interaction) in &mut interaction_query {
        let last = last_state.get(&id);
        if *interaction == Interaction::Hovered && matches!(last, Some(Interaction::Pressed)) {
            commands.trigger_targets(Click, id);
        }
        last_state.insert(id, *interaction);
    }
}
