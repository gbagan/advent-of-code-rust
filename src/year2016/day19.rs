use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, u32) {
    let n = input.try_unsigned::<u32>().unwrap();
    let p1 = part1(n);
    let p2 = part2(n);
    (p1, p2)
}

pub fn part1(n: u32) -> u32 {
    2 * (n - (1 << (31 - n.leading_zeros()))) + 1
}

pub fn part2(n: u32) -> u32 {
    let mut k = 1;
    while k < n {
        k *= 3;
    }
    n - k / 3
}