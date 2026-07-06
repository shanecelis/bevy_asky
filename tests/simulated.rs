use bevy::{
    input::{
        ButtonState,
        keyboard::{Key, KeyCode, KeyboardInput},
    },
    input_focus::{FocusCause, InputFocus},
    prelude::*,
};
use bevy_asky::{prelude::*, string_cursor::StringCursor};
use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

type SubmitLog<T> = Arc<Mutex<Vec<Result<T, Error>>>>;

fn headless_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(AskyPlugin)
        .add_message::<KeyboardInput>()
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<InputFocus>();
    app
}

fn focus(app: &mut App, entity: Entity) {
    app.world_mut()
        .resource_mut::<InputFocus>()
        .set(entity, FocusCause::Navigated);
}

fn observe_submit<T: Clone + Send + Sync + 'static>(app: &mut App, entity: Entity) -> SubmitLog<T> {
    let log = SubmitLog::<T>::default();
    let observer_log = log.clone();
    app.world_mut()
        .commands()
        .entity(entity)
        .observe(move |mut trigger: On<Submit<T>>| {
            observer_log
                .lock()
                .unwrap()
                .push(trigger.event_mut().take_result());
        });
    app.world_mut().flush();
    log
}

fn submitted<T>(log: &SubmitLog<T>) -> Result<T, Error> {
    let mut entries = log.lock().unwrap();
    assert_eq!(entries.len(), 1);
    entries.pop().unwrap()
}

fn char_key_code(ch: char) -> KeyCode {
    match ch {
        '0' => KeyCode::Digit0,
        '1' => KeyCode::Digit1,
        '2' => KeyCode::Digit2,
        '3' => KeyCode::Digit3,
        '4' => KeyCode::Digit4,
        '5' => KeyCode::Digit5,
        '6' => KeyCode::Digit6,
        '7' => KeyCode::Digit7,
        '8' => KeyCode::Digit8,
        '9' => KeyCode::Digit9,
        '-' => KeyCode::Minus,
        '+' => KeyCode::Equal,
        '.' => KeyCode::Period,
        ' ' => KeyCode::Space,
        _ => KeyCode::KeyA,
    }
}

fn keyboard_char(ch: char) -> KeyboardInput {
    KeyboardInput {
        logical_key: if ch == ' ' {
            Key::Space
        } else {
            Key::Character(ch.to_string().into())
        },
        state: ButtonState::Pressed,
        window: Entity::PLACEHOLDER,
        key_code: char_key_code(ch),
        text: Some(ch.to_string().into()),
        repeat: false,
    }
}

fn keyboard_key(logical_key: Key, key_code: KeyCode) -> KeyboardInput {
    KeyboardInput {
        logical_key,
        state: ButtonState::Pressed,
        window: Entity::PLACEHOLDER,
        key_code,
        text: None,
        repeat: false,
    }
}

fn send_keyboard(app: &mut App, input: KeyboardInput) {
    app.world_mut()
        .resource_mut::<Messages<KeyboardInput>>()
        .write(input);
    app.update();
}

fn press_button(app: &mut App, key: KeyCode) {
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(key);
    app.update();
    *app.world_mut().resource_mut::<ButtonInput<KeyCode>>() = ButtonInput::default();
    app.update();
}

#[test]
fn text_field_handles_editing_and_submits_text() {
    let mut app = headless_app();
    let entity = app
        .world_mut()
        .spawn((
            TextField,
            StringCursor::default(),
            next_tab_index(),
            Prompt(Cow::Borrowed("Name:")),
        ))
        .id();
    let log = observe_submit::<String>(&mut app, entity);
    focus(&mut app, entity);

    for ch in "hey".chars() {
        send_keyboard(&mut app, keyboard_char(ch));
    }
    send_keyboard(&mut app, keyboard_key(Key::ArrowLeft, KeyCode::ArrowLeft));
    send_keyboard(&mut app, keyboard_char('!'));
    send_keyboard(&mut app, keyboard_key(Key::Delete, KeyCode::Delete));
    send_keyboard(&mut app, keyboard_key(Key::Enter, KeyCode::Enter));

    assert_eq!(submitted(&log).unwrap(), "he!");
}

#[test]
fn password_handles_editing_and_submits_unmasked_text() {
    let mut app = headless_app();
    let entity = app
        .world_mut()
        .spawn((
            Password,
            StringCursor::default(),
            next_tab_index(),
            Prompt(Cow::Borrowed("Password:")),
        ))
        .id();
    let log = observe_submit::<String>(&mut app, entity);
    focus(&mut app, entity);

    for ch in "s3 cret!".chars() {
        send_keyboard(&mut app, keyboard_char(ch));
    }
    send_keyboard(&mut app, keyboard_key(Key::Backspace, KeyCode::Backspace));
    send_keyboard(&mut app, keyboard_key(Key::Enter, KeyCode::Enter));

    assert_eq!(submitted(&log).unwrap(), "s3 cret");
}

