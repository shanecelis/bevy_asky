//! Playing around with Cart's proposal.
use bevy::{
    prelude::*,
    asset::AssetPath,
};
use std::borrow::Cow;

pub enum ConstructError {
    InvalidProps { message: Cow<'static, str> }
}

pub trait Construct: Sized {
    type Props: Default + Clone;
    fn construct(context: &mut ConstructContext, props: Self::Props) -> Result<Self, ConstructError>;
}

pub struct ConstructContext<'a> {
    pub id: Entity,
    pub world: &'a mut World,
}

impl<T: Asset> Construct for Handle<T> {
    type Props = AssetPath<'static>;

    fn construct(
        context: &mut ConstructContext,
        path: Self::Props,
    ) -> Result<Self, ConstructError> {
        // if let Err(err) = path.validate() {
        //     return Err(ConstructError::InvalidProps {
        //         message: format!("Invalid Asset Path: {err}").into(),
        //     });
        // }
        Ok(context.world.resource::<AssetServer>().load(path))
    }
}

pub trait ConstructExt {
    fn construct<T: Construct + Component>(&mut self, props: T::Props) -> &mut Self where <T as Construct>::Props: Send;
}

struct ConstructCommand<T: Construct>(T::Props);

impl<T: Construct + Component> bevy::ecs::system::EntityCommand for ConstructCommand<T>
where <T as Construct>::Props: Send {
    fn apply(self, id: Entity, world: &mut World) {
        let mut context = ConstructContext {
            id,
            world
        };
        let c = T::construct(&mut context, self.0);
    }
}

impl ConstructExt for Commands<'_, '_> {
    fn construct<T: Construct + Component>(&mut self, props: T::Props) -> &mut Self where <T as Construct>::Props: Send {
        self.spawn_empty()
            .add(ConstructCommand::<T>(props));
        self
    }
}

impl ConstructExt for bevy::ecs::system::EntityCommands<'_> {
    fn construct<T: Construct + Component>(&mut self, props: T::Props) -> &mut Self where <T as Construct>::Props: Send {
        self.add(ConstructCommand::<T>(props));
        self
    }
}

// impl<T: Default + Clone> Construct for T {
//     type Props = T;
//     #[inline]
//     fn construct(context: &mut ConstructContext, props: Self::Props) -> Result<Self, ConstructError> {
//         Ok(props)
//     }
// }
