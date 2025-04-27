use crate::{prelude::*, string_cursor::*};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
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
    mut input: EventReader<KeyboardInput>,
    mut commands: Commands,
) {
    let mut any_focused_text = false;
    for (id, mut text_state) in query.iter_mut() {
        if !focus.is_focused(id) {
            continue;
        }
        any_focused_text |= true;
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
                    commands.trigger_targets(Submit::new(Ok(text_state.value.clone())), id);
                    // focus.block_and_move(id);
                }
                Key::Escape => {
                    commands.trigger_targets(Submit::<String>::new(Err(Error::Cancel)), id);
                    // commands.entity(id).try_insert(Feedback::error("canceled"));
                    // focus.block(id);
                }
                _x => {
                    // info!("Unhandled key {x:?}");
                }
            }
        }
    }
    focus.set_keyboard_nav(!any_focused_text);
}
