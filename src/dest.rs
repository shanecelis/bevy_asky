use super::*;
use crate::{construct::*, view::*, sync::AskyCommands};
use bevy::{ecs::system::SystemParam, prelude::*};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Dest {
    Root,
    Replace(Entity),
    ReplaceChildren(Entity),
    Append(Entity)
}

impl From<Entity> for Dest {
    fn from(id: Entity) -> Dest {
        Dest::Replace(id)
    }
}

impl Dest {
    pub fn entity_commands<'a>(&self, commands: &'a mut Commands) -> bevy::ecs::system::EntityCommands<'a> {
        use Dest::*;
        match self {
            Append(id) => {
                let mut child = None;
                commands.entity(*id).with_children(|parent| {
                    child = Some(parent.spawn_empty().id());
                });
                commands.entity(child.unwrap())
            }
            Replace(id) => commands.entity(*id),
            ReplaceChildren(id) => {
                commands.entity(*id)
                    .despawn_descendants();
                let mut child = None;
                commands.entity(*id).with_children(|parent| {
                    child = Some(parent.spawn_empty().id());
                });
                commands.entity(child.unwrap())
            }
            Root => commands.spawn_empty(),
        }
    }

    pub fn get_entity_commands<'a>(&self, commands: &'a mut Commands) -> Option<bevy::ecs::system::EntityCommands<'a>> {
        use Dest::*;
        match self {
            Append(id) => {
                let mut child = None;
                if let Some(mut ecommands) = commands.get_entity(*id) {
                    ecommands.with_children(|parent| {
                        child = Some(parent.spawn_empty().id());
                    });
                }
                child.and_then(|id| commands.get_entity(id))
            }
            Replace(id) => commands.get_entity(*id),
            ReplaceChildren(id) => {
                if let Some(mut ecommands) = commands.get_entity(*id) {
                    ecommands.despawn_descendants();
                }
                let mut child = None;
                if let Some(mut ecommands) = commands.get_entity(*id) {
                    ecommands.with_children(|parent| {
                        child = Some(parent.spawn_empty().id());
                    });
                }
                child.and_then(|id| commands.get_entity(id))
            }
            Root => Some(commands.spawn_empty()),
        }
    }
}
