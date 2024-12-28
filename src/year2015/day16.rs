use itertools::Itertools;
use crate::util::parser::*;

pub struct Aunt {
    number: u32,
    compounds: Vec<(usize, u32)>,
}

fn parse_aunt(line: &str) -> Aunt {
    let mut words = line.split(' ');
    let (_, number) = words.next_tuple().unwrap();
    let number = number.try_unsigned().unwrap();
    let mut compounds = Vec::new();
    while let Some((compound, nbr)) = words.next_tuple() {
        let compound = compound.trim_end_matches(':');
        let nbr = nbr.try_unsigned().unwrap();
        compounds.push((to_index(compound.as_bytes()) , nbr));
    }
    Aunt {number, compounds}
}

pub fn solve(input: &str) -> (u32, u32) {
    let aunts: Vec<_> = input.lines().map(parse_aunt).collect();
    let p1 = solve_for(&aunts, test1);
    let p2 = solve_for(&aunts, test2);
    (p1, p2)
}

const fn to_index(clue: &[u8]) -> usize {
    clue[0] as usize * 26 + clue[2] as usize - 2619
}

const CLUES: [u32; 676] = { 
    let mut clues = [0; 676];
    //clues[to_index(b"akitas")] = 0;
    clues[to_index(b"cars")] = 2;
    clues[to_index(b"cats")] = 7;
    clues[to_index(b"children")] = 3;
    clues[to_index(b"goldfish")] = 5;
    clues[to_index(b"perfumes")] = 1;
    clues[to_index(b"pomeranians")] = 3;
    clues[to_index(b"samoyeds")] = 2;
    //clues[to_index(b"vizslas")] = 0;
    clues[to_index(b"trees")] = 3;
    clues
};

const CATS: usize = to_index(b"cats");
const TREES: usize = to_index(b"trees");
const POMERANIANS: usize = to_index(b"pomeranians");
const GOLDFISH: usize = to_index(b"goldfish");

fn check(aunt: &Aunt, test: fn(usize, u32, u32) -> bool) -> bool {
    aunt.compounds
        .iter()
        .all(|&(compound, nbr)| test(compound, CLUES[compound], nbr))
}

fn test1(_: usize, n: u32, m: u32) -> bool {
    n == m
}

fn test2(compound: usize, n: u32, m: u32) -> bool {
    match compound {
        CATS | TREES => n < m,
        POMERANIANS | GOLDFISH => n > m,
        _ => n == m
    }
}

fn solve_for(aunts: &[Aunt], test: fn(usize, u32, u32) -> bool) -> u32 {
    aunts.iter().find(|aunt| check(aunt, test)).unwrap().number
}