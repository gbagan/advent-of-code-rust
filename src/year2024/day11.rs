use std::collections::HashMap;
use anyhow::*;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(u64, u64)> {
    let mut arrangement: HashMap<u64, u64> = input.iter_unsigned::<u64>().map(|x| (x, 1)).collect();
    
    for _ in 0..25 {
        arrangement = next_arrangement(&arrangement);
    }

    let p1 = arrangement.values().sum();
    
    for _ in 0..50 {
        arrangement = next_arrangement(&arrangement);
    }

    let p2 = arrangement.values().sum();

    Ok((p1, p2))
}

pub fn next_arrangement(current: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut next = HashMap::with_capacity(2*current.len());
    for (&value, &occurences) in current {
        if value == 0 {
            *next.entry(1).or_insert(0) += occurences;
        } else if let Some((prefix, suffix)) = split_number(value) {
            *next.entry(prefix).or_insert(0) += occurences;
            *next.entry(suffix).or_insert(0) += occurences;
        } else {
            *next.entry(value * 2024).or_insert(0) += occurences;
        }
    }
    next
}

fn split_number(n: u64) -> Option<(u64, u64)> {
    let m = n.ilog10() + 1;
    if m & 1 == 0 {
        let p = 10u64.pow(m >> 1);
        Some((n / p, n % p))
    } else {
        None
    }
}