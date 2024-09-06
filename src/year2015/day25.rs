use anyhow::*;
use itertools::Itertools;
use crate::util::{parser::*, power};

pub fn solve(input: &str) -> Result<(u64, u64)> {
    let (row, col) = input.iter_unsigned().next_tuple().ok_or_else(|| anyhow!("Parse error"))?;
    let p1 = part1(row, col);
    Ok((p1, 0))
}

fn mul(x: u64, y: u64) -> u64 {
    (x * y) % 33_554_393
}

pub fn part1(row: u64, col: u64) -> u64 {
    let first_code = 20_151_125;
    let base = 252_533;
    let exp = (row + col - 1) * (row + col - 2) / 2 + col - 1;
    mul(first_code, power(|&x, &y| mul(x, y), base, exp as usize))
}