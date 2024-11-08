use super::{Feedback, Prompt};
use crate::{construct::*, AskyEvent, Error, Submitter, Focus, Focusable, FocusParam};
use bevy::prelude::*;

use std::borrow::Cow;

#[derive(Component, Reflect)]
pub struct Checkbox {
    /// Initial checkbox of the prompt.
    pub checked: bool,
}

unsafe impl Submitter for Checkbox {
    type Out = bool;
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
    app.add_systems(PreUpdate, (checkbox_controller, checkbox_group_controller));
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
            .insert(NodeBundle::default())
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
    mut commands: Commands,
) {
    for (id, mut checkbox) in query.iter_mut() {
        if !focus.is_focused(id) {
            continue;
        }
        if input.any_just_pressed([KeyCode::Space, KeyCode::KeyH, KeyCode::KeyL, KeyCode::Enter]) {
            if input.just_pressed(KeyCode::Space) {
                checkbox.checked = !checkbox.checked;
            }
            if input.any_just_pressed([KeyCode::KeyL]) {
                checkbox.checked = true;
            }
            if input.any_just_pressed([KeyCode::KeyH]) {
                checkbox.checked = false;
            }

            if input.just_pressed(KeyCode::Enter) {
                let yes = checkbox.checked;
                // requests.send(NavRequest::Move(NavDirection::South));
                // I had tried using triggers in bevy_ui_navigation to fix my issues.
                // commands.trigger(NavRequest::Move(NavDirection::South));
                commands.trigger_targets(AskyEvent::<bool>(Ok(yes)), id);
                // commands
                //     .entity(id)
                //     .insert(Feedback::info(if yes { "Yes" } else { "No" }));
            }
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

#[derive(Component, Reflect)]
pub struct CheckboxGroup;

unsafe impl Submitter for CheckboxGroup {
    type Out = Vec<bool>;
}

impl Construct for CheckboxGroup {
    type Props = Vec<Cow<'static, str>>;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let mut commands = context.world.commands();
        let mut children = vec![];
        let group = context.id;
        commands
            .entity(context.id)
            // .insert(Focusable::default())
            // .insert(MenuSetting::default())
            // .insert(MenuBuilder::Root)
            // .insert(TextBundle::from_section("header", TextStyle::default()))
            .insert(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            })
            // .insert(Focusable::default())
            .with_children(|parent| {
                // let mut entity_commands = parent.column();

                for prompt in props {
                    let id = parent
                        .construct::<Checkbox>(prompt)
                        .id();
                    children.push(id);
                }
            });

        context.world.flush();
        Ok(CheckboxGroup)
    }
}

// fn add_menu_builders(query: Query<&MenuSetting, (Without<MenuBuild

fn checkbox_group_controller(
    mut query: Query<(Entity, &CheckboxGroup, &Children)>,
    checkboxes: Query<(Entity, &Checkbox)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut focus: FocusParam,
) {
    if input.any_just_pressed([KeyCode::Escape, KeyCode::Enter]) {
        for (id, group, children) in query.iter_mut() {
            if checkboxes
                .iter_many(children)
                .any(|(id, _)| focus.is_focused(id))
            {
                if input.just_pressed(KeyCode::Enter) {
                    let result: Vec<bool> = checkboxes
                        .iter_many(children)
                        .map(|(_, checkbox)| checkbox.checked)
                        .collect();
                    commands.trigger_targets(AskyEvent(Ok(result)), id);
                    focus.block(id);
                    // focus.move_focus(id);
                    // *state = AskyState::Complete;
                }

                if input.just_pressed(KeyCode::Escape) {
                    commands.trigger_targets(AskyEvent::<String>(Err(Error::Cancel)), id);
                    commands.entity(id).insert(Feedback::error("canceled"));
                }
            }
        }
    }
}
