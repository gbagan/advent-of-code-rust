use crate::util::iter::AOCIter;
use std::collections::HashSet;

pub struct Input<'a>  {
    replacements: Vec<(&'a str, &'a str)>,
    molecule: &'a str
}

pub fn parse(input: &str) -> Option<Input> {
    let mut replacements = vec!();
    let mut lines = input.lines ();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }
        let (mol1, mol2) = line.split_once(" => ")?;
            replacements.push((mol1, mol2));
    }
    let molecule = lines.next()?;
    Some(Input { replacements, molecule })
}

fn replacements<'a>(
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


pub fn part1(input: &Input) -> Option<usize> {
    let mut molecules = HashSet::new();
    for (from, to) in &input.replacements {
        for molecule in replacements(input.molecule, from, to) {
            molecules.insert(molecule);
        }
    }
    Some(molecules.len())
}

fn nb_atoms(mol: &str) -> usize {
    mol.chars().count_by(|c| c.is_ascii_uppercase())
}

pub fn part2(input: &Input) -> Option<usize> {
    let atoms = nb_atoms(input.molecule); //.chars().filter(|&c| c.is_ascii_uppercase()).count();
    let rn = input.molecule.matches("Rn").count();
    let y = input.molecule.matches('Y').count();
    let ar = input.molecule.matches("Ar").count();
    Some(atoms - rn - ar - y * 2 - 1)
}
