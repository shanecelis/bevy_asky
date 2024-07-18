use bevy::prelude::*;
use std::borrow::Cow;
pub mod view;

pub struct AskyPlugin;

/// AskySet defines when the input events are emitted.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AskySet {
    /// Run before any input events are emitted.
    Pre,
    /// Process the input.
    ProcessInput,
    /// Render views if necessary.
    ConstructView,
    /// Run after all input events are emitted.
    Post,
}

impl Plugin for AskyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, confirm_controller)
            // .configure_sets(
            //     Update,
            //     (
            //         AskySet::Pre,
            //         AskySet::ProcessInput,
            //         AskySet::ConstructView,
            //         AskySet::Post,
            //     )
            //         .chain(),
            // );
            ;
    }
}

fn confirm_controller(
    mut query: Query<(Entity, &mut AskyState, &Confirm, Option<&mut ConfirmState>)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (id, mut state, confirm, mut confirm_state) in query.iter_mut() {
        match *state {
            AskyState::Uninit => {
                if confirm_state.is_none() {
                    commands
                        .entity(id)
                        .insert(ConfirmState { yes: confirm.init });
                }
                *state = AskyState::Reading;
            }
            AskyState::Reading => {
                if let Some(ref mut confirm_state) = confirm_state {
                    if input.any_just_pressed([KeyCode::KeyY, KeyCode::KeyN, KeyCode::Enter]) {
                        if input.just_pressed(KeyCode::KeyY) {
                            confirm_state.yes = Some(true);
                        }
                        if input.just_pressed(KeyCode::KeyN) {
                            confirm_state.yes = Some(false);
                        }

                        if input.just_pressed(KeyCode::Enter) && confirm_state.yes.is_some() {
                            commands.trigger_targets(AskyEvent(Ok(confirm_state.yes.unwrap())), id);
                            *state = AskyState::Complete;
                        }
                    }
                } else {
                    panic!("cannot get start while reading.");
                }
            }
            _ => (),
        }
    }
}

#[derive(Event, Deref, Debug)]
pub struct AskyEvent<T>(Result<T, Error>);

#[derive(Component)]
pub struct Confirm {
    /// Message used to display in the prompt.
    pub message: Cow<'static, str>,
    /// Initial confirm_state of the prompt.
    pub init: Option<bool>,
}

#[derive(Debug, Component, Default)]
pub enum AskyState {
    Frozen,
    #[default]
    Uninit,
    Reading,
    Complete,
    Error,
}

#[derive(Component)]
struct ConfirmState {
    pub yes: Option<bool>,
}

/// Asky errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// User cancelled.
    #[error("cancelled")]
    Cancel,
    /// Input was invalid.
    #[error("invalid input")]
    InvalidInput,
    /// Invalid count with expected and actual.
    #[error("invalid count, expected {expected} actual {actual}")]
    InvalidCount {
        /// Expected count
        expected: usize,
        /// Actual count
        actual: usize,
    },
    /// Validation failed.
    #[error("validation fail")]
    ValidationFail,
    /// Message
    #[error("{0}")]
    Message(Cow<'static, str>),
    /// There was an [std::io::Error].
    #[error("io error {0}")]
    Io(#[from] std::io::Error),
    #[cfg(feature = "bevy")]
    /// Async error
    // #[error("async error {0}")]
    // Async(#[from] bevy_defer::AccessError),
    /// Promise error
    #[error("promise error {0}")]
    Promise(#[from] promise_out::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
