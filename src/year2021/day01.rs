use anyhow::*;
use crate::util::{iter::AOCIter, parser::*};

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let numbers: Vec<u32> = input.iter_unsigned().collect();
    let p1 = numbers.array_windows().count_if(|[x, y]| x < y);
    let p2 = numbers.array_windows().count_if(|[x, _, _, y]| x < y);

    Ok((p1, p2))
}