use std::{borrow::Cow, fmt::Display, ops::Deref};

/// String-Based name meant for IDs, this will limit copying
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StringName(Cow<'static, str>);

impl Display for StringName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for StringName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&'static str> for StringName {
    fn from(value: &'static str) -> Self {
        StringName(value.into())
    }
}

impl From<String> for StringName {
    fn from(value: String) -> Self {
        StringName(value.into())
    }
}

impl PartialEq<str> for StringName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
