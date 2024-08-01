use bevy::prelude::*;

mod confirm;
mod input;
pub use confirm::*;
pub use input::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(confirm::plugin);
    app.add_plugins(input::plugin);
}

