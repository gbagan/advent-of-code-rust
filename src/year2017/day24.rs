use crate::util::parser::*;
use itertools::Itertools;

#[derive(Clone, Copy)]
struct Edge {
    id: u64,
    left: usize,
    right: usize,
    weight: usize,
    length: u32,
}

struct State {
    end: usize,
    edges: u64,
    length: u32,
    weight: usize,
}

pub fn solve(input: &str) -> (usize, usize) {
    let edges: Vec<_> = input
        .iter_unsigned()
        .tuples()
        .enumerate()
        .map(|(i, (left, right))|
            Edge { id: 1 << i, left, right, weight: left + right, length: 1 })
        .collect();
    let n = 1 + edges.iter().map(|e| e.left.max(e.right)).max().unwrap();

    let mut graph = vec![vec!();n];
    for e in edges {
        graph[e.left].push(e);
        graph[e.right].push(Edge {left: e.right, right: e.left, ..e});
    }
    for v in 0..n {
        let nbor = &graph[v];
        if nbor.len() == 2 {
            let e1 = nbor[0];
            let e2 = nbor[1];
            let v1 = e1.right;
            let v2 = e2.right;
            let new_edge = Edge { id: e1.id,
                                      left: v1,
                                      right: v2,
                                      weight: e1.weight + e2.weight,
                                      length: e1.length + e2.length
                                    };
            let e = graph[v1].iter_mut().find(|e| e.right == v).unwrap();
            *e = new_edge;
            let e = graph[v2].iter_mut().find(|e| e.right == v).unwrap();
            *e = Edge { left: v2, right: v1, ..new_edge };
        }
    }

    for nbor in graph.iter_mut() {
        nbor.sort_unstable_by_key(|e| e.left.abs_diff(e.right)); 
    }

    let mut stack = vec!();
    let mut p1 = 0;
    let mut p2 = (0, 0);

    stack.push(State { end: 0, edges: 0, length: 0, weight: 0 });

    while let Some(state) = stack.pop() {
        p1 = p1.max(state.weight);
        p2 = p2.max((state.length, state.weight));
        for edge in &graph[state.end] {
            if edge.id & state.edges != 0 {
                continue;
            }
            let new_state = State { end: edge.right,
                                          length: state.length + edge.length,
                                          weight: state.weight + edge.weight,
                                          edges: state.edges | edge.id
                                        };
            stack.push(new_state);
            if edge.left == edge.right {
                break;
            }
        }
    }
    (p1, p2.1)
}