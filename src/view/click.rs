/* Original code Copyright (c) 2024 Shane Celis[1]
   Licensed under the MIT License[2] or Apache License v2.0[3]

   [1]: https://mastodon.gamedev.place/@shanecelis
   [2]: https://opensource.org/licenses/MIT
   [3]: https://www.apache.org/licenses/LICENSE-2.0
*/

//! Click event for Bevy
//!
//! A click is a press and release of a mouse button on the same UI element, or
//! an Enter/Space activation while the button is focused. A press that leaves a
//! UI element and releases elsewhere is not considered a click.
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
//! # use bevy::ui_widgets::Button;
//! # use bevy_asky::view::click::{self, Click};
//! fn setup(mut commands: Commands) {
//!     commands.spawn(Button)
//!         .observe(|trigger: On<Click>|
//!             eprintln!("Clicked on {}", trigger.event_target()));
//! }
//! ```
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, Button, ButtonPlugin};

/// Adds an observer that triggers a [Click] when a [Button] is activated.
///
/// Also adds [ButtonPlugin] if it is not already present.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_asky::view::click;
/// let mut app = App::new();
/// app.add_plugins(click::plugin);
/// ```
pub fn plugin(app: &mut App) {
    if !app.is_plugin_added::<ButtonPlugin>() {
        app.add_plugins(ButtonPlugin);
    }
    app.add_observer(button_click);
}

/// A trigger event that signifies a click on a button.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy::ui_widgets::Button;
/// # use bevy_asky::view::click::{self, Click};
/// fn setup(mut commands: Commands) {
///     commands.spawn(Button)
///         .observe(|trigger: On<Click>|
///             eprintln!("Clicked on {}", trigger.event_target()));
/// }
/// ```
/// [1]: https://gist.github.com/shanecelis/06b2d1a598e1e06d0a00671596e9f74f
#[derive(EntityEvent, Debug)]
pub struct Click {
    /// The entity that was clicked.
    pub entity: Entity,
}

/// Forwards a button [`Activate`] into a [`Click`] on that same entity.
fn button_click(
    activate: On<Activate>,
    buttons: Query<(), With<Button>>,
    mut commands: Commands,
) {
    let entity = activate.event_target();
    if buttons.contains(entity) {
        commands.trigger(Click { entity });
    }
}
