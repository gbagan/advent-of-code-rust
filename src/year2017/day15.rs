// part 1 can be parallelized using modular exponentiation
// no idea how to improve the execution time of part 2

use itertools::{iterate, Itertools};
use crate::util::{iter::*, parallel::*, parser::*, power};

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let (a, b) = input.iter_unsigned().next_tuple()?;
    let p1 = part1(a, b);
    let p2 = part2(a, b);
    Some((p1, p2))
}
#[inline]
fn next_a(a: &u64) -> u64 {
    a * 16_807 % 2_147_483_647
}

#[inline]
fn next_b(a: &u64) -> u64 {
    a * 48_271 % 2_147_483_647
}

fn nth_a(n: usize, x: u64) -> u64 {
    if n == 0 {
        x
    } else {
        (x * power(|a, b| (a * b) % 2_147_483_647, 16_807, n)) % 2_147_483_647
    }
}

fn nth_b(n: usize, x: u64) -> u64 {
    if n == 0 {
        x
    } else {
        (x * power(|a, b| (a * b) % 2_147_483_647, 48271, n)) % 2_147_483_647
    }
}


fn part1(a: u64, b: u64) -> usize {
    let n = 40_000_000usize / 64;
    (0..64).into_par_iter().map(|i| {
        let iter_a = iterate(nth_a(i*n, a), next_a);
        let iter_b = iterate(nth_b(i*n, b), next_b);
        iter_a.zip(iter_b).take(n).count_if(|(a, b)| a & 0xffff == b & 0xffff)
    }).sum()
}

fn part2(a: u64, b: u64) -> usize {
    let iter_a = iterate(a, next_a).filter(|&a| a & 3 == 0);
    let iter_b = iterate(b, next_b).filter(|&a| a & 7 == 0);
    iter_a.zip(iter_b).take(5_000_000).count_if(|(a, b)| a & 0xffff == b & 0xffff)
}