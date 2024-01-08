use nom::{
    branch::alt,
    character::complete::{anychar, char, none_of},
    combinator::map,
    IResult,
    sequence::{preceded, delimited},
    multi::{many0, separated_list0},
};
use aoc::coord::Coord;

fn parse_group(level: i64, input: &str) -> IResult<&str,Coord> {
    delimited(
        char('{'),
        map(
            separated_list0(
                char(','),
                alt((
                    |i| parse_group (level+1, i),
                    count_garbage
                ))
            ), |x| Coord::new(level, 0) + x.iter().sum::<Coord>()),
        char('}'),
    )(input)
}

fn count_garbage_aux(input: &str) -> IResult<&str, i64> {
    alt((
        map(preceded(char('!'), anychar), |_| 0),
        map(none_of("!>"), |_| 1)
    ))(input)
}

fn count_garbage(input: &str) -> IResult<&str, Coord> {
    delimited(
        char('<'),
        map(many0(count_garbage_aux), |x| Coord::new(0, x.iter().sum())),
        char('>')
    )(input)
}

fn input_parser(input: &str) -> IResult<&str, Coord> {
    parse_group(1, input)
}

fn main() {
    let input = include_str!("../../inputs/2017/09");
    match input_parser(input) {
        Err(_) => println!("parsing error"),
        Ok ((_, c)) => {
            println!("Part 1: {}", c.x);
            println!("Part 2: {}", c.y);
        }
    }
}
