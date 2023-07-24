use crate::things::{info::Info,treasure::Treasure};

#[derive(Clone,Debug)]
pub struct Room {
    info: Info,
    treasures: Box<Vec<Treasure>>,
    n: i32,
    s: i32,
    e: i32,
    w: i32,
}

impl Room {
    pub fn new(info: Info, treasures: Box<Vec<Treasure>>, n: i32, s: i32, e: i32, w: i32) -> Room {
        return Room {info, treasures, n, s, e, w}
    }

    pub fn get_name(&self) -> &String {
        &self.info.name
    }

    pub fn get_description(&self) -> &String {
        &self.info.description
    }

    pub fn get_treasures(&self) -> Box<Vec<Treasure>> {
        self.treasures.clone()
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
       format!("{}. {}.", self.get_name(), self.get_description())
    + "\nThings here:\n" + self.describe_treasures().as_str()
    }

    fn describe_treasures(&self) -> String {
        self.get_treasures()
            .into_iter()
            .map(|t| format!("{}: {}", t.get_name(), t.get_description()))
            .collect::<Vec<String>>()
            .join("\n")
    }
}