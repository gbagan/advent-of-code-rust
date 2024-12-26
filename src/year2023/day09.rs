// Pascal Triangle

use anyhow::*;
use crate::util::parser::*;

const N: usize = 21;

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        for (i, v) in line.iter_signed::<i64>().enumerate() {
            p1 += v * TRIANGLE[i];
            p2 += v * TRIANGLE[N-1-i];
        }
    }

    Ok((p1, p2))
}

const TRIANGLE: [i64; N+1] = {
    let n = N as i64;
    let mut triangle = [0; N+1];
    triangle[0] = 1;
    let mut c = 1;
    let mut i = 0;
    while i < n {
        c = (c * (i - n)) / (i + 1);
        triangle[i as usize + 1] = c;
        i += 1;
    };
    triangle
};