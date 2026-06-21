use crate::{construct::*, prelude::*, string_cursor::*};
use bevy::{
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
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
            .insert(next_tab_index());
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
    mut input: MessageReader<KeyboardInput>,
    mut commands: Commands,
    nav: TabNavigation,
    mut input_focus: ResMut<InputFocus>,
) {
    for ev in input.read() {
        if ev.state != ButtonState::Pressed {
            continue;
        }
        for (id, mut text_state) in query.iter_mut() {
            if input_focus.get() != Some(id) {
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
                            commands.trigger(Submit::new(id, Ok(number)));
                            crate::block_and_move_focus(&mut commands, id, &nav, &mut input_focus);
                        }
                        Err(_) => {
                            // commands
                            //     .trigger(Submit::new(id, Err(Error::InvalidNumber)));
                            // focus.block(id);
                            commands.entity(id).try_insert(Feedback::warn(format!(
                                "invalid number for {}",
                                T::short_type_path()
                            )));
                        }
                    }
                }
                Key::Escape => {
                    commands.trigger(Submit::<String>::new(id, Err(Error::Cancel)));
                    commands.entity(id).try_insert(Feedback::error("canceled"));
                    crate::block_and_move_focus(&mut commands, id, &nav, &mut input_focus);
                    // focus.unfocus(id, false);
                }
                x => info!("Unhandled key {x:?}"),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy::input::keyboard::KeyboardInput;

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

    #[test]
    fn test_number_key_presses() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AskyPlugin)
            .add_message::<KeyboardInput>()
            .init_resource::<bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>>()
            .init_resource::<bevy::input_focus::InputFocus>();

        // Create a Number<i32> entity with required components
        let entity = app
            .world_mut()
            .spawn((
                Number::<i32> {
                    default_value: None,
                },
                StringCursor::default(),
                next_tab_index(),
                Prompt(Cow::Borrowed("Enter a number: ")),
            ))
            .id();

        // Run update to let reset_focus system automatically set focus
        app.update();

        // Helper to send keyboard events
        fn send_key_event(app: &mut App, event: KeyboardInput) {
            app.world_mut()
                .resource_mut::<Messages<KeyboardInput>>()
                .write(event);
        }

        // Helper to create character events
        fn create_char_event(c: char) -> KeyboardInput {
            use bevy::input::keyboard::KeyCode;
            // Map character to KeyCode for the key_code field
            let key_code = match c {
                '1' => KeyCode::Digit1,
                '2' => KeyCode::Digit2,
                '3' => KeyCode::Digit3,
                '4' => KeyCode::Digit4,
                '5' => KeyCode::Digit5,
                '6' => KeyCode::Digit6,
                '7' => KeyCode::Digit7,
                '8' => KeyCode::Digit8,
                '9' => KeyCode::Digit9,
                '0' => KeyCode::Digit0,
                '-' => KeyCode::Minus,
                'a' => KeyCode::KeyA,
                _ => KeyCode::Digit1, // fallback
            };
            KeyboardInput {
                logical_key: Key::Character(c.to_string().into()),
                state: ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
                key_code,
                text: Some(c.to_string().into()),
                repeat: false,
            }
        }

        fn create_key_event(key: Key) -> KeyboardInput {
            use bevy::input::keyboard::KeyCode;
            let key_code = match key {
                Key::Backspace => KeyCode::Backspace,
                Key::Delete => KeyCode::Delete,
                Key::ArrowLeft => KeyCode::ArrowLeft,
                Key::ArrowRight => KeyCode::ArrowRight,
                Key::Enter => KeyCode::Enter,
                Key::Escape => KeyCode::Escape,
                _ => KeyCode::Backspace, // fallback
            };
            KeyboardInput {
                logical_key: key,
                state: ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
                key_code,
                text: None,
                repeat: false,
            }
        }

        // Simulate typing "123"
        send_key_event(&mut app, create_char_event('1'));
        send_key_event(&mut app, create_char_event('2'));
        send_key_event(&mut app, create_char_event('3'));

        // Run update to process the events
        app.update();

        // Verify the text was inserted
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "123");
        assert_eq!(cursor.index, 3);

        // Test that invalid characters are rejected (e.g., 'a' for i32)
        send_key_event(&mut app, create_char_event('a'));

        app.update();

        // Verify 'a' was not inserted (i32 only accepts digits and +/-)
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "123");
        assert_eq!(cursor.index, 3);

        // Test negative number
        // Move cursor to beginning
        send_key_event(&mut app, create_key_event(Key::ArrowLeft));
        send_key_event(&mut app, create_key_event(Key::ArrowLeft));
        send_key_event(&mut app, create_key_event(Key::ArrowLeft));

        app.update();

        // Verify cursor moved to beginning
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.index, 0);

        // Insert minus sign
        send_key_event(&mut app, create_char_event('-'));

        app.update();

        // Verify minus was inserted
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "-123");
        assert_eq!(cursor.index, 1);

        // Test backspace
        send_key_event(&mut app, create_key_event(Key::Backspace));

        app.update();

        // Verify backspace worked
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "123");
        assert_eq!(cursor.index, 0);

        // Test cursor movement
        send_key_event(&mut app, create_key_event(Key::ArrowRight));

        app.update();

        // Verify cursor moved right
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.index, 1);

        // Test delete
        send_key_event(&mut app, create_key_event(Key::Delete));

        app.update();

        // Verify delete worked (removed '2' at position 1)
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "13");
        assert_eq!(cursor.index, 1);

        // Test Enter key (should trigger submit, but we'll just verify it doesn't crash)
        // The number "13" is valid, so it should submit successfully
        send_key_event(&mut app, create_key_event(Key::Enter));

        app.update();

        // Verify state remains (Enter submits but doesn't change StringCursor)
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "13");
    }

    #[test]
    fn test_number_f32_key_presses() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AskyPlugin)
            .add_message::<KeyboardInput>()
            .init_resource::<bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>>()
            .init_resource::<bevy::input_focus::InputFocus>();

        // Create a Number<f32> entity with required components
        let entity = app
            .world_mut()
            .spawn((
                Number::<f32> {
                    default_value: None,
                },
                StringCursor::default(),
                next_tab_index(),
                Prompt(Cow::Borrowed("Enter a number: ")),
            ))
            .id();

        // Run update to let reset_focus system automatically set focus
        app.update();

        // Helper to send keyboard events
        fn send_key_event(app: &mut App, event: KeyboardInput) {
            app.world_mut()
                .resource_mut::<Messages<KeyboardInput>>()
                .write(event);
        }

        // Helper to create character events
        fn create_char_event(c: char) -> KeyboardInput {
            use bevy::input::keyboard::KeyCode;
            // Map character to KeyCode for the key_code field
            let key_code = match c {
                '1' => KeyCode::Digit1,
                '2' => KeyCode::Digit2,
                '3' => KeyCode::Digit3,
                '4' => KeyCode::Digit4,
                '5' => KeyCode::Digit5,
                '6' => KeyCode::Digit6,
                '7' => KeyCode::Digit7,
                '8' => KeyCode::Digit8,
                '9' => KeyCode::Digit9,
                '0' => KeyCode::Digit0,
                '-' => KeyCode::Minus,
                '.' => KeyCode::Period,
                'e' => KeyCode::KeyE,
                'E' => KeyCode::KeyE,
                'a' => KeyCode::KeyA,
                _ => KeyCode::Digit1, // fallback
            };
            KeyboardInput {
                logical_key: Key::Character(c.to_string().into()),
                state: ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
                key_code,
                text: Some(c.to_string().into()),
                repeat: false,
            }
        }

        fn create_key_event(key: Key) -> KeyboardInput {
            use bevy::input::keyboard::KeyCode;
            let key_code = match key {
                Key::Backspace => KeyCode::Backspace,
                Key::Delete => KeyCode::Delete,
                Key::ArrowLeft => KeyCode::ArrowLeft,
                Key::ArrowRight => KeyCode::ArrowRight,
                Key::Enter => KeyCode::Enter,
                Key::Escape => KeyCode::Escape,
                _ => KeyCode::Backspace, // fallback
            };
            KeyboardInput {
                logical_key: key,
                state: ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
                key_code,
                text: None,
                repeat: false,
            }
        }

        // Simulate typing "123.45"
        send_key_event(&mut app, create_char_event('1'));
        send_key_event(&mut app, create_char_event('2'));
        send_key_event(&mut app, create_char_event('3'));
        send_key_event(&mut app, create_char_event('.'));
        send_key_event(&mut app, create_char_event('4'));
        send_key_event(&mut app, create_char_event('5'));

        // Run update to process the events
        app.update();

        // Verify the text was inserted
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "123.45");
        assert_eq!(cursor.index, 6);

        // Test that invalid characters are rejected (e.g., 'a' for f32)
        send_key_event(&mut app, create_char_event('a'));

        app.update();

        // Verify 'a' was not inserted (f32 only accepts digits, decimal point, +/- and e/E)
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "123.45");
        assert_eq!(cursor.index, 6);

        // Test negative number
        // Move cursor to beginning
        for _ in 0..6 {
            send_key_event(&mut app, create_key_event(Key::ArrowLeft));
        }

        app.update();

        // Verify cursor moved to beginning
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.index, 0);

        // Insert minus sign
        send_key_event(&mut app, create_char_event('-'));

        app.update();

        // Verify minus was inserted
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "-123.45");
        assert_eq!(cursor.index, 1);

        // Test backspace
        send_key_event(&mut app, create_key_event(Key::Backspace));

        app.update();

        // Verify backspace worked
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "123.45");
        assert_eq!(cursor.index, 0);

        // Test cursor movement to middle
        // Move to position after decimal point (index 4: "123.|45")
        send_key_event(&mut app, create_key_event(Key::ArrowRight));
        send_key_event(&mut app, create_key_event(Key::ArrowRight));
        send_key_event(&mut app, create_key_event(Key::ArrowRight));
        send_key_event(&mut app, create_key_event(Key::ArrowRight));

        app.update();

        // Verify cursor moved to after the decimal point
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.index, 4);

        // Test delete (should remove '4' at position 4)
        send_key_event(&mut app, create_key_event(Key::Delete));

        app.update();

        // Verify delete worked
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "123.5");
        assert_eq!(cursor.index, 4);

        // Test that 'e' is rejected (scientific notation not supported in validation)
        // Move cursor to end
        send_key_event(&mut app, create_key_event(Key::ArrowRight));

        app.update();

        // Verify cursor is at end
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.index, 5);

        // Try to insert 'e' for scientific notation
        send_key_event(&mut app, create_char_event('e'));

        app.update();

        // Verify 'e' was NOT inserted (validation doesn't allow 'e' even though f32 can parse it)
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "123.5");
        assert_eq!(cursor.index, 5);

        // Test that only one decimal point is allowed
        // Try to insert another decimal point
        send_key_event(&mut app, create_char_event('.'));

        app.update();

        // Verify second decimal point was NOT inserted
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "123.5");
        assert_eq!(cursor.index, 5);

        // Test Enter key (should trigger submit, but we'll just verify it doesn't crash)
        // The number "123.5" is valid, so it should submit successfully
        send_key_event(&mut app, create_key_event(Key::Enter));

        app.update();

        // Verify state remains (Enter submits but doesn't change StringCursor)
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "123.5");
    }
}
