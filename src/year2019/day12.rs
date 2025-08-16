use std::simd::prelude::*;
use crate::util::{math::*, parser::*};

pub fn solve(input: &str) -> (i16, u64) {
    let n = input.iter_signed::<i16>().next_chunk::<12>().unwrap();
    let pos = i16x16::from_array([
        n[0], n[3], n[6], n[9], 
        n[1], n[4], n[7], n[10],
        n[2], n[5], n[8], n[11],
        0, 0, 0, 0
    ]);
    let vel = i16x16::splat(0);

    let p1 = part1(pos, vel);
    let p2 = part2(pos, vel);
    (p1, p2)
}

#[inline]
fn step(pos: &mut i16x16, vel: &mut i16x16) {
    let c0 = simd_swizzle!(*pos, [1, 2, 3, 0, 5, 6, 7, 4, 9, 10, 11, 8, 12, 13, 14, 15]);
    let c1 = simd_swizzle!(*pos, [2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 12, 13, 14, 15]);
    let c2 = simd_swizzle!(*pos, [3, 0, 1, 2, 7, 4, 5, 6, 11, 8, 9, 10, 12, 13, 14, 15]);
    *vel = *vel + (c0 - *pos).signum() + (c1 - *pos).signum() + (c2 - *pos).signum();
    *pos += *vel;
}

fn part1(mut pos: i16x16, mut vel: i16x16) -> i16 {
    for _ in 0..1000 {
        step(&mut pos, &mut vel);
    }
    let pos = pos.abs().to_array();
    let vel = vel.abs().to_array();
    let e: [i16; 4] = std::array::from_fn(|i| pos[i] + pos[i+4] + pos[i+8]);
    let f: [i16; 4] = std::array::from_fn(|i| vel[i] + vel[i+4] + vel[i+8]);
    e[0] * f[0] + e[1] * f[1] + e[2] * f[2] + e[3] * f[3]
}

fn part2(mut pos: i16x16, mut vel: i16x16) -> u64 {
    let zero = i16x16::splat(0);
    let mask1 = i16x16::from_array([i16::MIN, i16::MIN, i16::MIN, i16::MIN, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let mask2 = i16x16::from_array([0, 0, 0, 0, i16::MIN, i16::MIN, i16::MIN, i16::MIN, 0, 0, 0, 0, 0, 0, 0, 0]);
    let mask3 = i16x16::from_array([0, 0, 0, 0, 0, 0, 0, 0, i16::MIN, i16::MIN, i16::MIN, i16::MIN, 0, 0, 0, 0]);
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let mut counter = 0;

    while x == 0 || y == 0 || z == 0 {
        counter += 1;
        step(&mut pos, &mut vel);
        if vel & mask1 == zero {
            x = counter
        }
        if vel & mask2 == zero {
            y = counter
        }
        if vel & mask3 == zero {
            z = counter
        }
    }

    2 * x.lcm(y).lcm(z)
}