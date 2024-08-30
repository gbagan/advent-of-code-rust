use itertools::Itertools;

use crate::util::parser::*;
use crate::util::permutation::Permutation;
use crate::util::power;

// a dance is a tuple of (p, q)
// p represents the permutation implied by Spin and Exchange
// q represents the permutation implied by Partner
type Dance = (Permutation, Permutation);

pub fn solve(input: &str) -> Option<(String, String)> {
    let mut perm1 = Permutation::one(16);
    let mut perm2 = Permutation::one(16);
    let mut offset = 0;

    let mut numbers = input.iter_unsigned::<usize>();
    let mut symbols = input.bytes().filter(u8::is_ascii_lowercase);

    while let Some(c) = symbols.next() {
        match c {
            b's' => offset += 16 - numbers.next().unwrap(),
            b'x' => {
                let (a, b) = numbers.next_tuple().unwrap();
                perm1.indices.swap((a + offset) % 16, (b + offset) % 16);
            }
            b'p' => {
                let (a, b) = symbols.next_tuple().unwrap();
                perm2.indices.swap((a - b'a') as usize, (b - b'a') as usize);
            }
            _ => panic!("invalid character {c}")
        }
    }
    perm1.indices.rotate_left(offset % 16);
    let dance = (perm1, perm2.inv());

    let programs = b"abcdefghijklmnop";
    //let dance = perform_dance(&moves);
    let p1 = (&dance.1 >> &dance.0).apply(programs);
    let p1: String = String::from_utf8(p1).unwrap();
    
    let pdance = power(compose_dance, dance.clone(), 1_000_000_000);
    let p2 = (&pdance.1 >> &pdance.0).apply(programs);
    let p2 = String::from_utf8(p2).unwrap();
    Some((p1, p2))
}

fn compose_dance(d1: &Dance, d2: &Dance) -> Dance {
    (&d1.0 >> &d2.0, &d1.1 >> &d2.1)
}

