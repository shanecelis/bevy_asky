use super::{Feedback, Prompt};
use crate::construct::*;
use crate::{AskyEvent, AskyState, Error, NumLike};
use crate::{CursorDirection, StringCursor};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use bevy_alt_ui_navigation_lite::{events::Direction as NavDirection, prelude::*};
use std::borrow::Cow;

pub fn plugin(app: &mut App) {
    app.add_systems(
        PreUpdate,
        (
            number_controller::<f32>,
            number_controller::<f64>,
            number_controller::<i8>,
            number_controller::<i16>,
            number_controller::<i32>,
            number_controller::<i64>,
            number_controller::<isize>,
            number_controller::<u8>,
            number_controller::<u16>,
            number_controller::<u32>,
            number_controller::<u64>,
            number_controller::<usize>,
        ),
    );
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
/// let name = Number<T>::new("What is your name?").prompt()?;
///
/// # #[cfg(feature = "terminal")]
/// println!("Hello, {}!", name);
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Component)]
pub struct Number<T: NumLike> {
    /// Default value to submit when the input is empty
    pub default_value: Option<T>,
}

// impl<T: NumLike> From<Cow<'static, str>> for Number<T> {
//     fn from(message: Cow<'static, str>) -> Self {
//         Self {
//             message,
//             default_value: None,
//         }
//     }
// }

// impl<T: NumLike> From<&'static str> for Number<T> {
//     fn from(message: &'static str) -> Self {
//         Self {
//             message: message.into(),
//             default_value: None,
//         }
//     }
// }

impl<T: NumLike> Construct for Number<T> {
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
        Ok(Number { default_value: None })
    }
}

impl<T: NumLike> Number<T> {

    /// Set default value to submit when the input is empty.
    pub fn default(mut self, value: T) -> Self {
        self.default_value = Some(value);
        self
    }
}

fn number_controller<T: NumLike + Sync + 'static + TypePath>(
    mut query: Query<(Entity, &mut AskyState, &mut StringCursor, &mut Focusable), With<Number<T>>>,
    mut input: EventReader<KeyboardInput>,
    mut commands: Commands,
    mut requests: EventWriter<NavRequest>,
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
                commands.entity(id).remove::<Feedback>();
                match &ev.logical_key {
                    Key::Character(s) => {
                        for c in s.chars() {
                            if T::is_valid(c, &text_state) {
                                text_state.insert(c);
                            }
                        }
                    }
                    Key::Space => text_state.insert(' '),
                    Key::Backspace => text_state.backspace(),
                    Key::Delete => text_state.delete(),
                    Key::ArrowLeft => text_state.move_cursor(CursorDirection::Left),
                    Key::ArrowRight => text_state.move_cursor(CursorDirection::Right),
                    Key::Enter => {
                        match T::from_str(&text_state.value) {
                            Ok(number) => {
                                commands.trigger_targets(AskyEvent(Ok(number)), id);
                                *state = AskyState::Complete;
                                // focusable.block();
                                // requests.send(NavRequest::ScopeMove(ScopeDirection::Next));
                                requests.send(NavRequest::Move(NavDirection::South));
                            }
                            Err(_) => {
                                commands
                                    .trigger_targets(AskyEvent::<T>(Err(Error::InvalidNumber)), id);
                                commands.entity(id).insert(Feedback::warn(format!(
                                    "invalid number for {}",
                                    T::short_type_path()
                                )));
                            }
                        }
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

#[cfg(test)]
mod test {

    use crate::{ceil_char_boundary, floor_char_boundary};

    #[test]
    fn test_floor_char() {
        let s = "❤️🧡💛💚💙💜";
        assert_eq!(s.len(), 26);
        assert!(!s.is_char_boundary(13));

        let closest = floor_char_boundary(s, 13);
        assert_eq!(closest, 10);
        assert_eq!(&s[..closest], "❤️🧡");
        assert_eq!(floor_char_boundary(s, 0), 0);
        assert_eq!(floor_char_boundary(s, 26), 26);
        assert_eq!(floor_char_boundary(s, 27), 26);
    }

    #[test]
    fn test_ceil_char() {
        let s = "❤️🧡💛💚💙💜";
        assert_eq!(s.len(), 26);
        assert!(!s.is_char_boundary(13));

        let closest = ceil_char_boundary(s, 13);
        assert_eq!(closest, 14);
        assert_eq!(&s[..closest], "❤️🧡💛");
        assert_eq!(ceil_char_boundary(s, 0), 0);
        assert_eq!(ceil_char_boundary(s, 26), 26);
        assert_eq!(ceil_char_boundary(s, 27), 26);
    }
}
