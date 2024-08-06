use bevy::{
    ecs::component::{StorageType, ComponentHooks},
    a11y::Focus,
    prelude::*
};
#[derive(Resource, Deref, DerefMut, Default, Debug)]
pub struct Foci(Vec<Entity>);

#[derive(Clone, Default)]
pub struct Focusable { version: usize }

pub fn plugin(app: &mut App) {
    app
        .init_resource::<Foci>()
        .add_systems(Update, focus_controller);
}

fn focus_controller(
    // mut query: Query<(&Visibility, &mut Radio, Option<&Parent>)>,
    input: Res<ButtonInput<KeyCode>>,
    mut focus_maybe: Option<ResMut<Focus>>,
    mut query: Query<&mut Focusable>,
    foci: Res<Foci>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::Tab) {
        if let Some(mut focus) = focus_maybe {
            // There is a focus resource.
            if let Some(focus_id) = focus.0 {
                if let Ok(mut focusable) = query.get_mut(focus_id) {
                    focusable.version += 1;
                }
                dbg!(focus_id);
                if let Some(index) = foci.iter().position(|&x| x == focus_id) {
                    focus.0 = foci.get(index + 1).or(foci.first()).cloned();
                }
            } else {
                focus.0 = foci.first().cloned();
            }
            if let Some(focus_id) = focus.0 {
                if let Ok(mut focusable) = query.get_mut(focus_id) {
                    focusable.version += 1;
                }
            }
        } else {
            commands.insert_resource(Focus(foci.first().cloned()));
        }
        // dbg!(foci);



    }
}


impl Component for Focusable {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, targeted_entity, _component_id| {
            let mut foci = world.get_resource_mut::<Foci>().expect("Foci resource");
            foci.push(targeted_entity);
        });
        hooks.on_remove(|mut world, targeted_entity, _component_id| {
            let mut foci = world.get_resource_mut::<Foci>().expect("Foci resource");
            if let Some(index) = foci.iter().position(|&x| x == targeted_entity) {
                foci.remove(index);
            }
        });
    }
}
