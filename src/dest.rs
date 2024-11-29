use super::*;
use bevy::ecs::system::EntityCommands;
use std::fmt::Debug;

/// The destination for constructing new entities
#[derive(Clone, Debug)]
pub enum Dest {
    /// A new new entity at the root
    Root,
    /// Replace an existing entity
    Replace(Entity),
    /// Replace an existing entity's children
    ReplaceChildren(Entity),
    /// Add to an existing entity's children
    Append(Entity),
}

impl From<Entity> for Dest {
    fn from(id: Entity) -> Dest {
        Dest::Replace(id)
    }
}

impl Dest {
    /// Return the appropriate [EntityCommands].
    pub fn entity<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
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
                commands.entity(*id).despawn_descendants();
                let mut child = None;
                commands.entity(*id).with_children(|parent| {
                    child = Some(parent.spawn_empty().id());
                });
                commands.entity(child.unwrap())
            }
            Root => commands.spawn_empty(),
        }
    }

    /// A fallible version of `entity()`.
    pub fn get_entity<'a>(&self, commands: &'a mut Commands) -> Option<EntityCommands<'a>> {
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
