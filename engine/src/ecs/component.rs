use std::{any::Any, fmt::Debug};

pub trait Component: Debug + Any {}

impl<T: Debug + Any> Component for T {}

pub(crate) trait ComponentVec {
    fn push_none(&mut self);
}

impl<T> ComponentVec for Vec<Option<T>> {
    fn push_none(&mut self) {
        self.push(None);
    }
}
