use std::cmp::min;
use itertools::Itertools;

pub struct Reindeer {
    speed: u32,
    duration: u32,
    cycle: u32,
}

fn parse_reindeer(input: &str) -> Option<Reindeer> {
    let (speed, _, _, duration, _, _, _, _, _, _, rest) = input.split(' ').skip(3).next_tuple()?;
    let speed = speed.parse().ok()?;
    let duration = duration.parse().ok()?;
    let rest: u32 = rest.parse().ok()?;
    Some(Reindeer { speed, duration, cycle: duration + rest})
}

pub fn parse(input: &str) -> Option<Vec<Reindeer>> {
    Some(input.lines().filter_map(parse_reindeer).collect())
}


fn distance(reindeer: &Reindeer, total_duration: u32) -> u32 {
    let Reindeer {speed, duration, cycle} = reindeer;
    let q = total_duration / cycle;
    let r = total_duration % cycle;
    q * duration * speed + min(r, *duration) * speed
}

fn step(reindeer: &Reindeer, i: u32) -> u32 {
    let Reindeer {speed, duration, cycle} = *reindeer;
    if i % cycle < duration {speed} else {0}
}

pub fn part1(reindeers: &[Reindeer]) -> Option<u32> {
    reindeers.iter().map(|r| distance(r, 2503)).max()
}

pub fn part2(reindeers: &[Reindeer]) -> Option<u16> {
    let n = reindeers.len();
    let mut distances: Vec<u32> = reindeers.iter().map(|_| 0).collect(); 
    let mut scores: Vec<u16> = reindeers.iter().map(|_| 0).collect();
    for i in 0..2503 {
        for j in 0..n {
            distances[j] += step(&reindeers[j], i);
        }
        let best_distance = *distances.iter().max().unwrap_or(&0);
        for j in 0..n {
            if distances[j] == best_distance {
                scores[j] += 1;
            }
        }
    }
    scores.iter().max().copied()
}