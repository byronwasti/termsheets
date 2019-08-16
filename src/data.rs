use crate::parse::{parse, Operation};
use crate::viewer::Item;
use log::debug;
use std::collections::HashMap;
use crate::position::CellPos;

pub struct Data {
    cell_data: HashMap<CellPos, String>,
    calculated: HashMap<CellPos, String>,
    dag: HashMap<CellPos, Vec<CellPos>>,
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

    pub fn insert(&mut self, location: CellPos, value: String) {
        if value.starts_with("=") {
            if let Ok((c1, c2, op)) = parse(&value) {
                debug!("{:?} {:?} {:?}", &c1, &op, &c2);
                let c1 = CellPos::new(c1.0, c1.1);
                let c2 = CellPos::new(c2.0, c2.1);
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

    pub fn get(&self, location: CellPos) -> Option<&String> {
        let val = self.calculated.get(&location);
        if val.is_some() {
            val
        } else {
            self.cell_data.get(&location)
        }
    }
}
