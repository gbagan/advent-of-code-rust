use std::time::Instant;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    IResult,
    multi::separated_list0,
};
use aoc::coord::Coord;

fn direction(input: &str) -> IResult<&str,Coord> {
    alt((
        map(tag("nw"), |_| Coord::new(-1, -1)),
        map(tag("ne"), |_| Coord::new(1, 0)),
        map(tag("n"), |_| Coord::new(0, -1)),
        map(tag("sw"), |_| Coord::new(-1, 0)),
        map(tag("se"), |_| Coord::new(1, 1)),
        map(tag("s"), |_| Coord::new(0, 1)),
    ))(input)
}

fn input_parser(input: &str) -> IResult<&str,Vec<Coord>> {
    separated_list0(char(','), direction)(input)
}

fn distance(Coord {x, y}: &Coord) -> i64 {
    x.abs().max(y.abs()).max((x-y).abs())
}


fn main() {
    let input = include_str!("../../inputs/2017/11");
    //let input = "AoC 2017";
    match input_parser(input) {
        Err(_) => println!("parsing error"),
        Ok ((_, dirs)) => {
            let start = Instant::now();

            let coords: Vec<_> = dirs.iter().scan(Coord::origin(), |acc, dir| {
                *acc += *dir;
                Some (*acc)
            }).collect();

            println!("{}", dirs.len());

            let p1 = distance(coords.last().unwrap());
            let p2 = coords.iter().map(distance).max().unwrap();

            let end = start.elapsed().as_micros();

            println!("Part 1: {}", p1);
            println!("Part 2: {}", p2);
            println!("Time: {} μs", end);
        }
    }
}
