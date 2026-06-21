use crate::{Part, construct::*, prelude::*};
use accesskit::{Node as Accessible, Role};
use bevy::prelude::*;
use bevy_a11y::AccessibilityNode;
use std::borrow::Cow;

/// Radio element
///
/// Only one may be selected in a group of elements
#[derive(Component, Reflect)]
pub struct Radio {
    /// Initial radio of the prompt
    pub checked: bool,
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (radio_controller, radio_group_controller).in_set(AskySet::Controller),
    );
}

impl Construct for Radio {
    type Props = Cow<'static, str>;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        //
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(next_tab_index())
            .insert(Prompt(props.clone()))
            .insert(AccessibilityNode(Accessible::new(Role::RadioButton)));
        // commands.trigger(AddView(context.id));
        context.world.flush();
        Ok(Radio { checked: false })
    }
}

fn radio_controller(
    input_focus: Res<InputFocus>,
    mut query: Query<(Entity, &mut Radio, Option<&ChildOf>)>,
    child_query: Query<&Children>,
    input: Res<ButtonInput<KeyCode>>,
    mut toggled: Local<Vec<(Entity, Entity)>>,
) {
    if !input.any_just_pressed([KeyCode::Space, KeyCode::KeyH, KeyCode::KeyL]) {
        return;
    }
    toggled.clear();
    for (id, mut radio, parent) in query.iter_mut() {
        if input_focus.get() != Some(id) {
            continue;
        }
        let was_checked = radio.checked;

        if input.just_pressed(KeyCode::Space) {
            radio.checked = !radio.checked;
        }
        if input.any_just_pressed([KeyCode::KeyL]) {
            radio.checked = true;
        }
        if input.any_just_pressed([KeyCode::KeyH]) {
            radio.checked = false;
        }
        if radio.checked && !was_checked {
            // We've been checked and weren't checked before.
            if let Some(p) = parent {
                toggled.push((id, p.parent()));
            }
        }
    }
    for (toggled_child, parent) in toggled.drain(..) {
        for child in child_query.get(parent).unwrap() {
            if *child == toggled_child {
                continue;
            }
            if let Ok((_, mut radio, _)) = query.get_mut(*child) {
                radio.checked = false;
            }
        }
    }
}

/// Parent of entities with [Radio] component
#[derive(Component, Reflect, Default)]
pub struct RadioGroup;

unsafe impl Submitter for RadioGroup {
    type Out = usize;
}

impl Part for Radio {
    type Group = RadioGroup;
}

impl Construct for RadioGroup {
    type Props = Cow<'static, str>;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .column()
            .with_children(|parent| {
                parent.spawn(Text::new(props));
            });

        context.world.flush();
        Ok(RadioGroup)
    }
}

fn radio_group_controller(
    mut query: Query<(Entity, &Children), With<RadioGroup>>,
    radios: Query<(Entity, &Radio)>,
    input_focus: Res<InputFocus>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if !input.any_just_pressed([
        KeyCode::Escape,
        KeyCode::Enter,
        KeyCode::ArrowDown,
        KeyCode::ArrowUp,
    ]) {
        return;
    }
    for (id, children) in query.iter_mut() {
        if let Some(_index) = radios
            .iter_many(children)
            .position(|(id, _)| input_focus.get() == Some(id))
        {
            if input.just_pressed(KeyCode::Enter) {
                if let Some(selection) = radios
                    .iter_many(children)
                    .position(|(_, radio)| radio.checked)
                {
                    // commands.trigger(Submit::new(id, selection.ok_or(Error::InvalidInput)));
                    commands.trigger(Submit::new(id, Ok(selection)));
                } else {
                    commands
                        .entity(id)
                        .try_insert(Feedback::warn("must select one"));
                }
            }

            if input.just_pressed(KeyCode::Escape) {
                commands.trigger(Submit::<usize>::new(id, Err(Error::Cancel)));
                commands.entity(id).try_insert(Feedback::error("canceled"));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::input::keyboard::KeyCode;

    // Temporary resource to pass key code to press system
    #[derive(Resource)]
    struct KeyToPress(KeyCode);

    // Helper system to press a key
    fn press_key_system(key: Res<KeyToPress>, mut input: ResMut<ButtonInput<KeyCode>>) {
        input.press(key.0);
    }

    // Helper system to clear all key presses
    fn clear_keys_system(mut input: ResMut<ButtonInput<KeyCode>>) {
        // ButtonInput doesn't have a clear_all, so we need to release all pressed keys
        // For testing, we'll just reset the resource
        *input = ButtonInput::default();
    }

    #[test]
    fn test_radio_key_presses() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AskyPlugin)
            .add_message::<bevy::input::keyboard::KeyboardInput>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<bevy::input_focus::InputFocus>();

        // Create a Radio entity with required components
        let entity = app
            .world_mut()
            .spawn((
                Radio { checked: false },
                next_tab_index(),
                Prompt(Cow::Borrowed("Option 1")),
                AccessibilityNode(accesskit::Node::new(accesskit::Role::RadioButton)),
            ))
            .id();

        // Run update to let reset_focus system automatically set focus
        app.update();

        // Helper to simulate a key press
        fn simulate_key_press(app: &mut App, key: KeyCode) {
            // Set the key to press
            app.world_mut().insert_resource(KeyToPress(key));
            // Press the key using a system
            app.world_mut().run_system_once(press_key_system).unwrap();
            // Run update to process the key press
            app.update();
            // Clear the key press state
            app.world_mut().run_system_once(clear_keys_system).unwrap();
            // Run update again to clear the "just_pressed" state
            app.update();
        }

        // Verify initial state
        let radio = app.world().get::<Radio>(entity).unwrap();
        assert!(!radio.checked);

        // Simulate Space key press (should toggle to true)
        simulate_key_press(&mut app, KeyCode::Space);

        // Verify radio is now checked
        let radio = app.world().get::<Radio>(entity).unwrap();
        assert!(radio.checked);

        // Simulate Space key press again (should toggle back to false)
        simulate_key_press(&mut app, KeyCode::Space);

        // Verify radio is unchecked again
        let radio = app.world().get::<Radio>(entity).unwrap();
        assert!(!radio.checked);

        // Simulate KeyL press (should set to true)
        simulate_key_press(&mut app, KeyCode::KeyL);

        // Verify radio is checked
        let radio = app.world().get::<Radio>(entity).unwrap();
        assert!(radio.checked);

        // Simulate KeyH press (should set to false)
        simulate_key_press(&mut app, KeyCode::KeyH);

        // Verify radio is unchecked
        let radio = app.world().get::<Radio>(entity).unwrap();
        assert!(!radio.checked);

        // Test toggle again with Space
        simulate_key_press(&mut app, KeyCode::Space);
        let radio = app.world().get::<Radio>(entity).unwrap();
        assert!(radio.checked);

        simulate_key_press(&mut app, KeyCode::Space);
        let radio = app.world().get::<Radio>(entity).unwrap();
        assert!(!radio.checked);
    }
}
