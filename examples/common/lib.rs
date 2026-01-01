use bevy::prelude::*;
use bevy_asky::view;

pub fn set_initial_focus(
    mut input_focus: ResMut<bevy::input_focus::InputFocus>,
    query: Query<Entity, With<bevy_asky::focus::Focusable>>,
) {
    if input_focus.get().is_none() {
       if let Some(entity) = query.iter().next() {
           input_focus.set(entity);
       }
    }
}


pub fn views(app: &mut App) {
    #[cfg(feature = "ascii")]
    app.add_plugins(view::ascii::plugin);
    #[cfg(feature = "color")]
    app.add_plugins(view::color::plugin);
    #[cfg(feature = "button")]
    app.add_plugins(view::button::plugin);
    #[cfg(not(any(feature = "color", feature = "ascii", feature = "button")))]
    panic!("Please use a view feature: ascii, color, or button.");
}

#[cfg(all(not(feature = "color"), feature = "ascii"))]
pub use view::ascii::View;
/// This is the view that the examples will use.
#[cfg(feature = "color")]
pub use view::color::View;
