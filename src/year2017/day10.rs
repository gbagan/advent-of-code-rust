use crate::util::{knothash::{knothash, reverse}, parser::*};
use std::fmt::Write;

pub fn solve(input: &str) -> (u64, String) {
    let input = input.trim();
    let lengths: Vec<_> = input.iter_unsigned().collect();
    let knot = reverse(&lengths, 1);
    let p1 = knot[0] as u64 * knot[1] as u64;
    let p2 = part2(input);
    (p1, p2)
}

fn part2(input: &str) -> String {
    let dense_hash = knothash(input);
    let mut output = String::new();
    for x in dense_hash {
        let _ = write!(&mut output, "{x:02x}");
    }
    output
}