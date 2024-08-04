use bevy::prelude::*;

pub mod ascii;
pub mod button;
pub mod click;
pub mod color;
// pub(crate) mod interaction;
pub(crate) mod widget;


#[derive(Component)]
pub struct Question;

#[derive(Component)]
// pub struct Answer<T>(T);
pub enum Answer<T> {
    Selection(T),
    Final, //(Option<T>)
}
