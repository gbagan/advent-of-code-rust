use aoc::aoc_with_parser;
use itertools::iterate;
use aoc::iter::AOCIter;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, anychar, u64},
    IResult,
};

fn line_parser(input: &str) -> IResult<&str, u64> {
    let (input, _) = tag("Generator ")(input)?;
    let (input, _) = anychar(input)?;
    let (input, _) = tag(" starts with ")(input)?;
    u64(input)
}

fn solve(input: &str) -> Option<(usize, usize)> {
    let (line1, line2) = input.lines().next_tuple()?;
    let a = line1.split_ascii_whitespace().nth(4).and_then(|s| s.parse().ok())?;
    let b = line2.split_ascii_whitespace().nth(4).and_then(|s| s.parse().ok())?;
    
    Some((0, 0))
}

#[inline]
fn next_a(a: &u64) -> u64 {
    a * 16807 % 2_147_483_647
}

#[inline]
fn next_b(a: &u64) -> u64 {
    a * 48271 % 2147483647
}

fn part1(a: u64, b: u64) -> usize {
    let iter_a = iterate(a, next_a);
    let iter_b = iterate(b, next_b);
    iter_a.zip(iter_b).take(40_000_000).count_by(|(a, b)| a & 0xffff == b & 0xffff)
}

fn part2(a: u64, b: u64) -> usize {
    let iter_a = iterate(a, next_a).filter(|&a| a & 3 == 0);
    let iter_b = iterate(b, next_b).filter(|&a| a & 7 == 0);
    iter_a.zip(iter_b).take(5_000_000).count_by(|(a, b)| a & 0xffff == b & 0xffff)
}

fn main() {
    let input = include_str!("../../inputs/2017/15");
    aoc_with_parser(input, input_parser, |(a, b)| {
        (part1(a, b), part2(a, b))
    })
}
