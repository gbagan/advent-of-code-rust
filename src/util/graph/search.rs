use std::collections::HashSet;
use std::hash::Hash;
use num_traits::Zero;
use crate::util::heap::MinHeap;

pub fn dijkstra<V, C, FN, IN, FT>(
    start: &V,
    neighbors: FN,
    is_target: FT,
) -> Option<C>
where
    V: Eq + Hash + Clone,
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