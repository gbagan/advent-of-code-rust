use anyhow::*;
use crate::util::iter::AOCIter;
use std::collections::HashSet;

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let mut replacements = vec!();
    let mut lines = input.lines ();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (mol1, mol2) = line.split_once(" => ")
                .ok_or_else(|| anyhow!("Parse error: No delimiter => found on line {line}"))?;
        replacements.push((mol1, mol2));
    }
    let molecule = lines.next().ok_or_else(|| anyhow!("Parse error: No molecule found"))?;

    let p1 = part1(molecule, &replacements);
    let p2 = part2(molecule);
    Ok((p1, p2))
}

fn do_replacements<'a>(
    molecule: &'a str,
    from: &'a str,
    to: &'a str,
) -> impl Iterator<Item = String> + 'a {
    molecule.match_indices(from).map(|(i, _)| {
        let mut s = String::new();
        s.push_str(&molecule[..i]);
        s.push_str(to);
        s.push_str(&molecule[(i + from.len())..]);
        s
    })
}


pub fn part1(molecule: &str, replacements: &[(&str, &str)]) -> usize {
    let mut molecules = HashSet::new();
    for (from, to) in replacements {
        for molecule in do_replacements(molecule, from, to) {
            molecules.insert(molecule);
        }
    }
    molecules.len()
}

fn nb_atoms(mol: &str) -> usize {
    mol.chars().count_if(|c| c.is_ascii_uppercase())
}

pub fn part2(molecule: &str) -> usize {
    let atoms = nb_atoms(molecule); //.chars().filter(|&c| c.is_ascii_uppercase()).count();
    let rn = molecule.matches("Rn").count();
    let y = molecule.matches('Y').count();
    let ar = molecule.matches("Ar").count();
    atoms - rn - ar - y * 2 - 1
}
