use super::{Feedback, Password, Prompt};
use crate::construct::*;
use crate::{AskyEvent, AskyState, CursorDirection, Error, StringCursor, Submitter};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use bevy_alt_ui_navigation_lite::prelude::*;
use std::borrow::Cow;

pub fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, text_controller);
}

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
pub struct TextField;

impl Submitter for TextField {
    type Out = String;
    // fn submit(&self) -> Result<Self::Out, Error> {
    //     Ok(self.value.clone())
    // }
}

impl Construct for TextField {
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

        Ok(TextField)
    }
}

fn text_controller(
    mut query: Query<
        (Entity, &mut AskyState, &mut StringCursor, &Focusable),
        Or<(With<TextField>, With<Password>)>,
    >,
    mut input: EventReader<KeyboardInput>,
    mut commands: Commands,
) {
    for (id, mut state, mut text_state, focusable) in query.iter_mut() {
        if FocusState::Focused != focusable.state() {
            continue;
        }
        if let AskyState::Reading = *state {
            for ev in input.read() {
                if ev.state != ButtonState::Pressed {
                    continue;
                }
                match &ev.logical_key {
                    Key::Character(s) => {
                        for c in s.chars() {
                            text_state.insert(c);
                        }
                    }
                    Key::Space => text_state.insert(' '),
                    Key::Backspace => text_state.backspace(),
                    Key::Delete => text_state.delete(),
                    Key::ArrowLeft => text_state.move_cursor(CursorDirection::Left),
                    Key::ArrowRight => text_state.move_cursor(CursorDirection::Right),
                    Key::Enter => {
                        commands.trigger_targets(AskyEvent(Ok(text_state.value.clone())), id);
                        *state = AskyState::Complete;
                    }
                    Key::Escape => {
                        commands.trigger_targets(AskyEvent::<String>(Err(Error::Cancel)), id);
                        commands.entity(id).insert(Feedback::error("canceled"));

                        *state = AskyState::Error;
                    }
                    x => info!("Unhandled key {x:?}"),
                }
            }
        }
    }
}
