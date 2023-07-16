use crate::things::{thing::{Thing, ThingHolder, impl_thing, impl_thing_holder}, room::Room};

pub struct Actor {
    name: String,
    description: String,
    location: isize,
    pub things: Vec<Box<dyn Thing>>
}

impl_thing!(Actor);
impl_thing_holder!(Actor);

impl Actor {
    pub fn new(name: String, description: String, location: isize) -> Actor {
        Actor {name, description, location, things: vec![]}
    }

    pub fn get_location(&self) -> isize {
        self.location
    }

    pub fn set_location(&mut self, location: isize) {
        self.location = location;
    }

    pub fn move_to(&mut self, map: &Vec<Room>, r: isize) -> i32 {
        let exit;
        // match dir {
        //     Direction::North => {
        //         exit = r.get_n();
        //     },
        //     Direction::South => {
        //         exit = r.get_s();
        //     },
        //     Direction::East => {
        //         exit = r.get_e();
        //     },
        //     Direction::West => {
        //         exit = r.get_w();
        //     },
        //     _ => {
        //         exit = Direction::NOEXIT as i32;
        //     },
        // }
        if r != -1 {
            self.set_location(exit as isize);
        } else {
            exit = -1;
        }
        exit
        }
}