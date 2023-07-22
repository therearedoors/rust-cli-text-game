use crate::things::thing::{Thing, /*ThingHolder,*/ impl_thing};

pub struct Room {
    name: String,
    description: String,
    //things: Vec<Box<dyn Thing>>,
    n: i32,
    s: i32,
    e: i32,
    w: i32,
}

impl_thing!(Room);
//impl_thing_holder!(Room);

impl Room {
    pub fn new(name: String, description: String, things:Vec<Box<dyn Thing>>, n: i32, s: i32, e: i32, w: i32) -> Room {
        return Room {name, description, /*things,*/ n, s, e, w}
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
    + "\nThings here:\n" /*+ self.describe_treasures().as_str()*/)
    }

    // fn describe_treasures(&self) -> String {
    //     self.get_things()
    //         .into_iter()
    //         .map(|t| format!("{}: {}", t.get_name(), t.get_description()))
    //         .collect::<Vec<String>>()
    //         .join("\n")
    // }
}