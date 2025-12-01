use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, u32) {
    let input = input.trim().as_bytes();
    let amounts = input.iter_unsigned::<u32>();
    let directions = input.iter().filter(|&&c| c.is_ascii_uppercase());

    let mut dial = 1_000_000_050;
    let mut p1 = 0;
    let mut p2 = 0;

    for (&dir, amount) in directions.zip(amounts) {
        if dir == b'L' {
            let next = dial - amount;
            p2 += (dial - 1) / 100 - (next - 1) / 100;
            dial = next;
        } else {
            let next = dial + amount;
            p2 += next / 100 - dial / 100;
            dial = next;
        };
        p1 += (dial % 100 == 0) as u32;
    }

    (p1, p2)
}