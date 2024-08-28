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
    queue.push(Zero::zero(), *start);

    while let Some((cost, node)) = queue.pop() {
        if is_target(&node) {
            return Some(cost)
        }
        if !visited.insert(node) {
            continue;
        }
        for (nbor, cost2) in neighbors(&node) {
            queue.push(cost + cost2, nbor)
        }      
    }
    None
}



pub fn postorder_dfs(graph: &[Vec<usize>], source: usize) -> Vec<usize> {
    let mut output = vec!();
    let mut visited = vec![false; graph.len()];
    postorder_dfs_aux(graph, &mut visited, &mut output, source); 
    output
}

fn postorder_dfs_aux(graph: &[Vec<usize>], visited: &mut Vec<bool>, output: &mut Vec<usize>, v: usize) {
    if visited[v] {
        return;
    }
    visited[v] = true;
    for &u in &graph[v] {
        postorder_dfs_aux(graph, visited, output, u);
    }
    output.push(v);
}

#[test]
fn postorder_dfs_test() {
    let graph = vec!(vec!(1, 2), vec!(3), vec!(3), vec!());
    assert_eq!(postorder_dfs(&graph, 0), vec!(3, 1, 2, 0));
}