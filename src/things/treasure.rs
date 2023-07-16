use crate::things::thing::{Thing, impl_thing};

#[derive(Clone)]
pub struct Treasure {
    name: String,
    description: String,
    value: usize,
}

impl_thing!(Treasure);

impl Treasure {
    pub fn new(name: String, description: String, value: usize) -> Treasure {
        Treasure {name, description, value}
    }

    pub fn get_value(&self) -> usize {
        self.value
    }
}