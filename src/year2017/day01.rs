use std::simd::prelude::*;

pub fn solve(input: &str) -> (u32, u32) {
    let bytes = input.trim().as_bytes();
    let p1 = part1(bytes);
    let p2 = part2(bytes);
    (p1, p2)
}

fn part1(bytes: &[u8]) -> u32 {
    let bytes1 = &bytes[1..];
    let bytes2 = bytes;
    let mut total = compare_bytes(bytes1, bytes2);
    if bytes[0] == bytes[bytes.len()-1] {
        total += (bytes[0] - b'0') as u32
    }

    total
}

fn part2(bytes: &[u8]) -> u32 {
    let bytes1 = &bytes[..bytes.len()/2];
    let bytes2 = &bytes[bytes.len()/2..];
    2 * compare_bytes(bytes1, bytes2)
}


fn compare_bytes(bytes1: &[u8], bytes2: &[u8]) -> u32 {
    let mut total = u8x32::splat(0);
    let n = bytes1.len() & !31;
    for i in (0..n).step_by(32) {
        total += compare(u8x32::from_slice(&bytes1[i..]), u8x32::from_slice(&bytes2[i..]));
    }

    let mut total = total.cast::<u16>().reduce_sum() as u32;
    for i in bytes1.len() & !31 .. bytes1.len() {
        if bytes1[i] == bytes2[i] {
            total += (bytes1[i] - b'0') as u32
        }
    }

    total
}


#[inline(always)]
fn compare(s1: u8x32, s2: u8x32) -> u8x32 {
    let mask = s1.simd_eq(s2);
    mask.select(s1 - u8x32::splat(b'0'), u8x32::splat(0))
}