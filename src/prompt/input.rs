use crate::Error;
use std::borrow::Cow;

pub enum Direction {
    Left,
    Right,
}

// region: TextInput

/// State of the user input for read-line text prompts (like [`TextInput`]).
///
/// **Note**: This structure is not expected to be created, but it can be consumed when using a custom formatter.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct LineInput {
    /// Current value of the input.
    pub value: String,
    /// Current position of the cursor.
    pub col: usize,
}

impl LineInput {
    pub(crate) fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
        self.col = self.value.chars().count();
    }

    pub(crate) fn insert(&mut self, ch: char) {
        self.value.insert(self.col, ch);
        self.col += 1;
    }

    pub(crate) fn backspace(&mut self) {
        if !self.value.is_empty() && self.col > 0 {
            self.col -= 1;
            self.value.remove(self.col);
        }
    }

    pub(crate) fn delete(&mut self) {
        if !self.value.is_empty() && self.col < self.value.len() {
            self.value.remove(self.col);
        }
    }

    pub(crate) fn move_cursor(&mut self, position: Direction) {
        self.col = match position {
            Direction::Left => self.col.saturating_sub(1),
            Direction::Right => (self.col + 1).min(self.value.len()),
        }
    }
}

// endregion: TextInput

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
pub struct TextInput {
    /// Message used to display in the prompt
    pub message: Cow<'static, str>,
    // TextInput state for the prompt
    // pub input: LineInput,
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
//     pub input: LineInput,
//     /// State of the validation of the user input
//     // pub validator_result: Result<(), Cow<'a, str>>,
//     // validator: Option<Box<InputValidator<'a>>>,
// }



// impl AsMut<String> for TextInput {
//     fn as_mut(&mut self) -> &mut String {
//         &mut self.input.value
//     }
// }

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
    pub fn placeholder(&mut self, value: impl Into<Cow<'static, str>>) -> &mut Self {
        self.placeholder = Some(value.into());
        self
    }

    /// Set default value to submit when the input is empty.
    pub fn default(&mut self, value: impl Into<Cow<'static, str>>) -> &mut Self {
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

