use bevy::prelude::*;

mod confirm;
mod text;
mod number;
pub use confirm::*;
pub use text::*;
pub use number::*;

#[derive(Component)]
pub struct Prompt(pub String);
#[derive(Component)]
pub struct Placeholder(pub String);
#[derive(Component)]
pub struct DefaultValue<T: std::fmt::Display>(pub T);



pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(confirm::plugin);
    app.add_plugins(text::plugin);
    app.add_plugins(number::plugin);
}

