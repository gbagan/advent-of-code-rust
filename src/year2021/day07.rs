use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, u32) {
    let bin = |n: u32| n * (n+1) / 2;

    let mut numbers: Vec<u32> = input.iter_unsigned().collect();
    let mid = numbers.len() / 2;
    let median = *numbers.select_nth_unstable(mid).1;
    let p1 = numbers.iter().map(|x| x.abs_diff(median)).sum();

    let mean = numbers.iter().sum::<u32>() / numbers.len() as u32;
    let p2: u32 = numbers.iter().map(|&x| bin(x.abs_diff(mean))).sum();
    let p2b: u32 = numbers.iter().map(|&x| bin(x.abs_diff(mean+1))).sum();

    (p1, p2.min(p2b))
}