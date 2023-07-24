// use std::vec::Splice;
// use std::ops::RangeBounds;

#[derive(Clone,Debug)]
pub struct Info {
    pub name: String,
    pub description: String,
}

impl Info {
    fn new(name: String, description: String) -> Info {
        Info {name, description}
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_description(&self) -> &String {
        &self.description
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_description(&mut self, description: String) {
        self.description = description;
    }
}

// struct ThingHolder<T> {
//     things: Vec<T>,
// }

// impl<T, A> ThingHolder<T,A> {
//     fn new() -> ThingHolder {
//         ThingHolder {things: vec![]}
//     }

//     fn add_thing(&mut self, thing: T) {
//         self.things.push(thing);
//     }

//     fn remove_thing(&mut self, thing: T) {
//         self.things.remove(thing);
//     }

//     fn update_thing<R, I> (&mut self, range: R, thing: I) -> Splice<'_, <I as IntoIterator>::IntoIter, A>
//     where
//         R: RangeBounds<usize>,
//         I: IntoIterator<Item = T>, {
//         self.things.update(range, thing);
//     }

//     fn get_things(&self) -> &Vec<T> {
//         &self.things
//     }
// }