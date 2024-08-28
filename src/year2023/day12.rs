// dynamic programming

use rayon::prelude::*;

use crate::util::grid::Grid;

fn parse_line(line: &str) -> Option<(&[u8], Vec<u8>)> {
    let (springs, groups) = line.split_once(' ')?;
    let springs = springs.as_bytes();
    let groups = groups.split(',').filter_map(|x| x.parse().ok()).collect();
    Some((springs, groups))
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

pub fn parse(input: &str) -> Option<(u64, u64)> {
    let puzzles: Vec<_> = input.lines().filter_map(parse_line).collect();
    let p1 = puzzles.iter().map(|(springs, groups)| {
        let mut springs = springs.to_vec();
        springs.push(b'.');
        count_arrangements(&springs, groups)
    }).sum();
    let p2 = puzzles.par_iter().map(|(springs, groups)| {
        let mut springs2 =springs.to_vec();
        let mut groups2 = groups.to_vec();
        for _ in 0..4 {
            springs2.push(b'?');
            springs2.extend_from_slice(springs);
            groups2.extend_from_slice(groups);
        }
        springs2.push(b'.');
        count_arrangements(&springs2, &groups2)
    }).sum();

    Some((p1, p2))
}

pub fn part1(input: &(u64, u64)) -> Option<u64> {
    Some(input.0)
}

pub fn part2(input: &(u64, u64)) -> Option<u64> {
    Some(input.1)
}