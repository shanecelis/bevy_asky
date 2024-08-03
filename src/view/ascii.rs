use crate::construct::*;
use crate::{
    prompt::{Confirm, ConfirmState, Input, InputState, Prompt},
    AskyState,
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, confirm_view);
    app.add_systems(Update, text_view);
}

pub(crate) fn confirm_view(
    mut query: Query<
        (&AskyState, &ConfirmState, &mut Text, &Confirm),
        (
            With<View>,
            Or<(Changed<AskyState>, Changed<ConfirmState>)>,
        ),
    >,
) {
    for (state, confirm_state, mut text, confirm) in query.iter_mut() {
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
        (&AskyState, &InputState, &mut Text, Option<&Prompt>),
        (
            With<View>,
            Or<(Changed<AskyState>, Changed<InputState>)>,
        ),
    >,
) {
    for (state, text_state, mut text, prompt_maybe) in query.iter_mut() {
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
                if let Some(ref prompt) = prompt_maybe {
                    text.sections[1].value.replace_range(..,
                                                         &prompt.0);
                }
                text.sections[2].value.replace_range(
                    ..,
                    &text_state.value
                );
            }
        }
    }
}

#[derive(Component)]
pub struct View;

// impl Construct for View<Confirm> {
//     type Props = <Confirm as Construct>::Props;

//     fn construct(
//         context: &mut ConstructContext,
//         props: Self::Props,
//     ) -> Result<Self, ConstructError> {
//         // Our requirements.
//         let confirm: Confirm = context.construct(props)?;
//         let mut commands = context.world.commands();
//         commands.entity(context.id).insert(TextBundle {
//             text: Text::from_sections([
//                 "[_] ".into(),                      // 0
//                 confirm.message.to_string().into(), // 1
//                 " ".into(),                         // 2
//                 "".into(),                          // 3
//             ]),
//             ..default()
//         });
//         context.world.flush();

//         Ok(View(confirm))
//     }
// }

impl Construct for View {
    type Props = ();

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // Our requirements.
        // let text_input: Input = context.construct(props)?;
        let mut commands = context.world.commands();
        commands.entity(context.id).insert(TextBundle {
            text: Text::from_sections([
                "[_] ".into(),                      // 0
                "".into(), //text_input.message.to_string().into(), // 1
                "".into(),                          // 2
            ]),
            ..default()
        });
        context.world.flush();

        dbg!(context);
        Ok(View)
    }
}
