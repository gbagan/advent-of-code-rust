use anyhow::*;
use itertools::Itertools;
use crate::util::parser::*;
use std::cmp::Ordering;

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let mut it = input.lines();
    let mut table = [false; 10000];

    for line in it.by_ref() {
        if line.is_empty() {
            break;
        }
        let (x, y) = line.iter_unsigned::<usize>().next_tuple().unwrap();
        table[x*100+y] = true;
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for line in it {    
        let row = line.iter_unsigned::<usize>().collect_vec();
        let mut row2 = row.clone();
        let n = row2.len();
        row2.sort_unstable_by(|a, b|
            if a == b {
                Ordering::Equal
            } else if table[a*100+b] {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        );
        if row == row2 {
            p1 += row2[n/2];
        } else {
            p2 += row2[n/2];
        }
    }
    Ok((p1, p2))
}