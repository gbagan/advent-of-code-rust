use anyhow::*;
use itertools::Itertools;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for (x, y) in input.iter_unsigned::<u32>().tuples() {
        list1.push(x);
        list2.push(y);
    }    
    list1.sort_unstable();
    list2.sort_unstable();

    let p1 = list1.iter().zip(list2.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum();
 
    let n = list2.len();
    let mut p2 = 0;
    let mut i = 0;

    for x in list1 {
        while i < n && list2[i] < x {
            i += 1;
        }
        let mut counter = 0;
        while i < n && list2[i] == x {
            counter += 1;
            i += 1;
        }
        p2 += x * counter;
    }

    Ok((p1, p2))
}