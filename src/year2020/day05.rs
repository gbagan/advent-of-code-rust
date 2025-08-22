use std::simd::prelude::*;

pub fn solve(input: &str) -> (u64, u64) {
    let input = input.as_bytes();
    let mut slice = input;
    let zero = u8x16::splat(0);
    let four = u8x16::splat(4);

    let mut p1 = 0;
    let mut xor = 0;
    let mut min = u64::MAX;
    let mut max = 0;

    while slice.len() >= 16 {
        let v = u8x16::from_slice(slice);
        let n = (v.reverse() & four).simd_eq(zero).to_bitmask() >> 6;
        p1 = p1.max(n);
        xor ^= n;
        min = min.min(n);
        max = max.max(n);
        slice = &slice[11..];
    }
    // last line
    let v = u8x16::from_slice(&input[input.len()-17..]);
    
    let n = (v.reverse() & four).simd_eq(zero).to_bitmask() & 1023;
    p1 = p1.max(n);
    xor ^= n;
    min = min.min(n);
    max = max.max(n);

    let p2 = xor ^ cumulative_xor(min-1) ^ cumulative_xor(max);

    (p1, p2)
}

// 1 ^ 2 ^ ... ^ n
fn cumulative_xor(n: u64) -> u64 {
    match n & 3 {
        0 => n,
        1 => 1,
        2 => n + 1,
        _ => 0,
    }
}