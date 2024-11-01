use crate::construct::*;
use bevy::prelude::*;
use std::borrow::Cow;
use std::fmt;

mod checkbox;
mod confirm;
mod number;
mod password;
mod radio;
mod text;
mod toggle;
pub use checkbox::*;
pub use confirm::*;
pub use number::*;
pub use password::*;
pub use radio::*;
pub use text::*;
pub use toggle::*;

#[derive(Component, Deref, DerefMut)]
pub struct Prompt(pub Cow<'static, str>);
#[derive(Component, Deref, DerefMut)]
pub struct Placeholder(pub Cow<'static, str>);
#[derive(Component)]
// pub struct DefaultValue<T: std::fmt::Display>(pub T);
pub struct DefaultValue<T>(pub T);

impl Construct for Prompt {
    type Props = Cow<'static, str>;
    fn construct(
        _context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        Ok(Prompt(props))
    }
}

impl Construct for Placeholder {
    type Props = Cow<'static, str>;
    fn construct(
        _context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        Ok(Placeholder(props))
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

impl Construct for Feedback {
    type Props = Feedback;
    fn construct(
        _context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        Ok(props)
    }
}


#[derive(Component, Clone)]
pub struct Feedback {
    pub kind: FeedbackKind,
    pub message: Cow<'static, str>,
}

impl Feedback {
    pub fn clear(&mut self) {
        self.kind = FeedbackKind::None;
        self.message = "".into();
    }

    pub fn info(message: impl Into<Cow<'static, str>>) -> Self {
        Feedback {
            kind: FeedbackKind::Info,
            message: message.into(),
        }
    }

    pub fn warn(message: impl Into<Cow<'static, str>>) -> Self {
        Feedback {
            kind: FeedbackKind::Warn,
            message: message.into(),
        }
    }

    pub fn error(message: impl Into<Cow<'static, str>>) -> Self {
        Feedback {
            kind: FeedbackKind::Error,
            message: message.into(),
        }
    }
}
impl fmt::Display for Feedback {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            FeedbackKind::None => Ok(()),
            FeedbackKind::Info => write!(f, "{}", self.message),
            kind => write!(f, "{}: {}", kind, self.message),
        }
    }
}

impl fmt::Display for FeedbackKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FeedbackKind::Info => "info",
                FeedbackKind::Warn => "warn",
                FeedbackKind::Error => "error",
                FeedbackKind::None => "NONE",
            }
        )
    }
}

#[derive(Clone)]
pub enum FeedbackKind {
    None,
    Info,
    Warn,
    Error,
}

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(confirm::plugin);
    app.add_plugins(text::plugin);
    app.add_plugins(number::plugin);
    app.add_plugins(password::plugin);
    app.add_plugins(toggle::plugin);
    app.add_plugins(checkbox::plugin);
    app.add_plugins(radio::plugin);
}
