use std::ops::Range;
use std::simd::prelude::*;

pub fn solve(input: &str) -> (u32, u32) {
    let grid = input.as_bytes();
    let p1 = part1(&grid);
    let p2 = part2(&grid);
    (p1, p2)
}


const RANGE1: Range<usize> = 0..64;
const RANGE2: Range<usize> = 61..125;
const RANGE3: Range<usize> = 76..140;
const MASK2: u64 = u64::MAX - 7 - (7 << 61);
const MASK3: u64 = u64::MAX - (1u64 << 46) + 1;

macro_rules! encode {
    ($x:pat, $m:pat, $a:pat, $s:pat, $slice:expr) => {
        let line = u8x64::from_slice($slice);
        let $x = line.simd_eq(u8x64::splat(b'X')).to_bitmask();
        let $m = line.simd_eq(u8x64::splat(b'M')).to_bitmask();
        let $a = line.simd_eq(u8x64::splat(b'A')).to_bitmask();
        let $s = line.simd_eq(u8x64::splat(b'S')).to_bitmask();
    };
}


fn part1(grid: &[u8]) -> u32 {
    let mut p1 = 0;
    
    let line = &grid[..140];
    encode!(mut x11, m11, a11, mut s11, &line[RANGE1]);
    p1 += count_horizontal::<1>(x11, m11, a11, s11);
    encode!(mut x12, m12, a12, mut s12, &line[RANGE2]);
    p1 += count_horizontal::<2>(x12, m12, a12, s12);
    encode!(mut x13, m13, a13, mut s13, &line[RANGE3]);
    p1 += count_horizontal::<3>(x13, m13, a13, s13);

    let line = &grid[141..141+140];
    encode!(mut x21, mut m21, mut a21, mut s21, &line[RANGE1]);
    p1 += count_horizontal::<1>(x21, m21, a21, s21);
    encode!(mut x22, mut m22, mut a22, mut s22, &line[RANGE2]);
    p1 += count_horizontal::<2>(x22, m22, a22, s22);
    encode!(mut x23, mut m23, mut a23, mut s23, &line[RANGE3]);
    p1 += count_horizontal::<3>(x23, m23, a23, s23);


    let line = &grid[2*141..2*141+140];
    encode!(mut x31, mut m31, mut a31, mut s31, &line[RANGE1]);
    p1 += count_horizontal::<1>(x31, m31, a31, s31);
    encode!(mut x32, mut m32, mut a32, mut s32, &line[RANGE2]);
    p1 += count_horizontal::<2>(x32, m32, a32, s32);
    encode!(mut x33, mut m33, mut a33, mut s33, &line[RANGE3]);
    p1 += count_horizontal::<3>(x33, m33, a33, s33);

    for line in grid[3 * 141..].array_chunks::<141>() {
        encode!(x1, m1, a1, s1, &line[RANGE1]);
        p1 += count_horizontal::<1>(x1, m1, a1, s1);
        p1 += count_vertical::<1>(x11, m21, a31, s1, s11, a21, m31, x1);
        p1 += count_diagonal::<1>(x11, m21, a31, s1, x1, m31, a21, s11);

        encode!(x2, m2, a2, s2, &line[RANGE2]);
        p1 += count_horizontal::<2>(x2, m2, a2, s2);
        p1 += count_vertical::<2>(x12, m22, a32, s2, s12, a22, m32, x2);
        p1 += count_diagonal::<2>(x12, m22, a32, s2, x2, m32, a22, s12);
        
        encode!(x3, m3, a3, s3, &line[RANGE3]);
        p1 += count_horizontal::<3>(x3, m3, a3, s3);
        p1 += count_vertical::<3>(x13, m23, a33, s3, s13, a23, m33, x3);
        p1 += count_diagonal::<3>(x13, m23, a33, s3, x3, m33, a23, s13);

        (x11, x21, x31) = (x21, x31, x1);
        (m21, m31) = (m31, m1);
        (a21, a31) = (a31, a1);
        (s11, s21, s31) = (s21, s31, s1);

        (x12, x22, x32) = (x22, x32, x2);
        (m22, m32) = (m32, m2);
        (a22, a32) = (a32, a2);
        (s12, s22, s32) = (s22, s32, s2);

        (x13, x23, x33) = (x23, x33, x3);
        (m23, m33) = (m33, m3);
        (a23, a33) = (a33, a3);
        (s13, s23, s33) = (s23, s33, s3);
    }

    p1
}

#[inline]
fn count_horizontal<const N: usize>(mut x: u64, m: u64, a: u64, mut s: u64) -> u32 {
    if N==3 {
        x &= MASK3;
        s &= MASK3;
    }
    (x & (m << 1) & (a << 2) & (s << 3) | s & (a << 1) & (m << 2) & (x << 3)).count_ones()
}

#[inline]
fn count_vertical<const N: usize>(
    x1: u64, m1: u64, a1: u64, mut s1: u64, 
    s2: u64, a2: u64, m2: u64, mut x2: u64) -> u32
{
    if N == 2 {
        s1 &= MASK2;
        x2 &= MASK2;
    } else if N == 3 {
        s1 &= MASK3;
        x2 &= MASK3;
    }

    (x1 & m1 & a1 & s1 | x2 & m2 & a2 & s2).count_ones()
}

#[inline]
fn count_diagonal<const N: usize>(
    mut x1: u64, m1: u64, a1: u64, mut s1: u64,
    mut x2: u64, m2: u64, a2: u64, mut s2: u64
) -> u32
{
    if N == 3 {
        x1 &= MASK3;
        x2 &= MASK3;
        s1 &= MASK3;
        s2 &= MASK3;
    }

    let diag1 = x1 & (m1 << 1) & (a1 << 2) & (s1 << 3) | s2 & (a2 << 1) & (m2 << 2) & (x2 << 3);
    let diag2 = (x1 << 3) & (m1 << 2) & (a1 << 1) & s1 | (s2 << 3) & (a2 << 2) & (m2 << 1) & x2;

    diag1.count_ones() + diag2.count_ones()
}

fn part2(input: &[u8]) -> u32 {
    let mut rest = input;
    let mut p2 = 0;
    while rest.len() >= 2 * 141 + 66 {
        let slice = &rest[..2 * 141 + 66];
        let top_left = u8x64::from_slice(&slice);
        let top_right = u8x64::from_slice(&slice[2..]);
        let center = u8x64::from_slice(&slice[142..]);
        let bottom_left = u8x64::from_slice(&slice[141*2..]);
        let bottom_right = u8x64::from_slice(&slice[141*2..]);

        let a = center.simd_eq(u8x64::splat(b'A'));
        let b = (top_left ^ bottom_right).simd_eq(u8x64::splat(b'M' ^ b'S'));
        let c = (top_right ^ bottom_left).simd_eq(u8x64::splat(b'M' ^ b'S'));
        
        p2 += (a & b & c).to_bitmask().count_ones();

        rest = &rest[64..];
    }

    p2
}