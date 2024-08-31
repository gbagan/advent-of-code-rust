use crate::util::parser::*;

pub fn solve(input: &str) -> Option<(u32, u32)> {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|text|text.iter_unsigned::<u32>().sum())
        .collect();
    calories.sort_unstable();
    let p1 = calories[calories.len()-1];
    let p2 = calories.iter().rev().take(3).sum();
    Some((p1, p2))   
}