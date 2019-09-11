use crate::position::CellPos;
use log::debug;
use std::collections::{HashMap, HashSet, VecDeque};

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
            return;
        }

        let l = self.adjacency_list.get(&pos).unwrap();
        let l = (*l).clone();

        for neighbor in l {
            match neighbor {
                Neighbor::Incoming(n) => {
                    self.adjacency_list
                        .get_mut(&n)
                        .unwrap()
                        .remove(&Neighbor::Outgoing(pos));
                }
                Neighbor::Outgoing(n) => {
                    self.adjacency_list
                        .get_mut(&n)
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
                    _ => None,
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_topological_sort(&self, pos: CellPos) -> Result<Vec<CellPos>, ()> {
        let mut temporary = HashSet::new();
        let mut permanent = HashSet::new();
        let mut order = Vec::new();
        let mut stack = VecDeque::new();
        stack.push_back((pos, 0));

        while stack.len() > 0 {
            let (n, pass) = stack.pop_back().unwrap();
            if pass == 0 {
                if permanent.get(&n).is_some() {
                    continue;
                }
                if temporary.get(&n).is_some() {
                    return Err(());
                }
                temporary.insert(pos);
                stack.push_back((n, 1));
                for m in self.get_dependents(n) {
                    stack.push_back((m, 0));
                }
            } else {
                permanent.insert(n);
                order.push(n);
            }
        }

        order.reverse();
        Ok(order)
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

        g.insert(p2, &[p1]);
        let dep = g.get_dependents(p1);
        assert_eq!(dep, vec![p2]);

        g.remove(p2);
        let dep = g.get_dependents(p1);
        assert_eq!(dep, vec![]);
    }

    #[test]
    fn test_graph_topological() {
        let mut g = Dag::new();
        let p1 = CellPos::new(0, 0);
        let p2 = CellPos::new(1, 2);
        let p3 = CellPos::new(1, 3);

        g.insert(p2, &[p1]);
        g.insert(p3, &[p1, p2]);
        let dep = g.get_topological_sort(p1).unwrap();
        assert_eq!(dep, vec![p1, p2, p3]);
    }
}
