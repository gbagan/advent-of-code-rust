use aoc::aoc_with_parser;
use nom::{
    branch::alt,
    character::complete::{anychar, char, u8},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use aoc::permutation::Permutation;
use aoc::number::power;

enum Move {
    Spin(u8), Exchange(u8, u8), Partner(char, char)
}

// a dance is a tuple of (p, q)
// p represents the permutation implied by Spin and Exchange
// q represents the permutation implied by Partner
type Dance = (Permutation, Permutation);

fn input_parser(input: &str) -> IResult<&str, Vec<Move>> {
    let spin = map(preceded(char('s'), u8), |a| Move::Spin(a));
    let exchange = map(
        tuple((char('x'), u8, char('/'), u8)),
        |(_, a, _, b)| Move::Exchange(a, b));
    
    let partner = map(
        tuple((char('p'), anychar, char('/'), anychar)),
        |(_, a, _, b)| Move::Partner(a, b));
    
    separated_list1(char(','), alt((spin, exchange, partner)))(input)
}

fn letter_to_int(c: char) -> usize {
    (c as usize) - ('a' as usize)     
}

fn perform_dance(moves: &Vec<Move>) -> Dance {
    let mut perm1 = Permutation::one(16);
    let mut perm2 = Permutation::one(16);
    for move_ in moves {
        match move_ {
            Move::Spin(n) => perm1.indices.rotate_right(*n as usize),
            Move::Exchange(a, b) => perm1.indices.swap(*a as usize, *b as usize),
            Move::Partner(a, b) => perm2.indices.swap(letter_to_int(*a), letter_to_int(*b)),
        }
    }
    (perm1.inv(), perm2)
}

fn compose_dance(d1: &Dance, d2: &Dance) -> Dance {
    (&d1.0 >> &d2.0, &d1.1 >> &d2.1)
}

fn main() {
    let input = include_str!("../../inputs/2017/16");
    aoc_with_parser(input, input_parser, |moves| {
        let programs: Vec<_> = "abcdefghijklmnop".chars().collect();
        let dance = perform_dance(&moves);
    
        let p1 = (&dance.1 >> &dance.0).apply(&programs);
        let p1: String = p1.iter().collect();
    
        let pdance = power(compose_dance, dance.clone(), 1_000_000_000);
        let p2 = (&pdance.1 >> &pdance.0).apply(&programs);
        let p2: String = p2.iter().collect();

        (p1, p2)
    })
}