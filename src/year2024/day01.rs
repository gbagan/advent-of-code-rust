use anyhow::*;
use itertools::Itertools;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let (mut list1, mut list2): (Vec<_>, Vec<_>) =
        input.iter_unsigned::<u32>().tuples().unzip();
    
    radsort::sort(&mut list1);
    radsort::sort(&mut list2);

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