use std::mem;
use num_integer::Integer;

fn parse_line(line: &str) -> Option<(i64, i64)> {
    let (depth, range) = line.split_once(": ")?;
    let depth = depth.parse().ok()?;
    let range = range.parse().ok()?;
    Some((depth, range))
}


pub fn solve(input: &str) -> Option<(i64, i64)> {
    let mut pairs: Vec<_> = input.lines().filter_map(parse_line).collect();
    pairs.sort_unstable_by_key(|p| p.1);
    let p1 = part1(&pairs);
    let p2 = part2(&pairs)?;
    Some((p1, p2))
}

#[inline]
fn caught(depth: i64, range: i64) -> bool {
    depth % ((range-1)*2) == 0
}

fn part1(pairs: &[(i64, i64)]) -> i64 {
    pairs
        .iter()
        .filter(|(depth, range)| caught(*depth, *range))
        .map(|(depth, range)| depth * range)
        .sum() 
}

fn part2(pairs: &[(i64, i64)]) -> Option<i64> {
    let mut forbiddens: Vec<(i64, Vec<i64>)> = vec!();
    let mut prev_range = 0;
    for &(depth, range) in pairs {
        let period = 2 * (range - 1);
        let depth = depth % period; 
        if range == prev_range {
            let last = forbiddens.len()-1; 
            forbiddens[last].1.push(depth);
        } else {
            forbiddens.push((period, vec!(depth)));
        }
        prev_range = range;
    }

    let mut lcm = 1;
    let mut current_sieve = vec!(1);
    let mut next_sieve = Vec::new();

    for (period, forbidden) in forbiddens {
        let next_lcm = lcm.lcm(&period);
        for i in (0..next_lcm).step_by(lcm as usize) {
            for j in &current_sieve {
                if !forbidden.contains(&(-i - j).mod_floor(&period)) {
                    next_sieve.push(i + j);
                }
            }
        }
        lcm = next_lcm;
        mem::swap(&mut current_sieve, &mut next_sieve);
        next_sieve.clear();
    }
    
    current_sieve.first().copied()
}