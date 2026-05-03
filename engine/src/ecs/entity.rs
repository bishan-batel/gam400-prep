#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity(u16);

impl Entity {
    #[must_use]
    pub fn from_index(idx: u16) -> Self {
        Self(idx)
    }

    #[must_use]
    pub fn index(&self) -> u16 {
        self.0
    }
}
