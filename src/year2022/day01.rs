use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, u32) {
    let calories: Vec<u32> = input
        .split("\n\n")
        .map(|text|text.iter_unsigned::<u32>().sum())
        .collect();
    let mut top1 = 0;
    let mut top2 = 0;
    let mut top3 = 0;

    for val in calories {
        if val > top1 {
            (top1, top2, top3) = (val, top1, top2)
        } else if val > top2 {
            (top2, top3) = (val, top3)
        } else if val > top3 {
            top3 = val
        }
    }

    (top1, top1 + top2 + top3)
}