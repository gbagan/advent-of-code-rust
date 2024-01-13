use aoc::aoc_with_parser;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending,space1,u32},
    combinator::map,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

struct RGB {
    r: u32,
    g: u32,
    b: u32,
}

impl RGB {
    fn is_possible_set(&self) -> bool {
        self.r <= 12 && self.g <= 13 && self.b <= 14
    }

    fn max(&self, other: &RGB) -> RGB {
        RGB {r: self.r.max(other.r),
             g: self.g.max(other.g),
             b: self.b.max(other.b)
            }
    }

    fn power (&self) -> u32 {
        self.r * self.g * self.b
    }
}

struct Game {
    id: u32,
    bags: Vec<RGB>
}

fn input_parser(input: &str) -> IResult<&str, Vec<Game>> {
    fn color_set (input: &str) -> IResult<&str, RGB> {
        let (input, n) = terminated(u32, space1)(input)?;
        let (input, bag) = alt((
            map(tag("red"), |_| RGB { r: n, g: 0, b: 0}),
            map(tag("green"), |_| RGB { r: 0, g: n, b: 0}),
            map(tag("blue"), |_| RGB { r: 0, g: 0, b: n}),
        ))(input)?;
        Ok((input, bag))
    }

    let game = map(
        tuple((tag("Game "), u32, tag(": "), separated_list1(alt((tag("; "), tag(", "))), color_set))),
        |(_, id, _, bags)| Game {id, bags });

    separated_list1(line_ending, game)(input)
}

fn part1(games: &Vec<Game>) -> u32 {
    games.iter().filter(|game| game.bags.iter().all(|b| b.is_possible_set())).map(|g| g.id).sum()
}

fn part2_aux(game: &Game) -> u32 {
    game.bags.iter().fold(RGB { r: 0, g: 0, b: 0}, |acc, game| acc.max(game)).power()
}

fn part2(games: &Vec<Game>) -> u32 {
    games.iter().map(part2_aux).sum()
}

fn main() {
    let input = include_str!("../../inputs/2023/02");
    aoc_with_parser(input, input_parser, |games| (part1(&games), part2(&games)))
}