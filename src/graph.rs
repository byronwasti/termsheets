use crate::position::CellPos;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Neighbor {
    Incoming(CellPos),
    Outgoing(CellPos),
}

#[derive(Debug)]
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

        self.adjacency_list.remove(&pos);
    }

    pub fn get_dependents(&self, pos: CellPos) -> Vec<CellPos> {
        if let Some(l) = self.adjacency_list.get(&pos) {
            l.iter()
                .filter_map(|x| match x {
                    Neighbor::Outgoing(v) => Some(*v),
                    _ => None 
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let mut g = Dag::new();
        let p1 = CellPos::new(0, 0);
        let p2 = CellPos::new(1, 2);

        g.insert(p1, &[p2]);
        let dep = g.get_dependents(p2);
        assert_eq!(dep, vec![p1]);

        g.remove(p1);
        let dep = g.get_dependents(p2);
        assert_eq!(dep, vec![]);
    }
}
