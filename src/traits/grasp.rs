use crate::things::treasure::Treasure;

pub trait Grasp {
    fn grasp(&self, str: String) -> Option<Treasure>;
}

impl Grasp for Vec<Treasure> {
    fn grasp(&self, str: String) -> Option<Treasure> {
        for item in self {
            if item.get_name() == &str {
                return Some(item.clone());
            }
        }
        None
    }
}