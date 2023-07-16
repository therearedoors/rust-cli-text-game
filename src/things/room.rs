use crate::things::{thing::{Thing,Thingsein,ThingHolder},treasure::Treasure};

#[derive(Clone)]
pub struct Room {
    pub thing: Thing,
    things: Vec<Treasure>,
    n: i32,
    s: i32,
    w: i32,
    e: i32,
}

impl Thingsein for Room {
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

impl Room {
    pub fn new(name: String, description: String, things:Vec<Treasure>, n: i32, s: i32, e: i32, w: i32) -> Room {
        Room {thing: Thing::new(name,description), things, n, s, e, w}
    }

    pub fn get_n(&self) -> i32 {
        self.n
    }
    
    pub fn get_s(&self) -> i32 {
        self.s
    }

    pub fn get_e(&self) -> i32 {
        self.e
    }

    pub fn get_w(&self) -> i32 {
        self.w
    }

    pub fn describe(&self) -> String {
       format!("{}. {}.", self.get_name(), self.get_description()
    + "\nThings here:\n" + self.describe_treasures().as_str())
    }

    fn describe_treasures(&self) -> String {
        self.get_things()
            .into_iter()
            .map(|t| format!("{}: {}", t.get_name(), t.get_description()))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl ThingHolder<Treasure> for Room {
    fn get_things(&self) -> Vec<Treasure> {
        self.things.clone()
    }
    fn set_things(&mut self, things: Vec<Treasure>) {
        self.things = things;
    }
}
