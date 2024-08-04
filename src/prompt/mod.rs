use bevy::prelude::*;
use std::borrow::Cow;
use std::fmt;

mod confirm;
mod text;
mod number;
pub use confirm::*;
pub use text::*;
pub use number::*;

#[derive(Component)]
pub struct Prompt(pub Cow<'static, str>);
#[derive(Component)]
pub struct Placeholder(pub Cow<'static, str>);
#[derive(Component)]
pub struct DefaultValue<T: std::fmt::Display>(pub T);
#[derive(Component)]
pub struct Feedback {
    pub kind: FeedbackKind,
    pub message: Cow<'static, str>,
}

impl Feedback {
    fn info(message: impl Into<Cow<'static, str>>) -> Self {
        Feedback {
            kind: FeedbackKind::Info,
            message: message.into()
        }
    }

    fn warn(message: impl Into<Cow<'static, str>>) -> Self {
        Feedback {
            kind: FeedbackKind::Warn,
            message: message.into()
        }
    }

    fn error(message: impl Into<Cow<'static, str>>) -> Self {
        Feedback {
            kind: FeedbackKind::Error,
            message: message.into()
        }
    }
}
impl fmt::Display for Feedback {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl fmt::Display for FeedbackKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            FeedbackKind::Info => "info",
            FeedbackKind::Warn => "warn",
            FeedbackKind::Error => "error",
        })
    }
}

pub enum FeedbackKind {
    Info,
    Warn,
    Error
}

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(confirm::plugin);
    app.add_plugins(text::plugin);
    app.add_plugins(number::plugin);
}

