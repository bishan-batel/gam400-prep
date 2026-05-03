pub trait Component {}

pub(crate) trait ComponentVec {
    fn push_none(&mut self);
}

impl<T> ComponentVec for Vec<Option<T>> {
    fn push_none(&mut self) {
        self.push(None);
    }
}
