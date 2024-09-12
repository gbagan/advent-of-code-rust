use anyhow::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::util::parser::*;

pub struct Aunt<'a> {
    number: u32,
    compounds: Vec<(&'a str, u32)>,
}

fn parse_aunt(line: &str) -> Result<Aunt> {
    let mut words = line.split(' ');
    let (_, number) = words.next_tuple().context("no ' ' found")?;
    let number = number.try_unsigned()?;
    let mut compounds = Vec::new();
    while let Some((compound, nbr)) = words.next_tuple() {
        let compound = compound.trim_end_matches(':');
        let nbr = nbr.try_unsigned()?;
        compounds.push((compound, nbr));
    }
    Ok(Aunt {number, compounds})
}

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let aunts: Vec<_> = input.try_parse_lines_and_collect(parse_aunt)?;
    let p1 = solve_for(&aunts, test1).context("Part1: No solution found")?;
    let p2 = solve_for(&aunts, test2).context("Part2: No solution found")?;
    Ok((p1, p2))
}

lazy_static! {
    static ref CLUES: HashMap<&'static str,u32> = { 
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
            match CLUES.get(compound) {
                None => true,
                Some(nbr2) => test(compound, *nbr2, *nbr)
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

fn solve_for(aunts: &[Aunt], test: fn(&str, u32, u32) -> bool) -> Option<u32> {
    aunts
        .iter()
        .find(|aunt| check(aunt, test))
        .map(|aunt| aunt.number)
}