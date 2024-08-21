use crate::util::range::Range;
use itertools::Itertools;

struct ShiftRange {
    range: Range,
    shift: i64,
}

pub struct Input {
    seeds: Vec<i64>,
    maps: Vec<Vec<ShiftRange>>,
}

pub fn parse(input: &str) -> Option<Input> {
    let mut lines = input.lines();
    let (seeds, _) = lines.next_tuple()?;
    let seeds: Vec<i64> = seeds.split(' ').skip(1).filter_map(|s| s.parse().ok()).collect();
    
    let mut maps = vec!();
    while lines.next().is_some() {
        let mut map = vec!();
        while let Some(line) = lines.next() {
            if line.len() == 0 { break }
            let (destination, source, length) = line.split(' ').next_tuple()?;
            let destination: i64 = destination.parse().ok()?;
            let source: i64 = source.parse().ok()?;
            let length: i64 = length.parse().ok()?;
            let range = Range{lower: source, upper: source+length-1};
            map.push(ShiftRange {range, shift: destination - source});
        }
        maps.push(map)
    }
    Some(Input {seeds, maps})
}

fn step(seeds: &mut Vec<i64>, ranges: &Vec<ShiftRange>) {
    seeds.iter_mut().for_each(|seed| *seed =
        ranges
            .iter()
            .find_map(|r| r.range.contains(*seed).then_some(*seed + r.shift))
            .unwrap_or(*seed)
    )
}

pub fn part1(input: &Input) -> Option<i64> {
    let Input {seeds, maps} = input;
    let mut seeds = seeds.clone();
    for ranges in maps.iter() {
        step(&mut seeds, ranges);
    }
    seeds.iter().min().copied()
}

fn step2(seeds: &Vec<Range>, ranges: &Vec<ShiftRange>) -> Vec<Range> {
    let mut result = vec!(); 
    for &seed_range in seeds {
        for range in ranges {
            if let Some(intersection) = seed_range & range.range {
                result.push(intersection.shift(range.shift));
            }
        }
    }
    result
}

pub fn part2(input: &Input) -> Option<i64> {
    let Input {seeds, maps} = input;
    let mut seeds: Vec<_> = seeds.iter()
                                .tuples()
                                .map(|(&lower, &length)| Range {lower, upper: lower+length-1})
                                .collect();
    for ranges in maps {
        seeds = step2(&seeds, ranges);
    }
    seeds.iter().map(|seed| seed.lower).min()
}