// dynamic programming

use crate::util::{parallel::*, parser::*};

pub fn solve(input: &str) -> (u64, u64) {
    let puzzles: Vec<_> = input.lines().map(parse_line).collect();
    let mut springs2 = Vec::new();
    let mut table = Vec::new();
    let mut next_operational = Vec::new();

    let p1 = puzzles.iter().map(|(springs, groups)| {
        springs2.clear();
        springs2.extend_from_slice(springs);
        springs2.push(b'.');
        count_arrangements(&springs2, groups, &mut table, &mut next_operational)
    }).sum();
    
    let p2 = puzzles
        .into_par_iter()
        .chunks(16)
        .map(|puzzles| {
            let max_spring_len = puzzles.iter().map(|p| p.0.len()*5+5).max().unwrap();
            let max_group_len = puzzles.iter().map(|p| p.1.len()*5).max().unwrap();
            let max_table_size = puzzles.iter().map(|p|
                    ((p.0.len()*5+6)*(p.1.len()*5+1)))
                .max().unwrap();

            let mut springs2 = Vec::with_capacity(max_spring_len);
            let mut groups2 = Vec::with_capacity(max_group_len);
            let mut table = Vec::with_capacity(max_table_size);
            let mut next_operational = Vec::with_capacity(max_spring_len);

            let mut sum = 0;
            for (springs, groups) in puzzles {
                springs2.clear();
                groups2.clear();
                springs2.extend_from_slice(springs);
                groups2.extend_from_slice(groups);
                for _ in 0..4 {
                    springs2.push(b'?');
                    springs2.extend_from_slice(springs);
                    groups2.extend_from_slice(groups);
                }
                springs2.push(b'.');

                sum += count_arrangements(&springs2, &groups2, &mut table, &mut next_operational)
            }
            sum
        })
        .sum();
    (p1, p2)
}

fn parse_line(line: &str) -> (&[u8], Vec<u8>) {
    let (springs, groups) = line.split_once(' ').unwrap();
    let groups = groups.iter_unsigned().collect();
    (springs.as_bytes(), groups)
}

fn count_arrangements(springs: &[u8], groups: &[u8], table: &mut Vec<u64>, next_operational: &mut Vec<usize>) -> u64 {
    let n = springs.len();

    next_operational.clear();
    next_operational.resize(n, 0);

    for i in (0..n).rev() {
        if springs[i] == b'.' {
            next_operational[i] = i;
        } else {
            next_operational[i] = next_operational[i+1];
        }
    }

    let width = groups.len()+1;
    let height = n+1;

    table.clear();
    table.resize(width * height, 0);    
    table[springs.len() * width + groups.len()] = 1;
    for i in (0..springs.len()).rev() {
        for j in (0..groups.len()+1).rev() {
            let next_op = next_operational[i];
            if springs[i] != b'#' {
                table[i *width + j] = table[(i+1) * width + j]
            };
            if j < groups.len() {
                let i2 = i + groups[j] as usize;
                if next_op >= i2 && springs[i2] != b'#' {
                    table[i * width + j] += table[(i2+1) * width +  j+1];
                }
            }
        }
    }
    table[0]
}
