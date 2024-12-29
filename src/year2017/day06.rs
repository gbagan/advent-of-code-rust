use itertools::iterate;
use crate::util::{iter::*, parser::*};

const SPREAD: [u64; 16] = [
    0x0000000000000000,
    0x1000000000000000,
    0x1100000000000000,
    0x1110000000000000,
    0x1111000000000000,
    0x1111100000000000,
    0x1111110000000000,
    0x1111111000000000,
    0x1111111100000000,
    0x1111111110000000,
    0x1111111111000000,
    0x1111111111100000,
    0x1111111111110000,
    0x1111111111111000,
    0x1111111111111100,
    0x1111111111111110,
];

pub fn solve(input: &str) -> (usize, usize) {
    let banks: u64 = input.iter_unsigned().fold(0, |acc, n: u64| (acc << 4) + n);

    let (i, j, _) = iterate(banks, step).find_duplicate().unwrap();
    (j, j - i)
}

fn step(banks: &u64) -> u64 {
    let banks = *banks;
    let (offset, max) = find_max(banks);
    (banks & 0xffff_ffff_ffff_fff0u64.rotate_left(offset)) + SPREAD[max as usize].rotate_left(offset)
}

#[inline(always)]
fn find_max(banks: u64) -> (u32, u64) {
    let mut mask = 0x8888_8888_8888_8888;
    let mut banks2 = banks;
    for _ in 0..4 {
        let mask2 = banks2 & mask;
        mask = if mask2 == 0 {mask} else {mask2};
        banks2 <<= 1;
    }
    let offset = 60 - mask.leading_zeros();
    (offset, banks >> offset & 0xf)
}

#[test]
fn find_max_test() {
    let b = 0x347_9DAE_3123_4567;
    assert_eq!(find_max(b), (32, 14));
}

#[test]
fn step_test() {
    let b = 0x2347_9DAE_3123_4567;
    let c = 0x3458_AEA0_4234_5678;
    assert_eq!(step(&b), c);
}