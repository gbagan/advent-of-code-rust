use anyhow::*;
use itertools::Itertools;
use crate::util::parser::*;
use std::cmp::Ordering;

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let mut table = [Ordering::Greater; 10000];

    let split = memchr::memmem::find(input.as_bytes(), b"\n\n").context("No separator found")?;
    let section1 = &input[..split];
    let section2 = &input[split+2..];
    for (x, y) in section1.iter_unsigned::<usize>().tuples() {
        table[x*100+y] = Ordering::Less;
    }

    let mut p1 = 0;
    let mut p2 = 0;
    let mut row = Vec::new();

    for line in section2.lines() {    
        row.clear();
        row.extend(line.iter_unsigned::<usize>());
        let n = row.len();
        if row.is_sorted_by(|x, y| table[x*100+y] == Ordering::Less) {
            p1 += row[n/2];
        } else {
            p2 += *row.select_nth_unstable_by(n/2, |x, y| table[x*100+y]).1;
        }

    }
    Ok((p1, p2))
}