use anyhow::*;
use itertools::Itertools;
use crate::util::{math::chinese_remainder2, parser::*};

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let pairs: Vec<_> = input
            .iter_unsigned::<i64>()
            .tuples()
            .map(|(i, m, _, p)| (-p-i, m))
            .collect();

    let (p1, n) = chinese_remainder2(&pairs)
        .context("Part 1: No solution found")?;

    let (p2, _) = chinese_remainder2(&[(p1, n), (-(pairs.len() as i64) - 1, 11)])
        .context("Part 1: No solution found")?;

    Ok((p1, p2))
}