use crate::construct::*;
use bevy::{
    input::{
        ButtonState,
        keyboard::{KeyboardInput, Key}
    },
    a11y::Focus,
    prelude::*
};
use crate::{AskyEvent, AskyState, Error};
use std::borrow::Cow;

pub enum Direction {
    Left,
    Right,
}

/// State of the user input for read-line text prompts (like [`TextInput`]).
///
/// **Note**: This structure is not expected to be created, but it can be consumed when using a custom formatter.
#[derive(Debug, PartialEq, Eq, Default, Component)]
pub struct TextInputState {
    /// Current value of the input.
    pub value: String,
    /// Current index of the cursor (kept on ut8 char boundaries).
    pub index: usize,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, text_controller);
}

impl TextInputState {
    pub(crate) fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
        self.index = self.value.len();
    }

    pub(crate) fn insert(&mut self, ch: char) {
        self.value.insert(self.index, ch);
        self.index += ch.len_utf8();
    }

    pub(crate) fn backspace(&mut self) {
        if self.index >= self.value.len() {
            self.value.pop();
            self.index = self.value.len();
        } else if self.index >= 0 {
            let start = floor_char_boundary(&self.value, self.index.saturating_sub(1));
            let _ = self.value.drain(start..self.index);
            self.index = start;
            // self.index -= c.len_utf8();
        }
    }

    pub(crate) fn delete(&mut self) {
        if !self.value.is_empty() && self.index < self.value.len() {
            self.value.remove(self.index);
        }
    }

    pub(crate) fn move_cursor(&mut self, position: Direction) {
        self.index = match position {
            // TODO: When round_char_boundary is stabilized, use std's impl.
            // Direction::Left => self.value.floor_char_boundary(self.index.saturating_sub(1)),
            Direction::Left => floor_char_boundary(&self.value, self.index.saturating_sub(1)),
            // Direction::Right => self.value.ceil_char_boundary(self.index + 1),
            Direction::Right => ceil_char_boundary(&self.value, self.index + 1),
        }
    }
}

fn floor_char_boundary(s: &str, mut i: usize) -> usize {
    if i > s.len() {
        s.len()
    } else {
        while !s.is_char_boundary(i) {
            i = i.saturating_sub(1);
        }
        i
    }
}

fn ceil_char_boundary(s: &str, mut i: usize) -> usize {
    if i > s.len() {
        s.len()
    } else {
        while !s.is_char_boundary(i) {
            i = i.saturating_add(1);
        }
        i
    }
}

// pub type InputValidator<'a> = dyn Fn(&str) -> Result<(), Cow<'a, str>> + 'a + Send + Sync;

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
/// let name = TextInput::new("What is your name?").prompt()?;
///
/// # #[cfg(feature = "terminal")]
/// println!("Hello, {}!", name);
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TextInput {
    /// Message used to display in the prompt
    pub message: Cow<'static, str>,
    // TextInput state for the prompt
    // pub input: TextInputState,
    /// Placeholder to show when the input is empty
    pub placeholder: Option<Cow<'static, str>>,
    /// Default value to submit when the input is empty
    pub default_value: Option<Cow<'static, str>>,
    // State of the validation of the user input
    // pub validator_result: Result<(), Cow<'a, str>>,
    // validator: Option<Box<InputValidator<'a>>>,
}

// pub struct TextInputState {
//     /// TextInput state for the prompt
//     pub input: TextInputState,
//     /// State of the validation of the user input
//     // pub validator_result: Result<(), Cow<'a, str>>,
//     // validator: Option<Box<InputValidator<'a>>>,
// }
//

impl From<Cow<'static, str>> for TextInput {
    fn from(message: Cow<'static, str>) -> Self {
        Self {
            message,
            placeholder: None,
            default_value: None
        }
    }
}

impl From<&'static str> for TextInput {
    fn from(message: &'static str) -> Self {
        Self {
            message: message.into(),
            placeholder: None,
            default_value: None
        }
    }
}

impl Construct for TextInput {
    type Props = TextInput;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let state: AskyState = context.construct(AskyState::default())?;
        let input_state = TextInputState::default();
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(input_state)
            .insert(state);

        context.world.flush();
        Ok(props)
    }
}

impl TextInput {
    /// Create a new text prompt.
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        TextInput {
            message: message.into(),
            placeholder: None,
            default_value: None,
        }
    }

    /// Set text to show when the input is empty.
    ///
    /// This not will not be submitted when the input is empty.
    pub fn placeholder(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.placeholder = Some(value.into());
        self
    }

    /// Set default value to submit when the input is empty.
    pub fn default(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    // pub(crate) fn validate_to_submit(&mut self) -> bool {
    //     if let Some(validator) = &self.validator {
    //         self.validator_result = validator(self.get_value());
    //     }

    //     self.validator_result.is_ok()
    // }
}

fn text_controller(
    mut query: Query<(Entity, &mut AskyState, &mut TextInputState)>,
    mut input: EventReader<KeyboardInput>,
    mut commands: Commands,
    focus: Option<Res<Focus>>,
) {
    let focused = focus.map(|res| res.0.clone()).unwrap_or(None);
    for (id, mut state, mut text_state) in query.iter_mut() {
        if focused.map(|x| x != id).unwrap_or(false) {
            continue;
        }
        match *state {
            AskyState::Uninit => {
                *state = AskyState::Reading;
            }
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
                        Key::Backspace => text_state.backspace(),
                        Key::Delete => text_state.delete(),
                        Key::ArrowLeft => text_state.move_cursor(Direction::Left),
                        Key::ArrowRight => text_state.move_cursor(Direction::Right),
                        Key::Enter => {
                            commands.trigger_targets(AskyEvent(Ok(text_state.value.clone())), id);
                            *state = AskyState::Complete;
                        }
                        Key::Escape => {
                            commands.trigger_targets(AskyEvent::<String>(Err(Error::Cancel)), id);
                            *state = AskyState::Error;
                        }
                        _ => todo!()
                    }
                }
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_floor_char() {
        let s = "❤️🧡💛💚💙💜";
        assert_eq!(s.len(), 26);
        assert!(!s.is_char_boundary(13));

        let closest = floor_char_boundary(&s, 13);
        assert_eq!(closest, 10);
        assert_eq!(&s[..closest], "❤️🧡");
        assert_eq!(floor_char_boundary(&s, 0), 0);
        assert_eq!(floor_char_boundary(&s, 26), 26);
        assert_eq!(floor_char_boundary(&s, 27), 26);
    }

    #[test]
    fn test_ceil_char() {
        let s = "❤️🧡💛💚💙💜";
        assert_eq!(s.len(), 26);
        assert!(!s.is_char_boundary(13));

        let closest = ceil_char_boundary(&s, 13);
        assert_eq!(closest, 14);
        assert_eq!(&s[..closest], "❤️🧡💛");
        assert_eq!(ceil_char_boundary(&s, 0), 0);
        assert_eq!(ceil_char_boundary(&s, 26), 26);
        assert_eq!(ceil_char_boundary(&s, 27), 26);
    }
}
