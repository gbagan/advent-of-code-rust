use anyhow::*;
use itertools::Itertools;
use crate::util::parser::*;
use std::cmp::Ordering;

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let mut table = [false; 10000];

    let (section1, section2) = input.try_split_once("\n\n")?;
    for (x, y) in section1.iter_unsigned::<usize>().tuples() {
        table[x*100+y] = true;
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for line in section2.lines() {    
        let mut row = line.iter_unsigned::<usize>().collect_vec();
        let n = row.len();
        if row.array_windows().all(|[x, y]| table[x*100+y]) {
            p1 += row[n/2];
        } else {
            p2 += *row.select_nth_unstable_by(n/2, |x, y|
                if table[x*100+y] {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }).1;
        }
    }
    Ok((p1, p2))
}