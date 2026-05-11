use super::Component;
use ecs_custom_macros::Component;

#[derive(Component, Debug)]
pub struct Name(pub String);

impl From<String> for Name {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
