use bevy::prelude::*;
use crate::{AskyState, Confirm, ConfirmState};

pub struct AsciiViewPlugin;

impl Plugin for AsciiViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, confirm_view);
    }

}
fn confirm_view(mut query: Query<(Entity, &mut AskyState, &Confirm, &ConfirmState), Or<(Changed<AskyState>, Changed<ConfirmState>)>>,
                mut text: Query<&mut Text>,
                mut commands: Commands) {
    for (id, mut state, confirm, confirm_state) in query.iter_mut() {
        match *state {
            AskyState::Frozen | AskyState::Uninit => (),
            ref asky_state => {
                eprint!(".");
                let text = format!("[{}] {} {}",
                                   match asky_state {
                                       AskyState::Reading => " ",
                                       AskyState::Complete => "x",
                                       AskyState::Error => "!",
                                       _ => unreachable!()
                                   },
                                   confirm.message.as_ref(),
                                   if matches!(asky_state, AskyState::Complete) {
                                       match confirm_state.yes {
                                           Some(true) => "Yes",
                                           Some(false) => "No",
                                           None => unreachable!(),
                                       }
                                   } else {

                                       match confirm_state.yes {
                                           Some(true) => "Y/n",
                                           Some(false) => "y/N",
                                           None => "y/n"
                                       }
                                   });
                let new_child = commands.spawn(TextBundle::from(text)).id();
                commands.entity(id)
                        .despawn_descendants()
                        .replace_children(&[new_child]);
            }
        }
    }
}
