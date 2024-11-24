use crate::{construct::*, prelude::*, Part, view::widget::Widgets};
use bevy::prelude::*;

use std::borrow::Cow;

#[derive(Component, Reflect)]
pub struct Checkbox {
    /// Initial checkbox of the prompt.
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
    app.add_systems(PreUpdate, (checkbox_controller, checkbox_group_controller).in_set(AskySet::Controller));
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
            // .insert(NeedsView)
            .insert(Prompt(props.clone()));
        // commands.construct::<NeedsView>(());
        commands.trigger(AddView(context.id));
        context.world.flush();
        Ok(Checkbox { checked: false })
    }
}

fn checkbox_controller(
    focus: Focus,
    mut query: Query<(Entity, &mut Checkbox)>,
    input: Res<ButtonInput<KeyCode>>,
    // mut requests: EventWriter<NavRequest>,
    commands: Commands,
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
            //     commands.trigger_targets(AskyEvent::<bool>(Ok(yes)), id);
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
                parent.spawn(TextBundle::from_section(props, TextStyle::default()));
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
        if children.iter()
            .any(|id| focus.is_focused(*id))
        {
            if input.just_pressed(KeyCode::Enter) {
                let result: Vec<bool> = checkboxes
                    .iter_many(children)
                    .map(|(_, checkbox)| checkbox.checked)
                    .collect();
                commands.trigger_targets(AskyEvent(Ok(result)), id);
            }

            if input.just_pressed(KeyCode::Escape) {
                commands.trigger_targets(AskyEvent::<Vec<bool>>(Err(Error::Cancel)), id);
            }
        }
    }
}
