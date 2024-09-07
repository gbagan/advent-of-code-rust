// dynamic programming
// subset sum

use anyhow::*;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(u64, u64)> {
    let numbers: Vec<_> = input.iter_unsigned().collect();
    let p1 = subset_sum(&numbers,  numbers.iter().sum::<u64>() / 3)
        .context("Part 1: No solution found")?;
    let p2 = subset_sum(&numbers,  numbers.iter().sum::<u64>() / 4)
        .context("Part 2: No solution found")?;
    Ok((p1, p2))
}

fn subset_sum(numbers: &[u64], n: u64) -> Option<u64> {
    let n = n as usize;
    // to avoid integer overflow
    let limit = u64::MAX / numbers.iter().max().unwrap();
    let m = numbers.len();
    let size = (m+1) * (n+1);
    let mut table: Vec<Option<u64>> = vec![None; size];
    table[0] = Some(1);
    for i in 1..m+1 {
        let v = numbers[i-1];
        for j in 0..n+1 {
            let index = i * (n+1) + j;
            table[index] =
                if v > j as u64 {
                    table[index - (n+1)]
                } else {
                    match (table[index - (n+1)], table[index - (n+1) - v as usize]) {
                        (x, None) => x,
                        (None, Some(x)) => Some(limit.min(v * x)),
                        (Some(x), Some(y)) => Some(x.min(v * y).min(limit))
                    }
                }
        }
    }
    table[size-1]
}