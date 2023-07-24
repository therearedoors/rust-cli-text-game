use crate::things::info::Info;

#[derive(Clone,Debug)]
pub struct Treasure {
    info: Info,
    value: usize,
}

impl Treasure {
    pub fn new(info: Info, value: usize) -> Treasure {
        Treasure {info, value}
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn get_name(&self) -> &String {
        &self.info.name
    }

    pub fn get_description(&self) -> &String {
        &self.info.description
    }
}