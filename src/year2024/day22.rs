use anyhow::*;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(u64, u32)> {
    let mut p1 = 0;
    let mut diff = Vec::with_capacity(1000);
    let mut prices = vec![0; 130321];
    let mut seen = vec![0u16; 130321];

    let numbers: Vec<_> = input.iter_unsigned::<u64>().collect();

    let mut iter = 1;

    for n in numbers {
        diff.clear();
        let mut m = n;
        let mut x = (m % 10) as u32;
        for _ in 0..2000 {
            m = next_secret(m);
            let y = (m % 10) as u32;
            diff.push((9 + y - x, y));
            x = y;
        }
        for &[(d1, _), (d2, _), (d3, _), (d4, p)] in diff.array_windows() {
            let index = (d1 * 19 * 19 * 19 + d2 * 19 * 19 + d3 * 19 + d4) as usize;
            if seen[index] != iter {
                prices[index] += p;
                seen[index] = iter;
            }
        }
        p1 += m;
        iter += 1;
    }

    let p2 = *prices.iter().max().unwrap();

    Ok((p1, p2))
}

#[inline]
fn next_secret(mut n: u64) -> u64 {
    n ^= n << 6;
    n &= 16777215;
    n ^= n >> 5;
    n &= 16777215;
    n ^= n << 11;
    n & 16777215
}