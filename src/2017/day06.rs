use aoc::aoc_with_parser;
use itertools::iterate;
use aoc::iter::AOCIter;
use nom::{
    character::complete::{u32, space1},
    multi::separated_list1,
    IResult,
};

fn input_parser(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, u32)(input)
}

fn step(banks: &Vec<u32>) -> Vec<u32> {
    let mut banks2 = banks.clone();
    let n = banks.len();
    let (idx, max) = banks
            .iter()
            .enumerate()
            .max_by(|(i, a), (j, b)| if a==b {j.cmp(i)} else {a.cmp(b)})
            .unwrap();
    banks2[idx] = 0;
    for i in 1..=*max {
        banks2[(idx+i as usize) % n] += 1;
    }
    banks2
}

fn main() {
    let input = include_str!("../../inputs/2017/06");
    aoc_with_parser(input, input_parser, |banks| {
        let (i, j, _) = iterate(banks.clone(), step).find_repetition().unwrap();
        (j, j-i)
    })
}
