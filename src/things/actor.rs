use crate::things::{info::Info, room::Room, treasure::Treasure};
use crate::globals::Direction;

#[derive(Clone,Debug)]
pub struct Actor {
    info: Info,
    location: Box<Room>,
    treasures: Box<Vec<Treasure>>
}
//impl_thing_holder!(Actor);

impl Actor {
    pub fn new(info: Info, room: Room, treasure_vec: Vec<Treasure>) -> Actor {
        Actor {info, location: Box::new(room), treasures: Box::new(treasure_vec)}
    }

    pub fn get_name(&self) -> &String {
        &self.info.name
    }

    pub fn get_description(&self) -> &String {
        &self.info.description
    }

    pub fn get_location(&self) -> Box<Room> {
        self.location.clone()
    }

    pub fn set_location(&mut self, location: Room) {
        self.location = Box::new(location);
    }

    pub fn move_to(&mut self, map: Vec<Room>, direction: Direction) -> i32 {
        let exit: i32;
        match direction {
            Direction::North => {
                exit = self.location.get_n();
            },
            Direction::South => {
                exit = self.location.get_s();
            },
            Direction::East => {
                exit = self.location.get_e();
            },
            Direction::West => {
                exit = self.location.get_w();
            },
            _ => {
                exit = Direction::NOEXIT as i32;
            },
        }
        if exit != -1 {
            self.set_location(map[exit as usize].clone());
        }
        exit
        }
}