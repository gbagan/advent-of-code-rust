use anyhow::*;
use std::collections::HashMap;
use itertools::Itertools;

fn parse_line(s: &str) -> Option<(&str, i32, &str)> {
    let s = s.trim_end_matches('.');
    let (name1, _, g, gain, _, _, _, _, _, _, name2) = s.split(' ').next_tuple()?;
    let gain: i32 = gain.parse().ok()?;
    let gain = if g == "gain" {gain} else {-gain};
    Some((name1, gain, name2))
}

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let entries: Vec<_> = input
        .lines()
        .map(|line| parse_line(line).ok_or_else(|| anyhow!("Parse error on line {line}")))
        .try_collect()?;
    let mut i = 0;
    let mut dict = HashMap::new();
    for (name1, _, name2) in entries.iter() {
        if !dict.contains_key(name1) {
            dict.insert(*name1, i);
            i += 1;
        }
        if !dict.contains_key(name2) {
            dict.insert(*name2, i);
            i += 1;
        }
    }
    let n = i;

    let mut table = vec![0; n*n];

    for (city1, gain, city2) in &entries {
        let i = dict[city1];
        let j = dict[city2];
        table[i * n + j] += *gain;
        table[j * n + i] += *gain;
    }
    
    let mut p1 = 0;
    let mut p2 = 0;

    for perm in (1..n).permutations(n-1) {
        let mut sum = table[perm[0]]; // edge between 0 and first element of perm
        let mut min_edge = sum;
        for (i, j) in perm.iter().tuple_windows() {
            let edge = table[i*n+j];
            sum += edge;
            min_edge = min_edge.min(edge);
        }
        let edge = table[perm[n-2]]; // edge between 0 and last element of perm
        sum += edge;
        min_edge = min_edge.min(edge);
        p1 = p1.max(sum);
        p2 = p2.max(sum - min_edge);
    }
    Ok((p1, p2))
}
