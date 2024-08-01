use bevy::prelude::*;

mod confirm;
mod input;
pub use confirm::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(confirm::plugin);
}

