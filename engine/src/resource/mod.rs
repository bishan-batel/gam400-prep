use std::ops::{Deref, DerefMut};

use env_logger::Target;

/// Globally unique data
pub trait Resource: Send + Sync + 'static {}

/// Immutable handle to a globally unique 'Resource'
#[derive(Debug, Clone, Copy)]
pub struct Res<'a, T: ?Sized + Resource> {
    pub(crate) value: &'a T,
}

impl<'a, T: Resource> Res<'a, T> {
    pub fn into_inner(self) -> &'a T {
        self.value
    }
}

impl<T: Resource> Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T: Resource> From<ResMut<'a, T>> for Res<'a, T> {
    fn from(value: ResMut<'a, T>) -> Self {
        Self { value: value.value }
    }
}

/// Mutable handle to a globally unique 'Resource'
#[derive(Debug)]
pub struct ResMut<'a, T: ?Sized + Resource> {
    value: &'a mut T,
}

impl<'a, T: Resource> ResMut<'a, T> {
    pub fn into_inner(self) -> &'a mut T {
        self.value
    }
}

impl<T: Resource> Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<T: Resource> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}
