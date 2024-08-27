use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Aunt<'a> {
    number: u32,
    compounds: Vec<(&'a str, u32)>,
}

fn parse_aunt(line: &str) -> Option<Aunt> {
    let mut words = line.split(' ');
    let (_, number) = words.next_tuple()?;
    let number = number.trim_end_matches(':').parse().ok()?;
    let mut compounds = Vec::new();
    while let Some((compound, nbr)) = words.next_tuple() {
        let compound = compound.trim_end_matches(':');
        let nbr = nbr.trim_end_matches(',').parse().ok()?;
        compounds.push((compound, nbr));
    }
    Some(Aunt {number, compounds})
}

pub fn parse(input: &str) -> Option<Vec<Aunt>> {
    Some(input.lines().filter_map(parse_aunt).collect())
}

lazy_static! {
    static ref clues: HashMap<&'static str,u32> = { 
        let mut m = HashMap::new();
        m.insert("children", 3);
        m.insert("cats", 7);
        m.insert("samoyeds", 2);
        m.insert("pomeranians", 3);
        m.insert("akitas", 0);
        m.insert("vizslas", 0);
        m.insert("goldfish", 5);
        m.insert("trees", 3);
        m.insert("cars", 2);
        m.insert("perfumes", 1);
        m
    };
}

fn check(aunt: &Aunt, test: fn(&str, u32, u32) -> bool) -> bool {
    aunt.compounds
        .iter()
        .all(|(compound, nbr)|
            match clues.get(compound) {
                None => true,
                Some(nbr2) => test(*compound, *nbr2, *nbr)
            }
        )
}

fn test1(_: &str, n: u32, m: u32) -> bool {
    n == m
}

fn test2(compound: &str, n: u32, m: u32) -> bool {
    match compound {
        "cats" | "trees" => n < m,
        "pomeranians" | "goldfish" => n > m,
        _ => n == m
    }
}

pub fn solve(aunts: &[Aunt], test: fn(&str, u32, u32) -> bool) -> Option<u32> {
    aunts
        .iter()
        .find(|aunt| check(aunt, test))
        .map(|aunt| aunt.number)
}

pub fn part1(aunts: &[Aunt]) -> Option<u32> {
    solve(aunts, test1)
}

pub fn part2(aunts: &[Aunt]) -> Option<u32> {
    solve(aunts, test2)
}