//! Playing around with [Cart's proposal](https://github.com/bevyengine/bevy/discussions/14437).
use crate::Submitter;
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::borrow::Cow;
use std::marker::PhantomData;
use thiserror::Error;

/// Construction error
#[derive(Error, Debug)]
pub enum ConstructError {
    /// Invalid properties
    #[error("invalid properties {message:?}")]
    InvalidProps {
        /// Message
        message: Cow<'static, str>
    },
    /// Missing resource
    #[error("missing resource {message:?}")]
    MissingResource {
        /// Message
        message: Cow<'static, str>
    },
}

/// Construct property
pub enum ConstructProp<T: Construct> {
    /// Direct Value
    Value(T),
    /// Properties
    Prop(T::Props),
}

/// Construct driver trait
pub trait Construct: Sized {
    /// Properties must be Clone.
    ///
    /// NOTE: Cart's proposal states they must also be Default,
    /// but I had trouble making that work.
    type Props: Clone;

    /// Construct an item.
    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError>;

    /// Make a patch.
    fn patch<F: FnMut(&mut Self::Props)>(func: F) -> ConstructPatch<Self, F> {
        ConstructPatch {
            func,
            _marker: PhantomData,
        }
    }
}

/// Add a silent partner.
#[derive(Bundle)]
pub struct Add<A: Sync + Send + 'static + Bundle, B: Sync + Send + 'static + Bundle>(pub A, pub B);

unsafe impl<A: Submitter + Sync + Send + 'static + Bundle, B: Sync + Send + 'static + Bundle>
    Submitter for Add<A, B>
{
    /// Output of submitter.
    type Out = A::Out;
}

impl<A, B> Construct for Add<A, B>
where
    A: Construct + Sync + Send + 'static + Bundle,
    B: Construct<Props = ()> + Sync + Send + 'static + Bundle,
{
    type Props = A::Props;
    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        let a = A::construct(context, props)?;
        let b = B::construct(context, ())?;
        Ok(Add(a, b))
    }
}

impl<A, B> Construct for (A, B)
where
    A: Construct,
    B: Construct,
{
    type Props = (A::Props, B::Props);
    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        let a = A::construct(context, props.0)?;
        let b = B::construct(context, props.1)?;
        Ok((a, b))
    }
}

/// An entity and a mutable world
#[derive(Debug)]
pub struct ConstructContext<'a> {
    /// Entity to use for construction
    pub id: Entity,
    /// World
    pub world: &'a mut World,
}

impl<'a> ConstructContext<'a> {
    /// Construct helper function
    pub fn construct<T: Construct>(
        &mut self,
        props: impl Into<T::Props>,
    ) -> Result<T, ConstructError> {
        T::construct(self, props.into())
    }

    /// Construct from patch
    pub fn construct_from_patch<P: Patch>(
        &mut self,
        patch: &mut P,
    ) -> Result<P::Construct, ConstructError>
    where
        <<P as Patch>::Construct as Construct>::Props: Default,
    {
        let mut props = <<P as Patch>::Construct as Construct>::Props::default();
        patch.patch(&mut props);
        self.construct(props)
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

/// Construct extension
///
/// The main touch point for the user.
pub trait ConstructExt {
    /// Construct a type using the given properties.
    fn construct<T: Construct + Bundle>(&mut self, props: impl Into<T::Props>) -> EntityCommands
    where
        <T as Construct>::Props: Send;
}

/// Construct children extension
pub trait ConstructChildrenExt: ConstructExt {
    /// Construct a series of children using the given properties.
    fn construct_children<T: Construct + Bundle>(
        &mut self,
        props: impl IntoIterator<Item = impl Into<T::Props>>,
    ) -> EntityCommands
    where
        <T as Construct>::Props: Send;
}

struct ConstructCommand<T: Construct>(T::Props);

impl<T: Construct + Bundle> bevy::ecs::system::EntityCommand for ConstructCommand<T>
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
    fn construct<T: Construct + Bundle>(&mut self, props: impl Into<T::Props>) -> EntityCommands
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
    fn construct<T: Construct + Bundle>(&mut self, props: impl Into<T::Props>) -> EntityCommands
    where
        <T as Construct>::Props: Send,
    {
        let mut s = self.spawn_empty();
        s.add(ConstructCommand::<T>(props.into()));
        s
    }
}

impl<'w> ConstructExt for EntityCommands<'w> {
    // type Out = EntityCommands;
    fn construct<T: Construct + Bundle>(&mut self, props: impl Into<T::Props>) -> EntityCommands
    where
        <T as Construct>::Props: Send,
    {
        self.add(ConstructCommand::<T>(props.into()));
        self.reborrow()
    }
}

impl<'w> ConstructChildrenExt for EntityCommands<'w> {
    fn construct_children<T: Construct + Bundle>(
        &mut self,
        props: impl IntoIterator<Item = impl Into<T::Props>>,
    ) -> EntityCommands
    where
        <T as Construct>::Props: Send,
    {
        self.with_children(|parent| {
            for prop in props.into_iter() {
                parent.construct::<T>(prop);
            }
        });
        self.reborrow()
    }
}

// I couldn't have this an the tuple construct.
// impl<T: Default + Clone> Construct for T {
//     type Props = T;
//     #[inline]
//     fn construct(
//         _context: &mut ConstructContext,
//         props: Self::Props,
//     ) -> Result<Self, ConstructError> {
//         Ok(props)
//     }
// }

/// Modifies properties
pub trait Patch: Send + Sync + 'static {
    /// Of what type
    type Construct: Construct + Bundle;
    /// Modify properties
    fn patch(&mut self, props: &mut <Self::Construct as Construct>::Props);
}

/// Generic patch based on closure
pub struct ConstructPatch<C: Construct, F> {
    func: F,
    _marker: PhantomData<C>,
}

impl<C: Construct + Sync + Send + 'static + Bundle,
     F: FnMut(&mut C::Props) + Sync + Send + 'static>
    Patch for ConstructPatch<C, F>
{
    type Construct = C;
    fn patch(&mut self, props: &mut <Self::Construct as Construct>::Props) {
        (self.func)(props);
    }
}

// pub trait PatchExt {
//     type C: Construct;
//     fn patch<F: FnMut(&mut <<Self as PatchExt>::C as Construct>::Props)>(func: F) -> ConstructPatch<Self::C, F> {
//         ConstructPatch {
//             func,
//             _marker: PhantomData
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Default, Clone, Component)]
    struct Player {
        name: String,
    }

    impl Construct for Player {
        type Props = Player;
        fn construct(
            _context: &mut ConstructContext,
            props: Self::Props,
        ) -> Result<Self, ConstructError> {
            Ok(props)
        }
    }

    #[test]
    fn test_patch_name() {
        let mut player = Player {
            name: "shane".into(),
        };
        assert_eq!(player.name, "shane");

        let mut patch = Player::patch(|props| {
            props.name = "fred".to_string();
        });
        patch.patch(&mut player);
        assert_eq!(player.name, "fred");
    }
}
