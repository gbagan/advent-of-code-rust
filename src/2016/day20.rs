use aoc::aoc_with_parser;
use nom::{
    character::complete::{char, i64, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser
};
use aoc::range::Range;

fn input_parser(input: &str) -> IResult<&str, Vec<Range>> {
    let range =
        separated_pair(i64, char('-'), i64)
        .map(|(x, y)| Range::new(x, y));
    
    separated_list1(line_ending, range)(input)
}

fn main() {
    let input = include_str!("../../inputs/2016/20");
    aoc_with_parser(input, input_parser, |ranges| {
        let ranges = Range::disjoint_union(&ranges);
        let p1 = ranges[0].upper + 1;
        let p2: i64 = (1 << 32) - ranges.iter().map(|r| r.length()).sum::<i64>();
        (p1, p2)
    })
}