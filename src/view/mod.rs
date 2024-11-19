use crate::construct::*;
use crate::prelude::*;
use bevy::{
    prelude::*,
    ecs::{
        world::Command,
        system::{SystemParam, SystemId},
    }
};

pub mod ascii;
// pub mod button;
pub mod click;
pub mod color;
// pub(crate) mod interaction;
pub mod widget;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<ViewHook>();
}

#[derive(Component)]
pub struct Question;

#[derive(Component)]
// pub struct Answer<T>(T);
pub enum Answer<T> {
    Selection(T),
    Final, //(Option<T>)
}

#[derive(Resource, Default, Debug)]
pub struct ViewHook(Option<SystemId<Entity>>);

impl ViewHook {

    // pub fn run_hook_commands<'w,'s>(context: &'s mut ConstructContext<'w>) -> Commands<'w,'s> {
    //     let view_hook = context.world.resource::<ViewHook>();
    //     let cmd = view_hook.add_view_cmd(context.id);
    //     let mut commands = context.world.commands();
    //     if let Some(cmd) = cmd {
    //         commands.add(cmd);
    //     }
    //     commands
    // }
    pub fn run_hook_commands(id: Entity, world: &mut World) -> Commands {
        let view_hook = world.resource::<ViewHook>();
        let cmd = view_hook.add_view_cmd(id);
        let mut commands = world.commands();
        if let Some(cmd) = cmd {
            commands.add(cmd);
        }
        commands
    }

    pub fn add_view_cmd(&self, id: Entity) -> Option<impl Command> {
        self.0.map(|system_id| {
            move |world: &mut World| {
                if let Err(e) = world.run_system_with_input(system_id, id) {
                    warn!("error in ViewHook: {e}");
                }
            }
        })
    }

    pub fn add_view(&self, id: Entity, commands: &mut Commands) -> bool {
        if let Some(system_id) = self.0 {
            commands.run_system_with_input(system_id, id);
            true
        } else {
            // warn?
            false
        }
    }

    pub fn run_add_view(id: Entity, world: &mut World) -> bool {
        if let Some(system_id) = world.resource::<Self>().0 {
            world.run_system_with_input(system_id, id);
            true
        } else {
            // warn?
            false
        }
    }

    pub fn queue_add_view(id: Entity, world: &World, commands: &mut Commands) -> bool {
        if let Some(system_id) = world.resource::<Self>().0 {
            commands.run_system_with_input(system_id, id);
            true
        } else {
            // warn?
            false
        }
    }
}

pub fn add_view_to_checkbox<V>(
    checkboxes: Query<(Entity, &Parent), Added<Checkbox>>,
    group: Query<&CheckboxGroup, With<V>>,
    mut commands: Commands,
) where
    V: Construct<Props = ()> + Component + Send,
{
    for (id, parent) in &checkboxes {
        if group.get(parent.get()).is_ok() {
            commands.entity(id).construct::<V>(());
        }
    }
}

// pub(crate) fn add_view_to_checkbox<V>(
//     group: Query<&Children, (Added<CheckboxGroup>, With<V>)>,
//     checkboxes: Query<Entity, With<Checkbox>>,
//     mut commands: Commands,
// ) where
//     V: Construct<Props = ()> + Component + Send,
// {
//     for children in &group {
//         for id in checkboxes.iter_many(children) {
//             commands.entity(id).construct::<V>(());
//         }
//     }
// }

pub fn add_view_to_radio<V>(
    radios: Query<(Entity, &Parent), Added<Radio>>,
    group: Query<&RadioGroup, With<V>>,
    mut commands: Commands,
) where
    V: Construct<Props = ()> + Component + Send,
{
    for (id, parent) in &radios {
        if group.get(parent.get()).is_ok() {
            commands.entity(id).construct::<V>(());
        }
    }
}

pub fn replace_or_insert(text: &mut Text, index: usize, replacement: &str) {
    let len = text.sections.len();
    if len <= index {
        for _ in len.saturating_sub(1)..index {
            text.sections.push(TextSection::default());
        }
        text.sections.push(TextSection::from(replacement));
    } else {
        text.sections[index].value.replace_range(.., replacement);
    }
}

pub fn replace_or_insert_rep(
    text: &mut Text,
    index: usize,
    replacement: &str,
    repetition: usize,
) {
    let len = text.sections.len();
    if len <= index {
        for _ in len.saturating_sub(1)..index {
            text.sections.push(TextSection::default());
        }
        // This allocates a string, which is fine because TextSection needs one.
        text.sections
            .push(TextSection::from(replacement.repeat(repetition)));
    } else {
        text.sections[index].value.clear();
        for _ in 0..repetition {
            // This doesn't allocate a string.
            text.sections[index].value.push_str(replacement);
        }
    }
}
