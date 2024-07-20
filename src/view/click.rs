//! Click event and plugin for Bevy.
use bevy::ecs::entity::EntityHashMap;
use bevy::prelude::*;

/// A trigger event that signifies a click on a button, which is a mouse button
/// press and release on the same object.
#[derive(Event, Debug)]
pub struct Click;

/// Adds a system that triggers a [Click] event when an entity with an
/// [Interaction] component is pressed and released on that same UI element.
pub fn plugin(app: &mut App) {
    app.add_systems(Update, button_click);
}

/// [Interaction::Pressed] is not the same as a click, which is a press and
/// release on the same UI object. This system notes the last state of
/// interaction. If that state changes from [Interaction::Pressed] to
/// [Interaction::Hovered] then it will trigger a [Click] event targeting the
/// Entity with the [Interaction] component.
///
/// TODO: The `Local<HashMap>` should be reset or drop elements that are stale
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
