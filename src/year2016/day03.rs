use anyhow::*;
use crate::util::{iter::*, parser::*};
use itertools::Itertools;

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let numbers:Vec<_> = input.iter_unsigned().collect();
    let p1 = count_triangles(numbers.iter().copied());
    let first = count_triangles(numbers.iter().step_by(3).copied());
    let second = count_triangles(numbers.iter().skip(1).step_by(3).copied());
    let third = count_triangles(numbers.iter().skip(2).step_by(3).copied());
    let p2 = first + second + third;
    Ok((p1, p2))
}

fn count_triangles(iter: impl Iterator<Item = u32>) -> usize {
    iter.tuples().count_if(|&(a, b, c)| a < b + c && b < a + c && c < a + b)
}