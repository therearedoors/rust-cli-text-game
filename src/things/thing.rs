#[derive(Clone)]
pub struct Thing {
    name: String,
    description: String,
}

pub trait Thingsein {
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn get_description(&self) -> String;
    fn set_description(&mut self, description: String);
}

impl Thing {
    pub fn new(name: String, description: String) -> Thing {
        Thing {name: name, description: description}
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}

pub trait ThingHolder<T> {
    fn get_things(&self) -> Vec<T>;

    fn set_things(&mut self, things: Vec<T>);
}