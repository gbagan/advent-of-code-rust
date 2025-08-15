// connected components

use crate::util::parser::*;
use arrayvec::ArrayVec;

pub fn solve(input: &str) -> (u32, u32) {
    let graph: Vec<_> = input.lines().map(parse_node).collect();
    let n = graph.len();
    let mut p1 = 0;
    let mut p2 = 0;
    let mut seen = vec![false; n];
    let mut stack = Vec::new();

    for start in 0..n {
        if seen[start] {
            continue;
        }
        p2 += 1;
        stack.clear();
        stack.push(start);
        while let Some(v) = stack.pop() {
            if seen[v] {
                continue;
            }
            seen[v] = true;
            if start == 0 {
                p1 += 1;
            }
            for &u in &graph[v] {
                stack.push(u);
            }
        }

    }
    (p1, p2)
}

fn parse_node(line: &str) -> ArrayVec<usize, 6> {
    (&line[4..]).iter_unsigned().collect()
}