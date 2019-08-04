use std::collections::HashMap;

pub struct Item {
    pub x: usize,
    pub y: usize,
    pub data: String,
}

pub struct Data {
    data: Vec<Option<Item>>,
    dependencies: HashMap<usize, Vec<usize>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            dependencies: HashMap::new(),
        }
    }
}
