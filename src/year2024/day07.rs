use anyhow::*;
use itertools::Itertools;
use crate::util::{parallel::*, parser::*};

pub fn solve(input: &str) -> Result<(u64, u64)> {
    let mut p1 = 0;

    let rows = input.lines()
        .map(|line| line.iter_unsigned::<u64>().collect_vec())
        .collect_vec();

    for row in &rows {
        if solve_p1(row, row[1], 2) {
            p1 += row[0];
        }
    }

    let p2 = rows
        .into_par_iter()
        .map(|row|
            if solve_p2(&row, row[1], 2) {
                row[0]
            } else {
                0
            }
        ).sum();

    Ok((p1, p2))
}

pub fn solve_p1(row: &[u64], acc: u64, idx: usize) -> bool {
    if idx >= row.len() {
        acc == row[0]
    } else {
        let acc2 = acc * row[idx];
        if acc2 <= row[0] && solve_p1(row, acc2, idx + 1) {
            return true;
        }
        let acc2 = acc + row[idx];
        acc2 <= row[0] && solve_p1(row, acc2, idx + 1)
    }
}

pub fn solve_p2(row: &[u64], acc: u64, idx: usize) -> bool {
    if idx >= row.len() {
        acc == row[0]
    } else {
        let acc2 = concat_numbers(acc, row[idx]);
        if acc2 <= row[0] && solve_p2(row, acc2, idx + 1) {
            return true;
        }
        let acc2 = acc * row[idx];
        if acc2 <= row[0] && solve_p2(row, acc2, idx + 1) {
            return true;
        }
        let acc2 = acc + row[idx];
        acc2 <= row[0] && solve_p2(row, acc2, idx + 1)
    }
}

fn concat_numbers(x: u64, y: u64) -> u64 {
    let n = y.ilog10();
    10u64.pow(n+1) * x + y
}