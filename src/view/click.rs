/* Original code Copyright (c) 2024 Shane Celis[1]
   Licensed under the MIT License[2] or Apache License v2.0[3]

   [1]: https://mastodon.gamedev.place/@shanecelis
   [2]: https://opensource.org/licenses/MIT
   [3]: https://www.apache.org/licenses/LICENSE-2.0
*/

//! Click event for Bevy
//!
//! [Interaction::Pressed] is not the same as a click. A click is a press and
//! release of a mouse button on the same UI element. A press that leaves a UI
//! element and releases elsewhere is not considered a click.
//!
//! # Setup
//!
//! Add the plugin.
//!
//! ```
//! # use bevy::prelude::*;
//! # use bevy_asky::view::click;
//! let mut app = App::new();
//! app.add_plugins(click::plugin);
//! ```
//!
//! # Usage
//!
//! Add an observer on a button.
//!
//! ```
//! # use bevy::prelude::*;
//! # use bevy_asky::view::click::{self, Click};
//! fn setup(mut commands: Commands) {
//!     commands.spawn(ButtonBundle::default())
//!         .observe(|trigger: Trigger<Click>|
//!             eprintln!("Clicked on {}", trigger.target()));
//! }
//! ```
use bevy::ecs::entity::EntityHashMap;
use bevy::prelude::*;

/// Adds a system that triggers a [Click] event when [Button] with an
/// [Interaction] component is pressed and released.
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

/// Excerpt from this gist[1].
///
/// * * *
///
/// A trigger event that signifies a click on a button.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_asky::view::click::{self, Click};
/// fn setup(mut commands: Commands) {
///     commands.spawn(ButtonBundle::default())
///         .observe(|trigger: Trigger<Click>|
///             eprintln!("Clicked on {}", trigger.target()));
/// }
/// ```
/// [1]: https://gist.github.com/shanecelis/06b2d1a598e1e06d0a00671596e9f74f
#[derive(Event, Debug)]
pub struct Click;

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
