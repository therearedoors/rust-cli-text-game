use crate::globals::{Direction};
use crate::things::{room::Room,thing::{Thing,Thingsein, ThingHolder},treasure::Treasure};

#[derive(Clone)]
pub struct Actor {
    thing:Thing,
    location: Room,
    things: Vec<Treasure>,
}

impl Thingsein for Actor {
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

impl ThingHolder<Treasure> for Actor {
    fn get_things(&self) -> Vec<Treasure> {
        self.things.clone()
    }
    fn set_things(&mut self, things: Vec<Treasure>) {
        self.things = things;
    }
}

impl Actor {
    pub fn new(name: String, description: String, location: Room) -> Actor {
        Actor {thing: Thing::new(name,description), location, things: vec![]}
    }

    pub fn get_location(&self) -> Room {
        self.location.clone()
    }

    pub fn set_location(&mut self, location: Room) {
        self.location = location;
    }

    pub fn move_to(&mut self, map: Vec<Room>, r: Room, dir: Direction) -> i32 {
        let exit;
        match dir {
            Direction::North => {
                exit = r.get_n();
            },
            Direction::South => {
                exit = r.get_s();
            },
            Direction::East => {
                exit = r.get_e();
            },
            Direction::West => {
                exit = r.get_w();
            },
            _ => {
                exit = Direction::NOEXIT as i32;
            },
        }
        if exit != Direction::NOEXIT as i32 {
            self.set_location(map.get(exit as usize).unwrap().clone());
        }
        exit
        }
}