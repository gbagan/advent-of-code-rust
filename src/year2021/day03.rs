// assume there are 1000 numbers of 12 bits and no duplicate numbers

use std::simd::prelude::*;
use crate::util::{bits::*, iter::BoolIter};

const N: usize = 1000;

pub fn solve(input: &str) -> (u64, u32) {
    let mut input = input.as_bytes();
    
    assert_eq!(input.len(), 13*N); 

    let one = u8x16::splat(b'1');
    let mask = u8x16::from_array([
        0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff,
        0, 0, 0, 0,
    ]);

    let mut masks = [0u64; 64];
    let mut counts = i16x16::splat(0);
    
    while input.len() >= 16 {
        let t = u8x16::from_slice(input);
        let t = (t & mask).simd_eq(one);
        let value = t.to_bitmask().reverse_bits() >> 52;
        masks[(value >> 6) as usize] |= 1 << (value & 63);

        counts -= t.cast::<i16>().to_int();
        input = &input[13..];
    }

    let mut counts = counts.to_array();
    
    // last line
    let mut value = 0;
    for (i, &c) in input[input.len()-13..input.len()-1].iter().enumerate() {
        counts[i] += (c == b'1') as i16;
        value = value << 1 | (c == b'1') as u64;
    }
    masks[(value >> 6) as usize] |= 1 << (value & 63);

    // part 1

    let gamma: u64 =  counts[..12].iter().map(|&n| n >= (N/2) as i16).to_bitmask();
    let epsilon = !gamma & 0xfff;
    let p1 = gamma * epsilon;

    // part 2

    let mut numbers = [0; N];
    let mut i = 0;
    for (j, &mask) in masks.iter().enumerate() {
        for k in mask.bits() {
            numbers[i] = (j << 6 | k) as u32;
            i += 1;
        }
    }

    let split = search_split(&numbers, 0x800);
    let first = &numbers[..split];
    let last = &numbers[split..];

    let o2 = lookup::<true>(if split > 500 { first } else { last });
    let co2 = lookup::<false>(if split > 500 { last } else { first });
    let p2 = o2 * co2;

    (p1, p2)
}

fn search_split(numbers: &[u32], bit: u32) -> usize {
    let mut start = 0;
    let mut end = numbers.len();
    while start + 1 < end {
        let mid = start.midpoint(end);
        if numbers[mid] & bit == 0 {
            start = mid;
        } else {
            end = mid;
        }
    }
    end
}

fn lookup<const O2: bool>(mut numbers: &[u32]) -> u32 {
    let mut bit = 0x400;
    while numbers.len() > 1 {
        let split = search_split(numbers, bit);
        if (numbers.len() - split < split) == O2 {
            numbers = &numbers[..split];
		} else {
			numbers = &numbers[split..];
		}
        bit >>= 1;
    }
    numbers[0]
}