#[test]
fn number_rejects_invalid_chars_and_submits_parsed_value() {
    let mut app = headless_app();
    let entity = app
        .world_mut()
        .spawn((
            Number::<i32> {
                default_value: None,
            },
            StringCursor::default(),
            next_tab_index(),
            Prompt(Cow::Borrowed("Count:")),
        ))
        .id();
    let log = observe_submit::<i32>(&mut app, entity);
    focus(&mut app, entity);

    for ch in "12a".chars() {
        send_keyboard(&mut app, keyboard_char(ch));
    }
    assert_eq!(app.world().get::<StringCursor>(entity).unwrap().value, "12");

    send_keyboard(&mut app, keyboard_key(Key::ArrowLeft, KeyCode::ArrowLeft));
    send_keyboard(&mut app, keyboard_key(Key::ArrowLeft, KeyCode::ArrowLeft));
    send_keyboard(&mut app, keyboard_char('-'));
    send_keyboard(&mut app, keyboard_key(Key::Enter, KeyCode::Enter));

    assert_eq!(submitted(&log).unwrap(), -12);
}

#[test]
fn number_submits_default_value_when_empty() {
    let mut app = headless_app();
    let entity = app
        .world_mut()
        .spawn((
            Number::<i32> {
                default_value: Some(7),
            },
            StringCursor::default(),
            next_tab_index(),
            Prompt(Cow::Borrowed("Count:")),
        ))
        .id();
    let log = observe_submit::<i32>(&mut app, entity);
    focus(&mut app, entity);

    send_keyboard(&mut app, keyboard_key(Key::Enter, KeyCode::Enter));

    assert_eq!(submitted(&log).unwrap(), 7);
}

#[test]
fn confirm_selects_and_submits_boolean_choice() {
    let mut app = headless_app();
    let entity = app
        .world_mut()
        .spawn((
            Confirm { yes: false },
            next_tab_index(),
            Prompt(Cow::Borrowed("Continue?")),
        ))
        .id();
    let log = observe_submit::<bool>(&mut app, entity);
    focus(&mut app, entity);

    press_button(&mut app, KeyCode::KeyY);
    assert!(app.world().get::<Confirm>(entity).unwrap().yes);
    press_button(&mut app, KeyCode::Enter);

    assert!(submitted(&log).unwrap());
}

#[test]
fn toggle_selects_and_submits_index() {
    let mut app = headless_app();
    let entity = app
        .world_mut()
        .spawn((
            Toggle::new("Mode:", ["Easy", "Hard"]),
            next_tab_index(),
            Prompt(Cow::Borrowed("Mode:")),
        ))
        .id();
    let log = observe_submit::<usize>(&mut app, entity);
    focus(&mut app, entity);

    press_button(&mut app, KeyCode::KeyL);
    assert_eq!(app.world().get::<Toggle>(entity).unwrap().index, 1);
    press_button(&mut app, KeyCode::Enter);

    assert_eq!(submitted(&log).unwrap(), 1);
}

#[test]
fn toggle_cancel_submits_typed_cancel() {
    let mut app = headless_app();
    let entity = app
        .world_mut()
        .spawn((
            Toggle::new("Mode:", ["Easy", "Hard"]),
            next_tab_index(),
            Prompt(Cow::Borrowed("Mode:")),
        ))
        .id();
    let log = observe_submit::<usize>(&mut app, entity);
    focus(&mut app, entity);

    press_button(&mut app, KeyCode::Escape);

    assert!(matches!(submitted(&log), Err(Error::Cancel)));
}

#[test]
fn checkbox_group_toggles_children_and_submits_all_states() {
    let mut app = headless_app();
    let group = app.world_mut().spawn(CheckboxGroup).id();
    let mut boxes = Vec::new();
    app.world_mut().entity_mut(group).with_children(|parent| {
        for label in ["A", "B", "C"] {
            boxes.push(
                parent
                    .spawn((
                        Checkbox { checked: false },
                        next_tab_index(),
                        Prompt(Cow::Borrowed(label)),
                    ))
                    .id(),
            );
        }
    });
    let log = observe_submit::<Vec<bool>>(&mut app, group);

    focus(&mut app, boxes[0]);
    press_button(&mut app, KeyCode::Space);
    focus(&mut app, boxes[1]);
    press_button(&mut app, KeyCode::KeyY);
    focus(&mut app, boxes[2]);
    press_button(&mut app, KeyCode::KeyN);
    press_button(&mut app, KeyCode::Enter);

    assert_eq!(submitted(&log).unwrap(), vec![true, true, false]);
}

#[test]
fn radio_group_keeps_single_selection_and_submits_index() {
    let mut app = headless_app();
    let group = app.world_mut().spawn(RadioGroup).id();
    let mut radios = Vec::new();
    app.world_mut().entity_mut(group).with_children(|parent| {
        for label in ["Red", "Green", "Blue"] {
            radios.push(
                parent
                    .spawn((
                        Radio { checked: false },
                        next_tab_index(),
                        Prompt(Cow::Borrowed(label)),
                    ))
                    .id(),
            );
        }
    });
    let log = observe_submit::<usize>(&mut app, group);

    focus(&mut app, radios[0]);
    press_button(&mut app, KeyCode::Space);
    focus(&mut app, radios[1]);
    press_button(&mut app, KeyCode::Space);
    assert!(!app.world().get::<Radio>(radios[0]).unwrap().checked);
    assert!(app.world().get::<Radio>(radios[1]).unwrap().checked);

    press_button(&mut app, KeyCode::Enter);

    assert_eq!(submitted(&log).unwrap(), 1);
}
