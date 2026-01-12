use crate::{prelude::*, string_cursor::*};
use bevy::{
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    prelude::*,
};
use std::borrow::Cow;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<StringCursor>()
        .add_systems(Update, text_controller.in_set(AskySet::Controller));
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
#[derive(Debug, Clone, Component, Reflect)]
pub struct TextField;

unsafe impl Submitter for TextField {
    type Out = String;
}

impl Construct for TextField {
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
        Ok(TextField)
    }
}

fn text_controller(
    mut focus: FocusParam,
    mut query: Query<(Entity, &mut StringCursor), Or<(With<TextField>, With<Password>)>>,
    mut input: MessageReader<KeyboardInput>,
    mut commands: Commands,
) {
    let mut any_focused_text = false;
    // NOTE: Read from the event reader every frame lest you be plagued with
    // bugs from left over input from a prior frame.
    for ev in input.read() {
        if ev.state != ButtonState::Pressed {
            continue;
        }
        for (id, mut text_state) in query.iter_mut() {
            if !focus.is_focused(id) {
                continue;
            }
            any_focused_text |= true;
            trace!("text_controller handling button {ev:?}");
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
                    commands.trigger(Submit::new(id, Ok(text_state.value.clone())));
                }
                Key::Escape => {
                    commands.trigger(Submit::<String>::new(id, Err(Error::Cancel)));
                }
                x => {
                    debug!("Unhandled key {x:?}");
                }
            }
        }
    }
    focus.set_keyboard_nav(!any_focused_text);
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy::input::keyboard::KeyboardInput;

    #[test]
    fn test_text_field_key_presses() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AskyPlugin)
            .add_message::<KeyboardInput>()
            .init_resource::<bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>>()
            .init_resource::<bevy::input_focus::InputFocus>();

        // Create a TextField entity with required components
        let entity = app
            .world_mut()
            .spawn((
                TextField,
                StringCursor::default(),
                Focusable::default(),
                Prompt(Cow::Borrowed("Test: ")),
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
        // KeyboardInput needs both the new fields (key_code, text, repeat) and old fields (logical_key, state, window)
        fn create_char_event(c: char) -> KeyboardInput {
            use bevy::input::keyboard::KeyCode;
            // Map character to KeyCode for the key_code field
            let key_code = match c {
                'H' => KeyCode::KeyH,
                'e' => KeyCode::KeyE,
                'l' => KeyCode::KeyL,
                'o' => KeyCode::KeyO,
                _ => KeyCode::KeyH, // fallback
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
                Key::Space => KeyCode::Space,
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

        // Simulate typing "Hello"
        send_key_event(&mut app, create_char_event('H'));
        send_key_event(&mut app, create_char_event('e'));
        send_key_event(&mut app, create_char_event('l'));
        send_key_event(&mut app, create_char_event('l'));
        send_key_event(&mut app, create_char_event('o'));

        // Run update to process the events
        app.update();

        // Verify the text was inserted
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "Hello");
        assert_eq!(cursor.index, 5);

        // Simulate backspace
        send_key_event(&mut app, create_key_event(Key::Backspace));
        app.update();

        // Verify backspace worked
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "Hell");
        assert_eq!(cursor.index, 4);

        // Simulate moving cursor left
        send_key_event(&mut app, create_key_event(Key::ArrowLeft));
        app.update();

        // Verify cursor moved left
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "Hell");
        assert_eq!(cursor.index, 3);

        // Simulate delete
        send_key_event(&mut app, create_key_event(Key::Delete));
        app.update();

        // Verify delete worked (removed 'l' at position 3)
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "Hel");
        assert_eq!(cursor.index, 3);

        // Simulate typing space
        send_key_event(&mut app, create_key_event(Key::Space));
        app.update();

        // Verify space was inserted
        let cursor = app.world().get::<StringCursor>(entity).unwrap();
        assert_eq!(cursor.value, "Hel ");
        assert_eq!(cursor.index, 4);
    }
}
