use bevy::prelude::*;
use bevy_asky::view;

pub fn views(app: &mut App) {
    #[cfg(feature = "ascii")]
    app.add_plugins(view::ascii::plugin);
    #[cfg(feature = "color")]
    app.add_plugins(view::color::plugin);
    #[cfg(feature = "button")]
    app.add_plugins(view::button::plugin);
}
