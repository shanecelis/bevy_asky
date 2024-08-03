//! Playing around with [Cart's proposal](https://github.com/bevyengine/bevy/discussions/14437).
use bevy::{prelude::*, ecs::system::EntityCommands};
use std::borrow::Cow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConstructError {
    #[error("invalid properties {message:?}")]
    InvalidProps { message: Cow<'static, str> },
    #[error("missing resource {message:?}")]
    MissingResource { message: Cow<'static, str> },
}

pub struct Requirements {}

pub enum ConstructProp<T: Construct> {
    Value(T),
    Prop(T::Props),
}

pub trait Construct: Sized {
    // type Props: Default + Clone;
    type Props: Clone;
    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError>;
}

#[derive(Debug)]
pub struct ConstructContext<'a> {
    pub id: Entity,
    pub world: &'a mut World,
}

impl<'a> ConstructContext<'a> {
    pub fn construct<T: Construct>(
        &mut self,
        props: impl Into<T::Props>,
    ) -> Result<T, ConstructError> {
        T::construct(self, props.into())
    }
}

// impl<T: Asset> Construct for Handle<T> {
//     type Props = AssetPath<'static>;

//     fn construct(
//         context: &mut ConstructContext,
//         path: Self::Props,
//     ) -> Result<Self, ConstructError> {
//         // if let Err(err) = path.validate() {
//         //     return Err(ConstructError::InvalidProps {
//         //         message: format!("Invalid Asset Path: {err}").into(),
//         //     });
//         // }
//         Ok(context.world.resource::<AssetServer>().load(path))
//     }
// }

pub trait ConstructExt {
    fn construct<T: Construct + Component>(&mut self, props: impl Into<T::Props>) -> EntityCommands
    where
        <T as Construct>::Props: Send;
}

struct ConstructCommand<T: Construct>(T::Props);

impl<T: Construct + Component> bevy::ecs::system::EntityCommand for ConstructCommand<T>
where
    <T as Construct>::Props: Send,
{
    fn apply(self, id: Entity, world: &mut World) {
        let mut context = ConstructContext { id, world };
        let c = T::construct(&mut context, self.0).expect("component");
        world.entity_mut(id).insert(c);
    }
}

impl<'w> ConstructExt for Commands<'w, '_> {
    // type Out = EntityCommands;
    fn construct<T: Construct + Component>(&mut self, props: impl Into<T::Props>) -> EntityCommands
    where
        <T as Construct>::Props: Send,
    {
        let mut s = self.spawn_empty();
        s.add(ConstructCommand::<T>(props.into()));
        s
    }
}

impl<'w> ConstructExt for ChildBuilder<'w> {
    // type Out = EntityCommands;
    fn construct<T: Construct + Component>(&mut self, props: impl Into<T::Props>) -> EntityCommands
    where
        <T as Construct>::Props: Send,
    {
        let mut s = self.spawn_empty();
        s.add(ConstructCommand::<T>(props.into()));
        s
    }
}

impl<'w> ConstructExt for bevy::ecs::system::EntityCommands<'w> {
    // type Out = EntityCommands;
    fn construct<T: Construct + Component>(&mut self, props: impl Into<T::Props>) -> EntityCommands
    where
        <T as Construct>::Props: Send,
    {
        self.add(ConstructCommand::<T>(props.into()));
        self.reborrow()
    }
}

impl<T: Default + Clone> Construct for T {
    type Props = T;
    #[inline]
    fn construct(
        _context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        Ok(props)
    }
}
