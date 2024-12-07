use anyhow::*;
use itertools::Itertools;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(u64, u64)> {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let row = line.iter_unsigned::<u64>().collect_vec();
        if solve_p1(&row, row[0],row.len()-1) {
            p1 += row[0];
        }
        if solve_p2(&row, row[0],row.len()-1) {
            p2 += row[0];
        }
    }

    Ok((p1, p2))
}

pub fn solve_p1(row: &[u64], acc: u64, idx: usize) -> bool {
    let current = row[idx];
    if idx == 1 {
        acc == row[1]
    } else {
        if acc % current == 0 && solve_p1(row, acc / current, idx - 1) {
            return true;
        }
        acc >= current && solve_p1(row, acc - current, idx - 1)
    }
}

pub fn solve_p2(row: &[u64], acc: u64, idx: usize) -> bool {
    let current = row[idx];
    if idx == 1 {
        acc == row[1]
    } else {
        if let Some(acc2) = truncate_number(acc, current) {
            if solve_p2(row, acc2, idx - 1) {
                return true;
            }
        }
        if acc % current == 0 && solve_p2(row, acc / current, idx - 1) {
            return true;
        }
        acc >= current && solve_p2(row, acc - current, idx - 1)
    }
}

fn truncate_number(x: u64, y: u64) -> Option<u64> {
    let n = y.ilog10();
    let m = 10u64.pow(n+1);
    if x % m == y {
        Some(x / m)
    } else {
        None
    }
}