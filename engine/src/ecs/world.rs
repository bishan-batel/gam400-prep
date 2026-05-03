#[derive(Debug)]
pub struct World {}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        Self {}
    }
}
