use aoc::aoc_with_parser;
use nom::{
    character::complete::{char,u8},
    IResult,
    multi::separated_list1,
};
use aoc::knothash::{reverse,knothash};

fn input_parser(input: &str) -> IResult<&str,Vec<u8>> {
    separated_list1(char(','), u8)(input)
}

fn part2(input: &str) -> String {
    let dense_hash = knothash(&input);
    let p2: Vec<String> = dense_hash.iter().map(|i| format!("{:02x}", i)).collect();
    p2.join("")
}

fn main() {
    let input = include_str!("../../inputs/2017/10");
    aoc_with_parser(input, input_parser, |lengths| {
        let knot = reverse(&lengths, 1);
        let p1 = knot[0] as usize * knot[1] as usize;
        let p2 = part2(&input);
        (p1, p2)
    })
}