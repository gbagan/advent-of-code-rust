use crate::util::{iter::*, math::*, parser::*};

pub fn solve(input: &str) -> (u32, u32) {
    let mut pairs: Vec<(u32, u32)> = input.iter_unsigned().tuples().collect();
    pairs.sort_unstable_by_key(|p| p.1);
    let p1 = part1(&pairs);
    let p2 = part2(&pairs);
    (p1, p2)
}

fn part1(pairs: &[(u32, u32)]) -> u32 {
    pairs
        .iter()
        .filter(|&&(depth, range)| caught(depth, range))
        .map(|(depth, range)| depth * range)
        .sum()
}

#[inline]
fn caught(depth: u32, range: u32) -> bool {
    depth.is_multiple_of((range-1)*2)
}

fn part2(pairs: &[(u32, u32)]) -> u32 {
    let mut lcm = 1;
    let mut current_sieve = vec!(1);
    let mut next_sieve = Vec::new();

    for (depth, range) in pairs {
        let period = 2 * (range - 1);
        let next_lcm = lcm.lcm(period);
        for i in (0..next_lcm).step_by(lcm as usize) {
            for j in &current_sieve {
                if !(depth + i + j).is_multiple_of(period) {
                    next_sieve.push(i + j);
                }
            }
        }
        lcm = next_lcm;
        std::mem::swap(&mut current_sieve, &mut next_sieve);
        next_sieve.clear();
    }
    
    current_sieve[0]
}

