use aoc::aoc;
use nom::{
    branch::alt,
    character::complete::{anychar, char, u8},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use itertools::iterate;
use aoc::iter::AOCIter;

enum Move {
    Spin(u8), Exchange(u8, u8), Partner(char, char)
}

fn exchange_parser (input: &str) -> IResult<&str, Move> {
    let (input, a) = preceded(char('x'), u8)(input)?;
    let (input, b) = preceded(char('/'), u8)(input)?;
    Ok((input, Move::Exchange(a, b)))
}

fn partner_parser (input: &str) -> IResult<&str, Move> {
    let (input, a) = preceded(char('p'),  anychar)(input)?;
    let (input, b) = preceded(char('/'), anychar)(input)?;
    Ok((input, Move::Partner(a, b)))
}

fn move_parser (input: &str) -> IResult<&str, Move> {
    alt((
        map(preceded(char('s'), u8), |a| Move::Spin(a)),
        exchange_parser,
        partner_parser,
    ))(input)
}

fn input_parser (input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(char(','), move_parser)(input)
}

fn dance(programs: &Vec<char>, moves: &Vec<Move>) -> Vec<char> {
    let mut programs = programs.clone();
    for move_ in moves {
        match move_ {
            Move::Spin(n) => programs.rotate_right(*n as usize),
            Move::Exchange(a, b) => programs.swap(*a as usize, *b as usize),
            Move::Partner(a, b) => {
                let i = programs.iter().position(|&r| r == *a).unwrap();
                let j = programs.iter().position(|&r| r == *b).unwrap();
                programs.swap(i, j);
            },
        }
    }
    programs
}

fn part1(moves: &Vec<Move>) -> String {
    let programs: Vec<_> = "abcdefghijklmnop".chars().collect();
    let programs = dance(&programs, moves);
    programs.iter().collect()
}

fn part2(moves: &Vec<Move>) -> String {
    let programs: Vec<_> = "abcdefghijklmnop".chars().collect();
    let (i, j, mut progs) = iterate(programs, |p| dance(&p, &moves))
                                                .find_repetition()
                                                .unwrap();
    let n = (1_000_000_000 - i) % (j - i);
    for _ in 0..n {
        progs = dance(&progs, &moves);
    }
    progs.iter().collect()
}

fn main() {
    let input = include_str!("../../inputs/2017/16");

    match input_parser(input) {
        Err(_) => println!("parsing error"),
        Ok ((_, moves)) => {
            println!("{}", moves.len());
            aoc(|| (part1(&moves), part2(&moves)))
        }
    }
}