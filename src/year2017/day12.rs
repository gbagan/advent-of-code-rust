// connected components

use anyhow::*;
use crate::util::parser::*;
use itertools::Itertools;

fn parse_node(line: &str) -> Vec<usize> {
    line.iter_unsigned().skip(1).collect()
}

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let graph = input.lines().map(parse_node).collect_vec();
    let n = graph.len();
    let mut p1 = 0;
    let mut p2 = 0;
    let mut seen = vec![false; n];
    
    for start in 0..n {
        if seen[start] {
            continue;
        }
        p2 += 1;
        let mut stack = vec!(start);
        while let Some(v) = stack.pop() {
            if seen[v] {
                continue;
            }
            seen[v] = true;
            if start == 0 {
                p1 += 1;
            }
            for u in &graph[v] {
                stack.push(*u);
            }
        }

    }
    Ok((p1, p2))
}