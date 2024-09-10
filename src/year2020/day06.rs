use anyhow::*;
use crate::util::parser::*;
use itertools::Itertools;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let answers: Vec<Vec<_>> = input
        .split("\n\n")
        .map(|group| group.try_parse_lines_and_collect(parse_line))
        .try_collect()?;
    let mut p1 = 0;
    let mut p2 = 0;

    for group in &answers {
        p1 += group.iter().fold(0, |x, &y| x | y).count_ones();
        p2 += group.iter().fold(u32::MAX, |x, &y| x & y).count_ones();
    }

    Ok((p1, p2))
}

fn parse_line(line: &str) ->  Result<u32> {
    line.bytes().try_fold(0, |acc, c| {
        if c.is_ascii_lowercase() {
            Ok(acc | 1 << (c - b'a')) 
        } else {
            bail!("Non lowercase character {}", c as char)
        }
    })
}
