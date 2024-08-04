use crate::construct::*;
use super::{Feedback, Prompt, Password};
use bevy::{
    input::{
        ButtonState,
        keyboard::{KeyboardInput, Key}
    },
    a11y::Focus,
    prelude::*
};
use crate::{AskyEvent, AskyState, Error, StringCursor, InputDirection};
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
    // Message used to display in the prompt
    // pub message: Cow<'static, str>,


// impl TextModel for Input {
//     fn message<'a>(&'a self) -> &'a str { &self.message }
//     fn placeholder<'a>(&'a self) -> Option<&'a str> { self.placeholder.as_deref() }
//     fn default_value<'a>(&'a self) -> Option<&'a str> { self.default_value.as_deref() }
// }

// pub struct StringCursor {
//     /// Input state for the prompt
//     pub input: StringCursor,
//     /// State of the validation of the user input
//     // pub validator_result: Result<(), Cow<'a, str>>,
//     // validator: Option<Box<InputValidator<'a>>>,
// }
//

// impl From<Cow<'static, str>> for TextField {
//     fn from(message: Cow<'static, str>) -> Self {
//         Self {
//             message,
//         }
//     }
// }

// impl From<&'static str> for TextField {
//     fn from(message: &'static str) -> Self {
//         Self {
//             message: message.into(),
//         }
//     }
// }

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
            .insert(state);

        context.world.flush();

        Ok(TextField)
    }
}

// fn trigger<T: Send + Sync + 'static, I: Iterator<Item=(Entity, Result<T, Error>)>>(In(iter): In<I>, mut commands: Commands, mut query: Query<&mut AskyState>) {
//     for (id, result) in iter {
//         if let Ok(mut state) = query.get_mut(id) {
//             *state = if result.is_ok() {
//                 AskyState::Complete
//             } else {
//                 AskyState::Error
//             };
//         }
//         commands.trigger_targets(AskyEvent(result), id);
//     }
// }

fn text_controller(
    mut query: Query<(Entity, &mut AskyState, &mut StringCursor), Or<(With<TextField>, With<Password>)>>,
    mut input: EventReader<KeyboardInput>,
    mut commands: Commands,
    focus: Option<Res<Focus>>,
) {
    let focused = focus.map(|res| res.0).unwrap_or(None);
    for (id, mut state, mut text_state) in query.iter_mut() {
        if focused.map(|x| x != id).unwrap_or(false) {
            continue;
        }
        match *state {
            AskyState::Reading => {
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
                        Key::ArrowLeft => text_state.move_cursor(InputDirection::Left),
                        Key::ArrowRight => text_state.move_cursor(InputDirection::Right),
                        Key::Enter => {
                            commands.trigger_targets(AskyEvent(Ok(text_state.value.clone())), id);
                            *state = AskyState::Complete;
                        }
                        Key::Escape => {
                            commands.trigger_targets(AskyEvent::<String>(Err(Error::Cancel)), id);
                            commands.entity(id).insert(Feedback::error("canceled"));

                            *state = AskyState::Error;
                        }
                        x => info!("Unhandled key {x:?}")
                    }
                }
            }
            _ => (),
        }
    }
}
