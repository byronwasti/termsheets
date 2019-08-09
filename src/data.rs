use std::collections::HashMap;
use crate::viewer::Item;

pub struct Data {
    cell_data: HashMap<(usize, usize), String>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            cell_data: HashMap::new(),
        }
    }
}

impl Data {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, location: (usize, usize), value: String) {
        self.cell_data.insert(location, value);
    }

    pub fn get(&self, location: (usize, usize)) -> Option<&String> {
        self.cell_data.get(&location)
    }
}
