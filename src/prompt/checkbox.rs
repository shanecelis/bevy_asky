use super::{Feedback, Prompt};
use crate::construct::*;
use crate::{AskyEvent, Error, Submitter};
use bevy::prelude::*;
use bevy_alt_ui_navigation_lite::prelude::*;
use std::borrow::Cow;

#[derive(Component)]
pub struct Checkbox {
    /// Message used to display in the prompt.
    pub message: Cow<'static, str>,
    /// Initial checkbox of the prompt.
    pub checked: bool,
}

impl From<Cow<'static, str>> for Checkbox {
    fn from(message: Cow<'static, str>) -> Self {
        Checkbox {
            message,
            checked: false,
        }
    }
}

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
            .insert(Focusable::default())
            .insert(Prompt(props.clone()));

        context.world.flush();
        Ok(Checkbox {
            message: props,
            checked: false,
        })
    }
}

fn checkbox_controller(
    mut query: Query<(&mut Checkbox, &Focusable)>,
    input: Res<ButtonInput<KeyCode>>,
    commands: Commands,
) {
    for (mut checkbox, focusable) in query.iter_mut() {
        if FocusState::Focused != focusable.state() {
            continue;
        }
        if input.any_just_pressed([KeyCode::Space, KeyCode::KeyH, KeyCode::KeyL]) {
            if input.just_pressed(KeyCode::Space) {
                checkbox.checked = !checkbox.checked;
            }
            if input.any_just_pressed([KeyCode::KeyL]) {
                checkbox.checked = true;
            }
            if input.any_just_pressed([KeyCode::KeyH]) {
                checkbox.checked = false;
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

#[derive(Component)]
pub struct CheckboxGroup;

// pub struct CheckboxGroupProps {
//     options: Vec<Cow<'static, str>>,
// };
//
impl Submitter for CheckboxGroup {
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
        commands
            .entity(context.id)
            .insert(Focusable::default())
            .insert(MenuSetting::default())
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
                    parent
                        .construct::<Checkbox>(prompt)
                        // FIXME: Don't want to specify view here.
                        .construct::<crate::view::ascii::View>(())
                        .insert(MenuBuilder::EntityParent(context.id))

                        ;
                }
            })
            ;

        context.world.flush();
        Ok(CheckboxGroup)
    }
}

fn checkbox_group_controller(
    mut query: Query<(Entity, &CheckboxGroup, &Focusable,
                      &Children)>,
    checkboxes: Query<&Checkbox>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (id, checkbox, focusable, children) in query.iter_mut() {
        // dbg!(focusable.state());
        if !matches!(focusable.state(), FocusState::Active | FocusState::Focused)  {
            continue;
        }
        if input.just_pressed(KeyCode::Enter) {
            let mut result: Vec<bool> = vec![];
            for child in children {
                let checkbox = checkboxes.get(*child).unwrap();
                result.push(checkbox.checked);
            }
            commands.trigger_targets(AskyEvent(Ok(result)), id);
            // *state = AskyState::Complete;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.trigger_targets(AskyEvent::<String>(Err(Error::Cancel)), id);
            commands.entity(id).insert(Feedback::error("canceled"));
        }
    }
}
