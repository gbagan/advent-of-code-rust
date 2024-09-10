use anyhow::*;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let mut ids: Vec<_> = input.try_parse_lines_and_collect(parse_id)?;
    ids.sort_unstable();
    let p1 = *ids.last().unwrap();
    let p2 = ids
            .array_windows()
            .find_map(|&[x, y]| (y-x == 2).then_some(x+1))
            .context("Part 2: No solution found")?;
    Ok((p1, p2))
}

fn parse_id(line: &str) -> Result<u32> {
    line.bytes().try_fold(0, |acc, c|
        match c {
            b'F' | b'L' => Ok(2*acc),
            b'B' | b'R' => Ok(2*acc+1),
            _ => bail!("Unexpected character {}", c as char)
        }
    )
}