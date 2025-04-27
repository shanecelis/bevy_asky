use crate::{construct::*, prelude::*, string_cursor::*};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use std::borrow::Cow;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
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
        )
            .in_set(AskySet::Controller),
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
#[derive(Debug, Clone, Component, Reflect)]
pub struct Number<T: NumLike> {
    /// Default value to submit when the input is empty
    pub default_value: Option<T>,
}

unsafe impl<T: NumLike> Submitter for Number<T> {
    type Out = T;
}

impl<T: NumLike> Construct for Number<T> {
    type Props = Cow<'static, str>;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let input_state = StringCursor::default();
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(Prompt(props))
            .insert(input_state)
            .insert(Focusable::default());
        context.world.flush();
        Ok(Number {
            default_value: None,
        })
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
    mut query: Query<(Entity, &mut StringCursor), With<Number<T>>>,
    mut input: EventReader<KeyboardInput>,
    mut commands: Commands,
    mut focus: FocusParam,
) {
    for (id, mut text_state) in query.iter_mut() {
        if !focus.is_focused(id) {
            continue;
        }
        for ev in input.read() {
            if ev.state != ButtonState::Pressed {
                continue;
            }
            // commands.entity(id).remove::<Feedback>();
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
                            commands.trigger_targets(Submit::new(Ok(number)), id);
                            focus.block(id);
                            // focus.unfocus(id, true);
                            focus.move_focus_from(id);
                        }
                        Err(_) => {
                            // commands
                            //     .trigger_targets(Submit::<T>(Err(Error::InvalidNumber)), id);
                            // focus.block(id);
                            commands.entity(id).try_insert(Feedback::warn(format!(
                                "invalid number for {}",
                                T::short_type_path()
                            )));
                        }
                    }
                }
                Key::Escape => {
                    commands.trigger_targets(Submit::<String>::new(Err(Error::Cancel)), id);
                    commands.entity(id).try_insert(Feedback::error("canceled"));
                    focus.block(id);
                    focus.move_focus_from(id);
                    // focus.unfocus(id, false);
                }
                x => info!("Unhandled key {x:?}"),
            }
        }
    }
}

#[cfg(test)]
mod test {

    use crate::string_cursor::{ceil_char_boundary, floor_char_boundary};

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
