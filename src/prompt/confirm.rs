use crate::{construct::*, prelude::*};
use bevy::prelude::*;
use std::borrow::Cow;

/// Confirm query
#[derive(Debug, Component, Reflect)]
pub struct Confirm {
    /// Yes or no
    pub yes: bool,
}

impl OptionPrompt for Confirm {
    fn name(&self, index: usize) -> &str {
        match index {
            0 => "No",
            1 => "Yes",
            _ => panic!("No such option for Confirm."),
        }
    }
    fn state(&self) -> usize {
        if self.yes {
            1
        } else {
            0
        }
    }
}

unsafe impl Submitter for Confirm {
    type Out = bool;
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, confirm_controller.in_set(AskySet::Controller));
}

impl Construct for Confirm {
    type Props = Cow<'static, str>;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(Focusable::default())
            .insert(Prompt(props.clone()));
        context.world.flush();
        Ok(Confirm { yes: false })
    }
}

fn confirm_controller(
    mut query: Query<(Entity, &mut Confirm)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    focus: FocusParam,
) {
    for (id, mut confirm) in query.iter_mut() {
        if !focus.is_focused(id) {
            continue;
        }
        if input.any_just_pressed([
            KeyCode::KeyY,
            KeyCode::ArrowRight,
            KeyCode::ArrowLeft,
            KeyCode::KeyH,
            KeyCode::KeyL,
            KeyCode::KeyN,
            KeyCode::Enter,
            KeyCode::Escape,
        ]) {
            if input.any_just_pressed([KeyCode::KeyY, KeyCode::KeyL, KeyCode::ArrowRight]) {
                confirm.yes = true;
            }
            if input.any_just_pressed([KeyCode::KeyN, KeyCode::KeyH, KeyCode::ArrowLeft]) {
                confirm.yes = false;
            }
            if input.just_pressed(KeyCode::Enter) {
                // Make this not focusable again.
                // I had tried using triggers in bevy_ui_navigation to fix my issues.
                // commands.trigger(NavRequest::Move(NavDirection::South));
                commands.trigger(Submit::<bool>::new(id, Ok(confirm.yes)));
                // focus.block_and_move(id);
            }
            if input.just_pressed(KeyCode::Escape) {
                commands.trigger(Submit::<bool>::new(id, Err(Error::Cancel)));
                // commands.entity(id).try_insert(Feedback::error("canceled"));
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
    fn test_confirm_key_presses() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AskyPlugin)
            .add_event::<bevy::input::keyboard::KeyboardInput>()
            .init_resource::<ButtonInput<KeyCode>>();

        // Create a Confirm entity with required components
        let entity = app
            .world_mut()
            .spawn((
                Confirm { yes: false },
                Focusable::default(),
                Prompt(Cow::Borrowed("Do you confirm?")),
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
        let confirm = app.world().get::<Confirm>(entity).unwrap();
        assert_eq!(confirm.yes, false);

        // Simulate KeyY press (should set yes to true)
        simulate_key_press(&mut app, KeyCode::KeyY);

        // Verify confirm is now yes
        let confirm = app.world().get::<Confirm>(entity).unwrap();
        assert_eq!(confirm.yes, true);

        // Simulate KeyN press (should set yes to false)
        simulate_key_press(&mut app, KeyCode::KeyN);

        // Verify confirm is now no
        let confirm = app.world().get::<Confirm>(entity).unwrap();
        assert_eq!(confirm.yes, false);

        // Simulate KeyL press (should set yes to true)
        simulate_key_press(&mut app, KeyCode::KeyL);

        // Verify confirm is yes
        let confirm = app.world().get::<Confirm>(entity).unwrap();
        assert_eq!(confirm.yes, true);

        // Simulate KeyH press (should set yes to false)
        simulate_key_press(&mut app, KeyCode::KeyH);

        // Verify confirm is no
        let confirm = app.world().get::<Confirm>(entity).unwrap();
        assert_eq!(confirm.yes, false);

        // Simulate ArrowRight press (should set yes to true)
        simulate_key_press(&mut app, KeyCode::ArrowRight);

        // Verify confirm is yes
        let confirm = app.world().get::<Confirm>(entity).unwrap();
        assert_eq!(confirm.yes, true);

        // Simulate ArrowLeft press (should set yes to false)
        simulate_key_press(&mut app, KeyCode::ArrowLeft);

        // Verify confirm is no
        let confirm = app.world().get::<Confirm>(entity).unwrap();
        assert_eq!(confirm.yes, false);

        // Test Enter key (should trigger submit, but we'll just verify it doesn't crash)
        // Note: We can't easily test the Submit event without setting up observers,
        // but we can verify the state remains unchanged
        simulate_key_press(&mut app, KeyCode::KeyY);
        let confirm_before = app.world().get::<Confirm>(entity).unwrap();
        assert_eq!(confirm_before.yes, true);

        simulate_key_press(&mut app, KeyCode::Enter);
        // After Enter, the state should still be true (Enter submits but doesn't change state)
        let confirm_after = app.world().get::<Confirm>(entity).unwrap();
        assert_eq!(confirm_after.yes, true);

        // Test Escape key (should trigger cancel submit)
        simulate_key_press(&mut app, KeyCode::Escape);
        // State should remain unchanged
        let confirm_after_escape = app.world().get::<Confirm>(entity).unwrap();
        assert_eq!(confirm_after_escape.yes, true);
    }
}
