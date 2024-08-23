use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use num_traits::Zero;
use crate::util::heap::MinHeap;

pub fn bfs<V, F, IN>(start: V, neighbors: F) -> BfsIterator<V, F> 
    where
    V: Eq + Hash + Copy,
    F: Fn(&V) -> IN,
    IN: IntoIterator<Item = V>,
{
    let visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    BfsIterator { queue, neighbors, visited }
}

pub struct BfsIterator<V, F> 
{
    queue: VecDeque<(V, usize)>,
    visited: HashSet<V>,
    neighbors: F,
}

impl<V, F, IN> Iterator for BfsIterator<V, F>
    where
    V: Eq + Hash + Copy,
    F: Fn(&V) -> IN,
    IN: IntoIterator<Item = V>,
{
    type Item = (V, usize);

    fn next(&mut self) -> Option<(V, usize)> {
        while let Some((v, dist)) = self.queue.pop_front() {
            if !self.visited.insert(v) {
                continue;
            }
            for u in (self.neighbors)(&v) {
                self.queue.push_back((u, dist+1));
            }
            return Some((v, dist));
        }
        None
    }
}

pub fn dijkstra<V, C, FN, IN, FT>(
    start: &V,
    neighbors: FN,
    is_target: FT,
) -> Option<C>
where
    V: Eq + Hash + Copy,
    C: Zero + Ord + Copy,
    FN: Fn(&V) -> IN,
    IN: IntoIterator<Item = (V, C)>,
    FT: Fn(&V) -> bool
{
    let mut queue = MinHeap::new();
    let mut visited = HashSet::new();
    queue.push(Zero::zero(), start.clone());

    while let Some((cost, node)) = queue.pop() {
        if is_target(&node) {
            return Some(cost)
        }
        if !visited.insert(node.clone()) {
            continue;
        }
        for (nbor, cost2) in neighbors(&node) {
            queue.push(cost + cost2, nbor)
        }      
    }
    None
}