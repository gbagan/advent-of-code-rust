// dynamic programming

use anyhow::*;
use crate::util::{grid::Grid, parallel::*, parser::*};

pub fn solve(input: &str) -> Result<(u64, u64)> {
    let puzzles: Vec<_> = input.try_parse_lines_and_collect(parse_line)?;
    let p1 = puzzles.iter().map(|(springs, groups)| {
        let mut springs = springs.to_vec();
        springs.push(b'.');
        count_arrangements(&springs, groups)
    }).sum();
    
    let p2 = puzzles
        .into_par_iter()
        .map(|puzzle| {
            let (springs, groups) = puzzle;

            let mut springs2 = springs.to_vec();
            let mut groups2 = groups.to_vec();
            for _ in 0..4 {
                springs2.push(b'?');
                springs2.extend_from_slice(springs);
                groups2.extend_from_slice(groups);
            }
            springs2.push(b'.');

            count_arrangements(&springs2, &groups2)
        })
        .sum();
    Ok((p1, p2))
}

fn parse_line(line: &str) -> Result<(&[u8], Vec<u8>)> {
    let (springs, groups) = line.split_once(' ')
                    .with_context(|| format!("Parse error on line: {line}"))?;
    let springs = springs.as_bytes();
    let groups = groups.iter_unsigned().collect();
    Ok((springs, groups))
}

fn count_arrangements(springs: &[u8], groups: &[u8]) -> u64 {
    let n = springs.len();

    let mut next_operational = vec![0; n];
    for i in (0..n).rev() {
        if springs[i] == b'.' {
            next_operational[i] = i;
        } else {
            next_operational[i] = next_operational[i+1];
        }
    }

    let mut table: Grid<u64> = Grid::new(springs.len()+1, groups.len()+1, 0);
    table[(springs.len(), groups.len())] = 1;
    for i in (0..springs.len()).rev() {
        for j in (0..groups.len()+1).rev() {
            let next_op = next_operational[i];
            if springs[i] != b'#' {
                table[(i, j)] = table[(i+1, j)]
            };
            if j < groups.len() {
                let i2 = i + groups[j] as usize;
                if next_op >= i2 && springs[i2] != b'#' {
                    table[(i, j)] += table[(i2+1, j+1)];
                }
            }
        }
    }
    table[(0, 0)]
}
