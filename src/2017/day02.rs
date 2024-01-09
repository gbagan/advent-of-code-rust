use aoc::aoc;
use itertools::Itertools;

use nom::{
    character::complete::{line_ending,space1,u32},
    multi::separated_list1,
    IResult,
};

fn input_parser(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(line_ending, separated_list1(space1, u32))(input)
}

fn part1(vec: &Vec<Vec<u32>>) -> u32 {
    vec.iter().map(|row| row.iter().max().unwrap() - row.iter().min().unwrap()).sum()
}

fn find_divisible(vec:&Vec<u32>) -> Option<u32> {
    vec.iter().sorted().tuple_combinations().find_map(|(x, y)|
        if y % x == 0 {Some(y / x)} else { None }
    )
}

fn part2(vec: &Vec<Vec<u32>>) -> u32 {
    vec.iter().filter_map(find_divisible).sum()
}

fn main() {
    let input = include_str!("../../inputs/2017/02");
    match input_parser(input) {
        Err(_) => println!("parsing error"),
        Ok ((_, vec)) => {
            aoc(|| (part1(&vec), part2(&vec)))
        }
    }
}
