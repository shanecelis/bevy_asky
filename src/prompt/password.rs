use super::{Feedback, Prompt};
use crate::construct::*;
use crate::{AskyEvent, AskyState, Error, CursorDirection, StringCursor};
use bevy::{
    a11y::Focus,
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use bevy_ui_navigation::prelude::*;
use std::borrow::Cow;

pub fn plugin(app: &mut App) {}

/// Prompt to get one-line user input.
///
/// # Key Events
///
/// | Key         | Action                       |
/// | ----------- | ---------------------------- |
/// | `Enter`     | Submit current/initial value |
/// | `Backspace` | Delete previous character    |
/// | `Delete`    | Delete current character     |
/// | `Left`      | Move cursor left             |
/// | `Right`     | Move cursor right            |
///
/// # Examples
///
/// ```no_run
/// use asky::prelude::*;
///
/// # fn main() -> Result<(), Error> {
/// # #[cfg(feature = "terminal")]
/// let name = Input::new("What is your name?").prompt()?;
///
/// # #[cfg(feature = "terminal")]
/// println!("Hello, {}!", name);
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Component)]
pub struct Password;
impl Construct for Password {
    type Props = Cow<'static, str>;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let state: AskyState = context.construct(AskyState::default())?;
        let input_state = StringCursor::default();
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(Prompt(props))
            .insert(input_state)
            .insert(Focusable::default())
            .insert(state);

        context.world.flush();

        Ok(Password)
    }
}
