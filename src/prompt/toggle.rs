use crate::{construct::*, prelude::*};
use bevy::prelude::*;
use std::borrow::Cow;

/// Toggles between two named options
#[derive(Component, Clone, Reflect)]
pub struct Toggle {
    /// Prompt
    pub message: Cow<'static, str>,
    /// Options
    pub options: [Cow<'static, str>; 2],
    /// Initial toggle of the prompt
    pub index: usize,
}

impl OptionPrompt for Toggle {
    fn name(&self, index: usize) -> &str {
        match index {
            0 => &self.options[0],
            1 => &self.options[1],
            _ => panic!("No such option for Confirm."),
        }
    }
    fn state(&self) -> usize {
        self.index
    }
}

impl Toggle {
    /// Make a new toggle
    pub fn new<T: Into<Cow<'static, str>>>(
        message: impl Into<Cow<'static, str>>,
        options: [T; 2],
    ) -> Self {
        let mut iter = options.into_iter();
        Toggle {
            message: message.into(),
            options: [iter.next().unwrap().into(), iter.next().unwrap().into()],
            index: 0,
        }
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, toggle_controller.in_set(AskySet::Controller));
}

impl Construct for Toggle {
    type Props = Toggle;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(Prompt(props.message.clone()))
            .insert(Focusable::default());
        context.world.flush();
        Ok(props)
    }
}

fn toggle_controller(
    mut query: Query<(Entity, &mut Toggle)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut focus: FocusParam,
) {
    for (id, mut toggle) in query.iter_mut() {
        if !focus.is_focused(id) {
            continue;
        }
        if input.any_just_pressed([
            KeyCode::KeyH,
            KeyCode::ArrowLeft,
            KeyCode::KeyL,
            KeyCode::ArrowRight,
            KeyCode::Enter,
            KeyCode::Escape,
        ]) {
            if input.any_just_pressed([KeyCode::KeyH, KeyCode::ArrowLeft]) {
                toggle.index = 0;
            }
            if input.any_just_pressed([KeyCode::KeyL, KeyCode::ArrowRight]) {
                toggle.index = 1;
            }
            if input.just_pressed(KeyCode::Enter) {
                commands.trigger_targets(Submit::new(Ok(toggle.index)), id);
                focus.block_and_move(id);
            }

            if input.just_pressed(KeyCode::Escape) {
                commands.trigger_targets(Submit::<bool>::new(Err(Error::Cancel)), id);
                focus.move_focus_from(id);
                // focus.unfocus(id, false);
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

    // Temporary resource to pass entity to focus system
    #[derive(Resource)]
    struct FocusTarget(Entity);

    // Helper system to set focus
    fn set_focus_system(target: Res<FocusTarget>, mut focus: FocusParam) {
        focus.move_focus_to(target.0);
    }

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
    fn test_toggle_key_presses() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AskyPlugin)
            .add_event::<bevy::input::keyboard::KeyboardInput>()
            .init_resource::<ButtonInput<KeyCode>>();

        // Create a Toggle entity with required components
        let entity = app
            .world_mut()
            .spawn((
                Toggle::new("Choose:", ["Option A", "Option B"]),
                Focusable::default(),
                Prompt(Cow::Borrowed("Choose:")),
                GlobalTransform::default(),
            ))
            .id();

        // Set the focus target resource
        app.world_mut().insert_resource(FocusTarget(entity));

        // Set focus on the entity using a one-shot system
        app.world_mut().run_system_once(set_focus_system);

        // Helper to simulate a key press
        fn simulate_key_press(app: &mut App, key: KeyCode) {
            // Set the key to press
            app.world_mut().insert_resource(KeyToPress(key));
            // Press the key using a system
            app.world_mut().run_system_once(press_key_system);
            // Run update to process the key press
            app.update();
            // Clear the key press state
            app.world_mut().run_system_once(clear_keys_system);
            // Run update again to clear the "just_pressed" state
            app.update();
        }

        // Verify initial state
        let toggle = app.world().get::<Toggle>(entity).unwrap();
        assert_eq!(toggle.index, 0);

        // Simulate KeyL press (should set index to 1)
        simulate_key_press(&mut app, KeyCode::KeyL);

        // Verify toggle index is now 1
        let toggle = app.world().get::<Toggle>(entity).unwrap();
        assert_eq!(toggle.index, 1);

        // Simulate KeyH press (should set index to 0)
        simulate_key_press(&mut app, KeyCode::KeyH);

        // Verify toggle index is now 0
        let toggle = app.world().get::<Toggle>(entity).unwrap();
        assert_eq!(toggle.index, 0);

        // Simulate ArrowRight press (should set index to 1)
        simulate_key_press(&mut app, KeyCode::ArrowRight);

        // Verify toggle index is 1
        let toggle = app.world().get::<Toggle>(entity).unwrap();
        assert_eq!(toggle.index, 1);

        // Simulate ArrowLeft press (should set index to 0)
        simulate_key_press(&mut app, KeyCode::ArrowLeft);

        // Verify toggle index is 0
        let toggle = app.world().get::<Toggle>(entity).unwrap();
        assert_eq!(toggle.index, 0);

        // Test Enter key (should trigger submit, but we'll just verify it doesn't crash)
        simulate_key_press(&mut app, KeyCode::KeyL);
        let toggle_before = app.world().get::<Toggle>(entity).unwrap();
        assert_eq!(toggle_before.index, 1);

        simulate_key_press(&mut app, KeyCode::Enter);
        // After Enter, the state should still be 1 (Enter submits but doesn't change state)
        let toggle_after = app.world().get::<Toggle>(entity).unwrap();
        assert_eq!(toggle_after.index, 1);

        // Test Escape key (should trigger cancel submit)
        simulate_key_press(&mut app, KeyCode::Escape);
        // State should remain unchanged
        let toggle_after_escape = app.world().get::<Toggle>(entity).unwrap();
        assert_eq!(toggle_after_escape.index, 1);
    }
}
