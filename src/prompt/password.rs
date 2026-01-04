use crate::{construct::*, prelude::*, string_cursor::*};
use bevy::prelude::*;
use std::borrow::Cow;

pub(crate) fn plugin(_app: &mut App) {}

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
pub struct Password;

unsafe impl Submitter for Password {
    type Out = String;
}

impl Construct for Password {
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
        Ok(Password)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use bevy::input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    };

    #[test]
    fn test_password_key_presses() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AskyPlugin)
            .add_message::<KeyboardInput>()
            .init_resource::<bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>>()
            .init_resource::<bevy::input_focus::InputFocus>();

        // Create a Password entity with required components
        let entity = app
            .world_mut()
            .spawn((
                Password,
                StringCursor::default(),
                Focusable::default(),
                Prompt(Cow::Borrowed("Password: ")),
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
            // Note: Special characters like '@' and '#' don't have direct KeyCode variants,
            // but they're handled via the logical_key Character field
            let key_code = match c {
                'P' => KeyCode::KeyP,
                'a' => KeyCode::KeyA,
                's' => KeyCode::KeyS,
                'w' => KeyCode::KeyW,
                'o' => KeyCode::KeyO,
                'r' => KeyCode::KeyR,
                'd' => KeyCode::KeyD,
                '1' => KeyCode::Digit1,
                '2' => KeyCode::Digit2,
                '3' => KeyCode::Digit3,
                '@' => KeyCode::Digit2, // '@' is typically Shift+2, use Digit2 as fallback
                '#' => KeyCode::Digit3, // '#' is typically Shift+3, use Digit3 as fallback
                _ => KeyCode::KeyA,     // fallback
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

        // Simulate typing "Password123"
        send_key_event(&mut app, create_char_event('P'));
        send_key_event(&mut app, create_char_event('a'));
        send_key_event(&mut app, create_char_event('s'));
        send_key_event(&mut app, create_char_event('s'));
        send_key_event(&mut app, create_char_event('w'));
        send_key_event(&mut app, create_char_event('o'));
        send_key_event(&mut app, create_char_event('r'));
        send_key_event(&mut app, create_char_event('d'));
        send_key_event(&mut app, create_char_event('1'));
        send_key_event(&mut app, create_char_event('2'));
        send_key_event(&mut app, create_char_event('3'));

        // Run update to process the events
        app.update();

        // Verify the text was inserted
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "Password123");
        assert_eq!(cursor.index, 11);

        // Simulate backspace
        send_key_event(&mut app, create_key_event(Key::Backspace));

        app.update();

        // Verify backspace worked
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "Password12");
        assert_eq!(cursor.index, 10);

        // Simulate moving cursor left
        send_key_event(&mut app, create_key_event(Key::ArrowLeft));
        send_key_event(&mut app, create_key_event(Key::ArrowLeft));

        app.update();

        // Verify cursor moved left (from position 10, moving left twice gets us to position 8)
        // Position 8 is after '1' and before '2' in "Password12"
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "Password12");
        assert_eq!(cursor.index, 8);

        // Simulate delete (should remove character at position 8, which is '1')
        send_key_event(&mut app, create_key_event(Key::Delete));

        app.update();

        // Verify delete worked (removed '1' at position 8, leaving "Password2")
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "Password2");
        assert_eq!(cursor.index, 8);

        // Move cursor to end before inserting special characters
        send_key_event(&mut app, create_key_event(Key::ArrowRight));

        app.update();

        // Verify cursor is at end
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.index, 9);

        // Simulate typing special characters (passwords often have these)
        send_key_event(&mut app, create_char_event('@'));
        send_key_event(&mut app, create_char_event('#'));

        app.update();

        // Verify special characters were inserted
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "Password2@#");
        assert_eq!(cursor.index, 11);

        // Test Enter key (should trigger submit, but we'll just verify it doesn't crash)
        send_key_event(&mut app, create_key_event(Key::Enter));

        app.update();

        // Verify state remains (Enter submits but doesn't change StringCursor)
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "Password2@#");

        // Test Escape key
        send_key_event(&mut app, create_key_event(Key::Escape));

        app.update();

        // Verify state remains (Escape submits cancel but doesn't change StringCursor)
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "Password2@#");
    }
}
