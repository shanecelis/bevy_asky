use crate::construct::*;
use crate::{
    prompt::{Confirm, ConfirmState, Input, InputState},
    AskyState,
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, confirm_view);
    app.add_systems(Update, text_view);
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

pub(crate) fn text_view(
    mut query: Query<
        (&AskyState, &InputState, &mut Text),
        (
            With<View<Input>>,
            Or<(Changed<AskyState>, Changed<InputState>)>,
        ),
    >,
) {
    for (mut state, text_state, mut text) in query.iter_mut() {
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
                text.sections[2].value.replace_range(
                    ..,
                    &text_state.value
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

impl Construct for View<Input> {
    type Props = <Input as Construct>::Props;

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        let text_input: Input = context.construct(props)?;
        let mut commands = context.world.commands();
        commands.entity(context.id).insert(TextBundle {
            text: Text::from_sections([
                "[_] ".into(),                      // 0
                text_input.message.to_string().into(), // 1
                "".into(),                          // 2
            ]),
            ..default()
        });
        context.world.flush();

        Ok(View(text_input))
    }
}
