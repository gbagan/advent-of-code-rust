use aoc::aoc_with_parser;
use nom::{
    branch::alt,
    character::complete::{anychar, char, none_of},
    IResult, Parser,
    sequence::{preceded, delimited},
    multi::{many0, separated_list0},
};
use aoc::coord::Coord;

fn parse_group(level: i64, input: &str) -> IResult<&str,Coord> {
    delimited(
        char('{'),
        separated_list0(
                char(','),
                alt((
                    |i| parse_group (level+1, i),
                    count_garbage
                ))
            ).map(|x| Coord::new(level, 0) + x.iter().sum::<Coord>()),
        char('}'),
    )(input)
}

fn count_garbage_aux(input: &str) -> IResult<&str, i64> {
    alt((
        preceded(char('!'), anychar).map(|_| 0),
        none_of("!>").map(|_| 1)
    ))(input)
}

fn count_garbage(input: &str) -> IResult<&str, Coord> {
    delimited(
        char('<'),
        many0(count_garbage_aux).map(|x| Coord::new(0, x.iter().sum())),
        char('>')
    )(input)
}

fn input_parser(input: &str) -> IResult<&str, Coord> {
    parse_group(1, input)
}

fn main() {
    let input = include_str!("../../inputs/2017/09");
    aoc_with_parser(input, input_parser, |c| (c.x, c.y))
}