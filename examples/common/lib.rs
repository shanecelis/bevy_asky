use bevy::prelude::*;
use bevy_asky::view;

pub fn views(app: &mut App) {
    app
        .add_plugins(view::ascii::plugin)
        .add_plugins(view::color::plugin)
        .add_systems(Update, view::color::replace_view);

    #[cfg(feature = "button")]
    app.add_plugins(view::button::plugin);
}
