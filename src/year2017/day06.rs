use itertools::iterate;
use aoc::iter::*;

pub fn parse(input: &str) -> Option<Vec<u32>> {
    Some(input.trim().split_ascii_whitespace().map(|x| x.split().ok()).collect())
}

fn step(banks: &Vec<u32>) -> Vec<u32> {
    let mut banks = banks.clone();
    let n = banks.len();
    let (idx, max) = banks
            .iter()
            .enumerate()
            .max_by(|(i, a), (j, b)| if a==b {j.cmp(i)} else {a.cmp(b)})
            .unwrap();
    banks[idx] = 0;
    for i in 1..=*max {
        banks[(idx+i as usize) % n] += 1;
    }
    banks
}

fn main() {
    let input = include_str!("../../inputs/2017/06");
    aoc_with_parser(input, input_parser, |banks| {
        let (i, j, _) = iterate(banks.clone(), step).find_repetition().unwrap();
        (j, j-i)
    })
}