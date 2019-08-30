use crate::position::CellPos;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Neighbor {
    Incoming(CellPos),
    Outgoing(CellPos),
}

pub struct Dag {
    adjacency_list: HashMap<CellPos, HashSet<Neighbor>>,
}

impl Dag {
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn insert(&mut self, pos: CellPos, neighbors: &[CellPos]) {
        self.remove(pos);

        for neighbor in neighbors {
            if let Some(mut l) = self.adjacency_list.get_mut(neighbor) {
                l.insert(Neighbor::Outgoing(pos));
            } else {
                let mut hs = HashSet::new();
                hs.insert(Neighbor::Outgoing(pos));
                self.adjacency_list.insert(*neighbor, hs);
            }
        }
    
        let hs: HashSet<_> = neighbors.iter().map(|n| Neighbor::Incoming(*n)).collect();
        self.adjacency_list.insert(pos, hs);
    }

    pub fn remove(&mut self, pos: CellPos) {
        if self.adjacency_list.get(&pos).is_none() {
            return
        }

        let l = self.adjacency_list.get(&pos).unwrap();
        let l = (*l).clone();

        for neighbor in l {
            match neighbor {
                Neighbor::Incoming(n) => {
                    self.adjacency_list.get_mut(&n)
                        .unwrap()
                        .remove(&Neighbor::Outgoing(pos));
                }
                Neighbor::Outgoing(n) => {
                    self.adjacency_list.get_mut(&n)
                        .unwrap()
                        .remove(&Neighbor::Incoming(pos));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let g = Dag::new();
    }
}
