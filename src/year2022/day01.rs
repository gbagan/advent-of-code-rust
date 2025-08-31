use crate::util::{iter::*, parser::*};

pub fn solve(input: &str) -> (u32, u32) {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|text|text.iter_unsigned::<u32>().sum())
        .collect();
    
    calories.sort_unstable();

    let (top1, top2, top3) = calories.into_iter().rev().next_tuple().unwrap();

    (top1, top1 + top2 + top3)
}