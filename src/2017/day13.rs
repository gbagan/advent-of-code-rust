use aoc::aoc;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending,u64},
    multi::separated_list1,
    IResult,
};

fn pair_parser(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, depth) = u64(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, range) = u64(input)?;
    Ok((input, (depth, range)))
}

fn input_parser(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(line_ending, pair_parser)(input)
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

fn main() {
    let input = include_str!("../../inputs/2017/13");
    match input_parser(input) {
        Err(_) => println!("parsing error"),
        Ok ((_, pairs)) => {
            aoc(|| (part1(&pairs), part2(&pairs)))
        }
    }
}
