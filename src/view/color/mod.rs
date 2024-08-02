use crate::construct::*;
use bevy::prelude::*;
mod confirm;
mod text;

#[derive(Component)]
pub struct View<T>(T);

#[derive(Debug, Resource, Component)]
struct ColorView {
    text_color: Srgba,
    background: Option<Srgba>,
    highlight: Srgba,
    complete: Srgba,
    answer: Srgba,
    lowlight: Srgba,
}

impl Default for ColorView {
    fn default() -> Self {
        Self {
            text_color: Srgba::WHITE,
            background: None,
            highlight: Srgba::hex("80ADFA").unwrap(),
            complete: Srgba::hex("94DD8D").unwrap(),
            answer: Srgba::hex("FFB9E8").unwrap(),
            lowlight: Srgba::hex("5A607A").unwrap(),
        }
    }
}

pub fn plugin(app: &mut App) {
    app
        .add_plugins(confirm::plugin)
        .add_plugins(text::plugin)
        .insert_resource(ColorView::default());
}
