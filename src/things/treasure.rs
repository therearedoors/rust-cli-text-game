use crate::things::{thing::{Thing, Thingsein}};

#[derive(Clone)]
pub struct Treasure {
    thing: Thing,
    value: usize,
}

impl Thingsein for Treasure {
    fn get_name(&self) -> String {
        self.thing.get_name()
    }
    fn set_name(&mut self, name: String) {
        self.thing.set_name(name);
    }
    fn get_description(&self) -> String {
        self.thing.get_description()
    }
    fn set_description(&mut self, description: String) {
        self.thing.set_description(description);
    }
}

impl Treasure {
    pub fn new(name: String, description: String, value: usize) -> Treasure {
        Treasure {thing: Thing::new(name, description), value}
    }

    pub fn get_value(&self) -> usize {
        self.value
    }
}