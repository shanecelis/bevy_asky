use bevy::prelude::*;
use std::borrow::Cow;
use crate::construct::*;
use std::fmt;

mod confirm;
mod text;
mod number;
pub use confirm::*;
pub use text::*;
pub use number::*;

#[derive(Component, Deref, DerefMut)]
pub struct Prompt(pub Cow<'static, str>);
#[derive(Component, Deref, DerefMut)]
pub struct Placeholder(pub Cow<'static, str>);
#[derive(Component)]
// pub struct DefaultValue<T: std::fmt::Display>(pub T);
pub struct DefaultValue<T>(pub T);

impl Construct for Placeholder {
    type Props = Cow<'static, str>;
    fn construct(
        _context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        Ok(Placeholder(props.into()))
    }
}

impl<T: std::fmt::Display + Clone> Construct for DefaultValue<T> {
    type Props = T;
    fn construct(
        _context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        Ok(DefaultValue(props))
    }
}

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

