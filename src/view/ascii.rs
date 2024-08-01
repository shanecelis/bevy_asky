use crate::construct::*;
use crate::{
    prompt::{Confirm, ConfirmState},
    AskyState,
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, confirm_view);
}

pub(crate) fn confirm_view(
    mut query: Query<
        (&AskyState, &ConfirmState, &mut Text),
        (
            With<View<Confirm>>,
            Or<(Changed<AskyState>, Changed<ConfirmState>)>,
        ),
    >,
) {
    for (mut state, confirm_state, mut text) in query.iter_mut() {
        match *state {
            AskyState::Frozen | AskyState::Uninit => (),
            ref asky_state => {
                eprint!(".");
                text.sections[0].value.replace_range(
                    1..=1,
                    match asky_state {
                        AskyState::Reading => " ",
                        AskyState::Complete => "x",
                        AskyState::Error => "!",
                        _ => unreachable!(),
                    },
                );
                text.sections[3].value.replace_range(
                    ..,
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
                            None => "y/n",
                        }
                    },
                );
            }
        }
    }
}

#[derive(Component)]
pub struct View<T>(pub T);

impl Construct for View<Confirm> {
    type Props = <Confirm as Construct>::Props;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let confirm: Confirm = context.construct(props)?;
        let mut commands = context.world.commands();
        commands.entity(context.id).insert(TextBundle {
            text: Text::from_sections([
                "[_] ".into(),                      // 0
                confirm.message.to_string().into(), // 1
                " ".into(),                         // 2
                "".into(),                          // 3
            ]),
            ..default()
        });
        context.world.flush();

        Ok(View(confirm))
    }
}
