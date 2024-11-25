use crate::{construct::*, prelude::*, Part, view::widget::Widgets};
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
    app.add_systems(Update, (radio_controller, radio_group_controller).in_set(AskySet::Controller));
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
            .insert(Focusable::default())
            .insert(Prompt(props.clone()))
            .insert(AccessibilityNode(NodeBuilder::new(Role::RadioButton)));
        commands.trigger(AddView(context.id));
        context.world.flush();
        Ok(Radio { checked: false })
    }
}

fn radio_controller(
    focus: FocusParam,
    mut query: Query<(Entity, &mut Radio, Option<&Parent>)>,
    child_query: Query<&Children>,
    input: Res<ButtonInput<KeyCode>>,
    mut toggled: Local<Vec<(Entity, Entity)>>,
) {
    toggled.clear();
    let changed = false;
    for (id, mut radio, parent) in query.iter_mut() {
        if !focus.is_focused(id) {
            continue;
        }
        if input.any_just_pressed([
            KeyCode::Space,
            KeyCode::KeyH,
            KeyCode::KeyL,
        ]) {
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
                parent.spawn(TextBundle::from_section(props, TextStyle::default()));

            });
        commands.trigger(AddView(context.id));

        context.world.flush();
        Ok(RadioGroup)
    }
}

fn radio_group_controller(
    mut query: Query<(Entity, &Children), With<RadioGroup>>,
    radios: Query<(Entity, &Radio)>,
    focus: FocusParam,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if !input.any_just_pressed([KeyCode::Escape, KeyCode::Enter, KeyCode::ArrowDown, KeyCode::ArrowUp]) {
        return;
    }
    for (id, children) in query.iter_mut() {
        if let Some(index) = radios
            .iter_many(children)
            .position(|(id, _)| focus.is_focused(id))
        {
            if input.just_pressed(KeyCode::Enter) {
                if let Some(selection) = radios
                    .iter_many(children)
                    .position(|(_, radio)| radio.checked) {
                        // commands.trigger_targets(AskyEvent(selection.ok_or(Error::InvalidInput)), id);
                        commands.trigger_targets(AskyEvent(Ok(selection)), id);
                    } else {
                        commands.entity(id).insert(Feedback::warn("must select one"));
                    }
            }

            if input.just_pressed(KeyCode::Escape) {
                commands.trigger_targets(AskyEvent::<usize>(Err(Error::Cancel)), id);
                commands.entity(id).insert(Feedback::error("canceled"));
            }
        }
    }
}
