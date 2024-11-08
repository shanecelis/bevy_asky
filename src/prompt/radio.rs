use crate::{construct::*, prelude::*};
use bevy::{
    a11y::{accesskit::*, AccessibilityNode},
    prelude::*,
};
use std::borrow::Cow;

#[derive(Component, Reflect)]
pub struct Radio {
    /// Initial radio of the prompt.
    pub checked: bool,
}

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, (radio_controller, radio_group_controller));
}

impl Construct for Radio {
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
            .insert(Prompt(props.clone()))
            .insert(AccessibilityNode(NodeBuilder::new(Role::RadioButton)));

        context.world.flush();
        Ok(Radio { checked: false })
    }
}

fn radio_controller(
    focus: Focus,
    mut query: Query<(Entity, &mut Radio, Option<&Parent>)>,
    child_query: Query<&Children>,
    input: Res<ButtonInput<KeyCode>>,
    mut toggled: Local<Vec<(Entity, Entity)>>,
) {
    toggled.clear();
    for (id, mut radio, parent) in query.iter_mut() {
        if !focus.is_focused(id) {
            continue;
        }
        if input.any_just_pressed([
            KeyCode::Space,
            KeyCode::KeyH,
            KeyCode::KeyL,
            KeyCode::Enter,
            KeyCode::Escape,
        ]) {
            let was_checked = radio.checked;

            if input.just_pressed(KeyCode::Space) {
                radio.checked = true;
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
                    toggled.push((id, **p));
                }
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

#[derive(Component, Reflect)]
pub struct RadioGroup;

unsafe impl Submitter for RadioGroup {
    type Out = usize;
}

impl Construct for RadioGroup {
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
                        .construct::<Radio>(prompt)
                        .insert(Focusable::default())
                        .id();
                    children.push(id);
                }
            });

        context.world.flush();
        Ok(RadioGroup)
    }
}

// fn add_menu_builders(query: Query<&MenuSetting, (Without<MenuBuild

fn radio_group_controller(
    mut query: Query<(Entity, &RadioGroup, &Children)>,
    radios: Query<(Entity, &Radio)>,
    focus: Focus,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if !input.any_just_pressed([KeyCode::Escape, KeyCode::Enter]) {
        return;
    }
    for (id, group, children) in query.iter_mut() {
        if radios
            .iter_many(children)
            .any(|(id, _)| focus.is_focused(id))
        {
            if input.just_pressed(KeyCode::Enter) {
                let selection = radios
                    .iter_many(children)
                    .position(|(_, radio)| radio.checked);

                commands.trigger_targets(AskyEvent(selection.ok_or(Error::InvalidInput)), id);
                // requests.send(NavRequest::ScopeMove(ScopeDirection::Next));
                // *state = AskyState::Complete;
            }

            if input.just_pressed(KeyCode::Escape) {
                commands.trigger_targets(AskyEvent::<String>(Err(Error::Cancel)), id);
                commands.entity(id).insert(Feedback::error("canceled"));
            }
        }
    }
}
