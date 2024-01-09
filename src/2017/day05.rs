use aoc::aoc;

use nom::{
    character::complete::{line_ending,i32},
    multi::separated_list1,
    IResult,
};

fn input_parser(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(line_ending, i32)(input)
}

fn part1(jumps: &Vec<i32>) -> u32 {
    let mut jumps = jumps.clone();
    let n = jumps.len() as i32;

    let mut steps = 0;
    let mut offset = 0;
    while offset < n {
        let tmp = offset + jumps[offset as usize];
        jumps[offset as usize] += 1;
        offset = tmp;
        steps += 1;
    }
    steps
}

fn part2(jumps: &Vec<i32>) -> u32 {
    let mut jumps = jumps.clone();
    let n = jumps.len() as i32;

    let mut steps = 0;
    let mut offset = 0;
    while offset < n {
        let offset2 = jumps[offset as usize];
        if offset2 >= 3 {
            jumps[offset as usize] -= 1;
        } else {
            jumps[offset as usize] += 1;
        }
        offset += offset2;
        steps += 1;
    }
    steps
}

fn main() {
    let input = include_str!("../../inputs/2017/05");
    match input_parser(input) {
        Err(_) => println!("parsing error"),
        Ok ((_, jumps)) => {
            aoc(|| (
                part1(&jumps),
                part2(&jumps),
            ))
        }
    }
}
