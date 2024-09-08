use anyhow::*;
use itertools::Itertools;
use std::collections::HashMap;
use num_integer::Integer;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let (directions, rest) = input.try_split_once("\n\n")?;
    let directions = directions.as_bytes();
    let nodes: HashMap<_, _> = rest.iter_uppercase().tuples().map(|(x, y, z)| (x, (y, z))).collect();

    let starting_nodes: Vec<_> = nodes.keys().filter(|n| n.ends_with('A')).collect();

    let n = directions.len();
    let mut p1 = n;
    let mut p2 = n;

    for &start in starting_nodes {
        let mut counter = 0;
        let mut current1 = start;
        let mut current2 = start;
        while !current1.ends_with('Z') && !current2.ends_with('Z') {
            (current1, current2) = nodes[current2];
            counter += 1;
        }
        if current1 == "ZZZ" || current2 == "ZZZ" {
            p1 = p1.lcm(&counter);
        }
        p2 = p2.lcm(&counter);
    }

    Ok((p1, p2))
}