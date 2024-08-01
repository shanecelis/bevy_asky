use bevy::prelude::*;

mod confirm;
mod text;
pub use confirm::*;
pub use text::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(confirm::plugin);
    app.add_plugins(text::plugin);
}

