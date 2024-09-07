use anyhow::*;
use itertools::Itertools;
use crate::util::parser::*;

fn parse_line(line: &str) -> Vec<u32> {
    let mut numbers = line.iter_unsigned().collect_vec();
    numbers.sort_unstable();
    numbers
}

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let mut p1 = 0;
    let mut p2 = 0;
    for numbers in input.lines().map(parse_line) {
        p1 += numbers[numbers.len()-1] - numbers[0];
        p2 += numbers.iter().tuple_combinations().find_map(|(x, y)|
                if y % x == 0 {Some(y / x)} else { None }
                ).unwrap_or(0);
    }
    Ok((p1, p2))
}
