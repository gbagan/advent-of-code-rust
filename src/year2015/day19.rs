use ahash::{HashSet, HashSetExt};
use memchr::memmem;

pub fn solve(input: &str) -> (usize, usize) {
    let mut replacements = vec!();
    let mut lines = input.lines ();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (mol1, mol2) = line.split_once(" => ").unwrap();
        replacements.push((mol1.as_bytes(), mol2.as_bytes()));
    }
    let molecule = lines.next().unwrap().as_bytes();

    let p1 = part1(molecule, &replacements);
    let p2 = part2(molecule);
    (p1, p2)
}

pub fn part1(molecule: &[u8], replacements: &[(&[u8], &[u8])]) -> usize {
    let mut molecules = HashSet::new();
    for (from, to) in replacements {
        for molecule in do_replacements(molecule, from, to) {
            molecules.insert(molecule);
        }
    }
    molecules.len()
}

fn do_replacements<'a>(
    molecule: &'a [u8],
    from: &'a [u8],
    to: &'a [u8],
) -> impl Iterator<Item = Vec<u8>> + 'a {
    memmem::find_iter(molecule, from).map(|i| {
        let mut s = Vec::with_capacity(molecule.len() + to.len() - from.len());
        s.extend_from_slice(&molecule[..i]);
        s.extend_from_slice(to);
        s.extend_from_slice(&molecule[(i + from.len())..]);
        s
    })
}

pub fn part2(molecule: &[u8]) -> usize {
    let atoms = nb_atoms(molecule);
    let rn = memmem::find_iter(molecule, b"Rn").count();
    let y = memmem::find_iter(molecule, b"Y").count();
    let ar = memmem::find_iter(molecule, b"Ar").count();
    atoms - rn - ar - y * 2 - 1
}

fn nb_atoms(mol: &[u8]) -> usize {
    mol.iter().filter(|c| c.is_ascii_uppercase()).count()
}