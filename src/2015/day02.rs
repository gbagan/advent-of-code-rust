use aoc::aoc_with_parser;
use nom::{
    character::complete::{char,u64},
    multi::separated_list1,
    sequence::tuple,
    IResult,
  };

struct Box {
    l: u64,
    h: u64,
    w: u64,
}

fn box_parser(input: &str) -> IResult<&str, Box> {
    let (input, (l, _, h, _, w)) = tuple((u64, char('x'), u64, char('x'), u64))(input)?;
    Ok((input, Box { l, h, w }))
}

fn input_parser(input: &str) -> IResult<&str, Vec<Box>> {
    separated_list1(char('\n'), box_parser)(input)
}

fn paper (Box {l, h, w}: &Box) -> u64 {
    let areas = [l*w, l*h, w*h];
    let sum_areas: u64 = areas.into_iter().sum();
    2 * sum_areas + areas.into_iter().min().unwrap()
}

fn part1 (boxes: &Vec<Box>) -> u64 {
    boxes.iter().map(paper).sum()
}

fn ribbon (Box {l, h, w}: &Box) -> u64 {
    l * h * w + 2 * [l+w, l+h, w+h].into_iter().min().unwrap()
}

fn part2 (boxes: &Vec<Box>) -> u64 {
    boxes.iter().map(ribbon).sum()
}

fn main() {
    let input = include_str!("../../inputs/2015/02");
    aoc_with_parser(input, input_parser, |boxes| (part1(&boxes), part2(&boxes)))
}