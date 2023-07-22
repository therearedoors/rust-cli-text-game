pub trait Thing {
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn get_description(&self) -> String;
    fn set_description(&mut self, description: String);
}

// pub trait ThingHolder {
//     fn get_things(&self) -> Vec<Box<dyn Thing>>;
//     fn set_things(&mut self, things: Vec<Box<dyn Thing>>);
// }

macro_rules! impl_thing {
    ($ty: ty) => {
        impl Thing for $ty {
            fn get_name(&self) -> String {
                return self.name.clone();
            }
        
            fn set_name(&mut self, name: String) {
                self.name = name;
            }
        
            fn get_description(&self) -> String {
                return self.description.clone();
            }
        
            fn set_description(&mut self, description: String) {
                self.description = description;
            }
        }
    }
}

// macro_rules! impl_thing_holder {
//     ($ty: ty) => {
//         impl ThingHolder for $ty {
//             fn get_things(&self) -> Vec<Box<dyn Thing>> {
//                 return self.things.clone();
//             }

//             fn set_things(&mut self, things: Vec<Box<dyn Thing>>) {
//                 self.things = things;
//             }
//         }
//     }
// }

pub(crate) use impl_thing;
//pub(crate) use impl_thing_holder;
