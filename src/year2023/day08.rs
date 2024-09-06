use anyhow::*;
use itertools::Itertools;
use std::collections::HashMap;
use num_integer::Integer;

fn parse_line(line: &str) -> Result<(&str, (&str, &str))> {
    let (node, _, succ1, succ2) = line.split(' ').next_tuple()
                                            .ok_or_else(|| anyhow!("Parse error"))?;
    let succ1 = succ1.trim_matches(['(', ',']);
    let succ2 = succ2.trim_matches(')');
    Ok((node, (succ1, succ2)))
}

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let mut lines = input.lines();
    let (directions, _) = lines.next_tuple().ok_or_else(|| anyhow!("Parse error: empty input"))?;
    let directions = directions.as_bytes();
    let nodes: HashMap<&str, (&str, &str)> = lines.map(parse_line).try_collect()?;

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