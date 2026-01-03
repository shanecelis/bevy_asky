use crate::{construct::*, prelude::*, Part};
use bevy::prelude::*;

use std::borrow::Cow;

/// Checkbox
#[derive(Component, Reflect)]
pub struct Checkbox {
    /// Initial checkbox of the prompt
    pub checked: bool,
}

// impl From<Cow<'static, str>> for Checkbox {
//     fn from(message: Cow<'static, str>) -> Self {
//         Checkbox {
//             message,
//             checked: false,
//         }
//     }
// }

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        PreUpdate,
        (checkbox_controller, checkbox_group_controller).in_set(AskySet::Controller),
    );
}

impl Part for Checkbox {
    type Group = CheckboxGroup;
}

impl Construct for Checkbox {
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
        Ok(Checkbox { checked: false })
    }
}

fn checkbox_controller(
    focus: Focus,
    mut query: Query<(Entity, &mut Checkbox)>,
    input: Res<ButtonInput<KeyCode>>,
    // mut requests: EventWriter<NavRequest>,
) {
    use KeyCode::*;

    if input.any_just_pressed([Space, KeyY, KeyN]) {
        for (id, mut checkbox) in query.iter_mut() {
            if !focus.is_focused(id) {
                continue;
            }
            if input.just_pressed(Space) {
                checkbox.checked = !checkbox.checked;
            }
            if input.any_just_pressed([KeyY]) {
                checkbox.checked = true;
            }
            if input.any_just_pressed([KeyN]) {
                checkbox.checked = false;
            }

            // if input.just_pressed(Enter) {
            //     let yes = checkbox.checked;
            //     // requests.send(NavRequest::Move(NavDirection::South));
            //     // I had tried using triggers in bevy_ui_navigation to fix my issues.
            //     // commands.trigger(NavRequest::Move(NavDirection::South));
            //     commands.trigger_targets(Submit::<bool>(Ok(yes)), id);
            //     // commands
            //     //     .entity(id)
            //     //     .insert(Feedback::info(if yes { "Yes" } else { "No" }));
            // }
        }
    }
}

// impl Component for Checkbox {
//     const STORAGE_TYPE: StorageType = StorageType::Table;

//     fn register_component_hooks(hooks: &mut ComponentHooks) {
//         hooks.on_add(|mut world, targeted_entity, _component_id| {
//             if world.get::<ConfirmState>(targeted_entity).is_none() {
//                 let confirm_init = world.get::<Checkbox>(targeted_entity).unwrap().init;
//                 let mut commands = world.commands();
//                 commands
//                     .entity(targeted_entity)
//                     .insert(ConfirmState { yes: confirm_init });
//             }
//         });
//     }
// }

/// Checkbox Group
///
/// Given to parent of checkbox group that handles submission.
#[derive(Component, Reflect, Default)]
pub struct CheckboxGroup;

unsafe impl Submitter for CheckboxGroup {
    type Out = Vec<bool>;
}

impl Construct for CheckboxGroup {
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
        Ok(CheckboxGroup)
    }
}

fn checkbox_group_controller(
    mut query: Query<(Entity, &Children), With<CheckboxGroup>>,
    checkboxes: Query<(Entity, &Checkbox)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    focus: FocusParam,
) {
    if !input.any_just_pressed([KeyCode::Escape, KeyCode::Enter]) {
        return;
    }
    for (id, children) in query.iter_mut() {
        if children.iter().any(|id| focus.is_focused(id)) {
            if input.just_pressed(KeyCode::Enter) {
                let result: Vec<bool> = checkboxes
                    .iter_many(children)
                    .map(|(_, checkbox)| checkbox.checked)
                    .collect();
                commands.trigger(Submit::new(id, Ok(result)));
            }

            if input.just_pressed(KeyCode::Escape) {
                commands.trigger(Submit::<Vec<bool>>::new(id, Err(Error::Cancel)));
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
    fn test_checkbox_key_presses() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AskyPlugin)
            .add_event::<bevy::input::keyboard::KeyboardInput>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<bevy::input_focus::InputFocus>();

        // Create a Checkbox entity with required components
        let entity = app
            .world_mut()
            .spawn((
                Checkbox { checked: false },
                Focusable::default(),
                Prompt(Cow::Borrowed("Test checkbox")),
            ))
            .id();

        // Run update to let reset_focus system automatically set focus
        app.update();

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
        let checkbox = app.world().get::<Checkbox>(entity).unwrap();
        assert_eq!(checkbox.checked, false);

        // Simulate Space key press (should toggle to true)
        simulate_key_press(&mut app, KeyCode::Space);

        // Verify checkbox is now checked
        let checkbox = app.world().get::<Checkbox>(entity).unwrap();
        assert_eq!(checkbox.checked, true);

        // Simulate Space key press again (should toggle back to false)
        simulate_key_press(&mut app, KeyCode::Space);

        // Verify checkbox is unchecked again
        let checkbox = app.world().get::<Checkbox>(entity).unwrap();
        assert_eq!(checkbox.checked, false);

        // Simulate KeyY press (should set to true)
        simulate_key_press(&mut app, KeyCode::KeyY);

        // Verify checkbox is checked
        let checkbox = app.world().get::<Checkbox>(entity).unwrap();
        assert_eq!(checkbox.checked, true);

        // Simulate KeyN press (should set to false)
        simulate_key_press(&mut app, KeyCode::KeyN);

        // Verify checkbox is unchecked
        let checkbox = app.world().get::<Checkbox>(entity).unwrap();
        assert_eq!(checkbox.checked, false);

        // Test toggle again with Space
        simulate_key_press(&mut app, KeyCode::Space);
        let checkbox = app.world().get::<Checkbox>(entity).unwrap();
        assert_eq!(checkbox.checked, true);

        simulate_key_press(&mut app, KeyCode::Space);
        let checkbox = app.world().get::<Checkbox>(entity).unwrap();
        assert_eq!(checkbox.checked, false);
    }
}
