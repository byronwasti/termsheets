use crate::graph::Dag;
use crate::parse::{parse, LangError};
use crate::position::CellPos;
use crate::viewer::Item;
use log::debug;
use std::collections::HashMap;

pub struct Data {
    cell_data: HashMap<CellPos, String>,
    calculated: HashMap<CellPos, String>,
    dag: Dag,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            cell_data: HashMap::new(),
            calculated: HashMap::new(),
            dag: Dag::new(),
        }
    }
}

impl Data {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, location: CellPos, value: String) {
        if value.starts_with("=") {
            self.dag.remove(location);
            let out = parse(&value, &self);
            match out {
                Ok((val, deps)) => {
                    self.calculated.insert(location, val.to_string());
                    self.dag.insert(location, &deps);
                }
                Err(e) => {
                    self.calculated.insert(location, e.to_string());
                }
            }
        }

        self.cell_data.insert(location, value);
        self.update_using_dag(location);
    }

    pub fn get(&self, location: CellPos) -> Option<&String> {
        if let Some(val) = self.calculated.get(&location) {
            Some(val)
        } else {
            self.cell_data.get(&location)
        }
    }

    fn update_using_dag(&mut self, cell: CellPos) {
        let traversal = self.dag.get_traversal(cell).unwrap();
        debug!("Traversal of length {}", traversal.len());
        for dep in traversal {
            let val = self.cell_data.get(&dep).unwrap();
            let out = parse(&val, &self);
            match out {
                Ok((val, _)) => {
                    self.calculated.insert(dep, val.to_string());
                }
                Err(e) => {
                    self.calculated.insert(dep, e.to_string());
                }
            }
        }
    }
}
