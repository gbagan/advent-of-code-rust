use anyhow::*;
use crate::util::{parser::*, range::Range};
use itertools::Itertools;

struct ShiftRange {
    range: Range<i64>,
    shift: i64,
}

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let mut lines = input.lines();
    let (line, _) = lines.next_tuple().context("Parse error")?;
    let seeds: Vec<i64> = line.iter_unsigned().collect();
    
    let mut maps = vec!();
    while lines.next().is_some() {
        let mut map = vec!();
        for line in lines.by_ref() {
            if line.is_empty() { break }
            let (destination, source, length) = line.iter_unsigned().next_tuple()
                                                            .context("Parse error")?;
            let range = Range{lower: source, upper: source+length-1};
            map.push(ShiftRange {range, shift: destination - source});
        }
        maps.push(map)
    }
    
    let p1 = part1(&seeds, &maps).context("Part 1: No solution found")?;
    let p2 = part2(&seeds, &maps).context("Part 2: No solution found")?;
    Ok((p1, p2))
}

fn step(seeds: &mut [i64], ranges: &[ShiftRange]) {
    seeds.iter_mut().for_each(|seed| *seed =
        ranges
            .iter()
            .find_map(|r| r.range.contains(*seed).then_some(*seed + r.shift))
            .unwrap_or(*seed)
    )
}

fn part1(seeds: &[i64], maps: &[Vec<ShiftRange>]) -> Option<i64> {
    let mut seeds = seeds.to_vec();
    for ranges in maps.iter() {
        step(&mut seeds, ranges);
    }
    seeds.iter().min().copied()
}

fn step2(seeds: &Vec<Range<i64>>, ranges: &Vec<ShiftRange>) -> Vec<Range<i64>> {
    let mut result = vec!(); 
    for &seed_range in seeds {
        for range in ranges {
            if let Some(intersection) = seed_range.intersection(&range.range) {
                result.push(intersection.shift(range.shift));
            }
        }
    }
    result
}

fn part2(seeds: &[i64], maps: &[Vec<ShiftRange>]) -> Option<i64> {
    let mut seeds: Vec<_> = seeds.iter()
                                .tuples()
                                .map(|(&lower, &length)| Range {lower, upper: lower+length-1})
                                .collect();
    for ranges in maps {
        seeds = step2(&seeds, ranges);
    }
    seeds.iter().map(|seed| seed.lower).min()
}