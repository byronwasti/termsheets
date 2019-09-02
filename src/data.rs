use crate::parse::{parse, LangError};
use crate::viewer::Item;
use log::debug;
use std::collections::HashMap;
use crate::position::CellPos;

pub struct Data {
    cell_data: HashMap<CellPos, String>,
    calculated: HashMap<CellPos, String>,
    dag: (),
}

impl Default for Data {
    fn default() -> Self {
        Self {
            cell_data: HashMap::new(),
            calculated: HashMap::new(),
            dag: (),
        }
    }
}

impl Data {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, location: CellPos, value: String) {
        if value.starts_with("=") {
             let val = parse(&value, &self);
             match val {
                 Ok(val) => {
                     self.calculated.insert(location, val.to_string());
                 },
                 Err(e) => {
                     self.calculated.insert(location, e.to_string());
                 }
             }
        }
        self.cell_data.insert(location, value);
    }

    pub fn get(&self, location: CellPos) -> Option<&String> {
        if let Some(val) = self.calculated.get(&location) {
            Some(val)
        } else {
            self.cell_data.get(&location)
        }
    }

    fn update_dag(&mut self, cell: CellPos, dependencies: &[CellPos]) {
        for pos in dependencies {
            /*
            if let Some(vec) = self.dag.get_mut(&pos) {
                vec.push(cell);
            } else {
                self.dag.insert(*pos, vec![cell]);
            }
            */
        }
    }
}

