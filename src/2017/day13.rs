use aoc::aoc_with_parser;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending,u64},
    sequence::separated_pair,
    multi::separated_list1,
    IResult,
};

fn input_parser(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let pair = separated_pair(u64, tag(": "), u64);
    separated_list1(line_ending, pair)(input)
}

#[inline]
fn caught(depth: u64, range: u64) -> bool {
    depth % ((range-1)*2) == 0
}

fn part1(pairs: &Vec<(u64, u64)>) -> u64 {
    pairs
    .iter()
    .filter(|(depth, range)| caught(*depth, *range))
    .map(|(depth, range)| depth * range)
    .sum() 
}

fn part2(pairs: &Vec<(u64, u64)>) -> u64 {
    (0..)
    .find(|i| ! pairs.iter().any(|(depth, range)| caught(i+depth, *range)))
    .unwrap()
}
