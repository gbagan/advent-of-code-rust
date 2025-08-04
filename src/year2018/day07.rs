use std::collections::BinaryHeap;
use arrayvec::*;

type Graph = [ArrayVec<usize, 25>; 26];

pub fn solve(input: &str) -> (String, u32) {
    let input = input.as_bytes();
    
    let mut graph: Graph  = std::array::from_fn(|_| ArrayVec::new());
    let mut in_degree = [0u32; 26];
    let mut is_node = [false; 26];

    for line in input.array_chunks::<49>() {
        let node1 = (line[5] - b'A') as usize;
        let node2 = (line[36] - b'A') as usize;
        graph[node1].push(node2);
        in_degree[node2] += 1;
        is_node[node1] = true;
        is_node[node2] = true;
    }

    let order = topological_ordering(&graph, in_degree, &is_node);

    let p1: String = order
        .iter()
        .filter(|&&i| is_node[i])
        .map(|&i| (i as u8 + b'A') as char)
        .collect();

    let p2 = part2(&graph, in_degree, &is_node, 5);
    (p1, p2)
}

pub fn topological_ordering(graph: &Graph, mut in_degree: [u32; 26], is_node: &[bool; 26]) -> ArrayVec<usize, 26> {
    let mut heap = BinaryHeap::new();
    let mut order: ArrayVec<usize, 26> = ArrayVec::new();
    for i in 0..26 {
        if is_node[i] && in_degree[i] == 0 {
            heap.push(26-i);
        }
    }

    while let Some(v) = heap.pop() {
        let v = 26-v;
        order.push(v);
        for &u in &graph[v] {
            in_degree[u] -= 1;
            if in_degree[u] == 0 {
                heap.push(26 - u);
            }
        }
    }

   order
}

pub fn part2(graph: &Graph, mut in_degree: [u32; 26], is_node: &[bool; 26], nb_workers: usize) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut workers: ArrayVec<(usize, u32), 5> = ArrayVec::new();

    for i in 0..26 {
        if is_node[i] && in_degree[i] == 0 {
            heap.push(26-i);
        }
    }

    let mut time = 0;

    loop {
        if workers.len() < nb_workers && let Some(task) = heap.pop() {
            let task = 26-task;
            workers.push((task, duration(task)));
        } else if workers.is_empty() {
            return time;
        } else {
            let min_worker = workers.iter().map(|p| p.1).min().unwrap();
            let mut i = 0;
            while i < workers.len() {
                workers[i].1 -= min_worker;
                if workers[i].1 == 0 {
                    let task = workers[i].0;
                    for &next in &graph[task] {
                        in_degree[next] -= 1;
                        if in_degree[next] == 0 {
                            heap.push(26 - next);
                        }
                    }
                    workers.swap_remove(i);
                } else {
                    i += 1;
                }
            }
            time += min_worker;
        }

    }
}

fn duration(task: usize) -> u32 {
    61 + task as u32
}