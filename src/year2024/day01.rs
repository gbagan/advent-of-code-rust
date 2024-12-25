// https://lemire.me/blog/2022/01/21/swar-explained-parsing-eight-digits/

use anyhow::*;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let mut list1 = Vec::with_capacity(1000);
    let mut list2 = Vec::with_capacity(1000);
    
    for line in input.as_bytes().array_chunks::<14>() {
        let x = parse_5first(read_u64(&line[..8]));
        let y = parse_5last(read_u64(&line[5..13]));

        list1.push(x);
        list2.push(y);
    }
    
    radsort::sort(&mut list1);
    radsort::sort(&mut list2);

    let p1 = list1.iter().zip(list2.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum();
 
    let n = list2.len();
    let mut p2 = 0;
    let mut i = 0;

    for x in list1 {
        while i < n && list2[i] < x {
            i += 1;
        }
        let mut counter = 0;
        while i < n && list2[i] == x {
            counter += 1;
            i += 1;
        }
        p2 += x * counter;
    }

    Ok((p1, p2))
}

#[inline]
fn parse_5first(n: u64) -> u32 {
    parse_8digits((n << 24) | 0x303030)
}

#[inline]
fn parse_5last(n: u64) -> u32 {
    parse_8digits((n & 0xFFFFFFFFFF000000) | 0x303030)
}

#[inline]
fn read_u64(s: &[u8]) -> u64 {
    u64::from_le_bytes(s.try_into().unwrap())
}

#[inline]
fn parse_8digits(mut val: u64) -> u32 {
    const MASK: u64 = 0xFF | (0xFF << 32);
    const MUL1: u64 = 100 + (1000000 << 32);
    const MUL2: u64 = 1 + (10000 << 32);

    val -= 0x3030303030303030;
    val = (val * 10) + (val >> 8);
    val = ((val & MASK).wrapping_mul(MUL1) + ((val >> 16) & MASK).wrapping_mul(MUL2)) >> 32;

    val as u32
}