use std::collections::HashMap;
use crate::viewer::Item;
use crate::parse::{parse, Operation};
use log::debug;

type Coord = (usize, usize);

pub struct Data {
    cell_data: HashMap<Coord, String>,
    calculated: HashMap<Coord, String>,
    dag: HashMap<Coord, Vec<Coord>>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            cell_data: HashMap::new(),
            calculated: HashMap::new(),
            dag: HashMap::new(),
        }
    }
}

impl Data {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, location: (usize, usize), value: String) {
        if value.starts_with("=") {
            if let Ok((c1, c2, op)) = parse(&value) {
                debug!("{:?} {:?} {:?}", &c1, &op, &c2);
                let c1 = self.get(c1);
                let c2 = self.get(c2);

                if let (Some(c1), Some(c2)) = (c1, c2) {
                    if let (Ok(c1), Ok(c2)) = (c1.parse::<i32>(), c2.parse::<i32>()) {
                        let val = match op {
                            Operation::Mul => c1 * c2,
                            Operation::Add => c1 + c2,
                            Operation::Sub => c1 - c2,
                            Operation::Div => c1 / c2,
                        };
                        self.calculated.insert(location, val.to_string());
                    } else {
                        self.calculated.insert(location, "#CELL_ERR".to_string());
                    }
                } else {
                    self.calculated.insert(location, "#REF_ERR".to_string());
                }
            } else {
                self.calculated.insert(location, "#PARSE_ERR".to_string());
            }
        }

        self.cell_data.insert(location, value);
    }

    pub fn get(&self, location: (usize, usize)) -> Option<&String> {
        let val = self.calculated.get(&location);
        if val.is_some() {
            val
        } else {
            self.cell_data.get(&location)
        }
    }
}
