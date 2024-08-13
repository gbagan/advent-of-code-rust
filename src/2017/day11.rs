use aoc::aoc_with_parser;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    IResult, Parser,
    multi::separated_list1,
};
use aoc::coord::Coord;

fn input_parser(input: &str) -> IResult<&str,Vec<Coord>> {
    let direction = alt((
        tag("nw").map(|_| Coord::new(-1, -1)),
        tag("ne").map(|_| Coord::new(1, 0)),
        tag("n").map(|_| Coord::new(0, -1)),
        tag("sw").map(|_| Coord::new(-1, 0)),
        tag("se").map(|_| Coord::new(1, 1)),
        tag("s").map(|_| Coord::new(0, 1)),
    ));
    
    separated_list1(char(','), direction)(input)
}

fn distance(Coord {x, y}: &Coord) -> i64 {
    x.abs().max(y.abs()).max((x-y).abs())
}

fn main() {
    let input = include_str!("../../inputs/2017/11");
    aoc_with_parser(input, input_parser, |dirs| {
        let coords: Vec<_> = dirs.iter().scan(Coord::origin(), |acc, dir| {
            *acc += *dir;
            Some (*acc)
        }).collect();
        let p1 = distance(coords.last().unwrap());
        let p2 = coords.iter().map(distance).max().unwrap();
        (p1, p2)
    })
}