use std::cmp::min;
use crate::util::parser::*;

pub struct Reindeer {
    speed: u32,
    duration: u32,
    cycle: u32,
}

pub fn solve(input: &str) -> (u32, u16) {
    let reindeers: Vec<_> = input
                            .iter_unsigned()
                            .array_chunks()
                            .map(|[speed, duration, rest]| Reindeer{speed, duration, cycle: duration + rest})
                            .collect();
    let p1 = reindeers.iter().map(|r| distance(r, 2503)).max().unwrap_or(0);
    let p2 = part2(&reindeers);
    (p1, p2)
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


pub fn part2(reindeers: &[Reindeer]) -> u16 {
    let n = reindeers.len();
    let mut distances: Vec<u32> = vec![0; n]; 
    let mut scores: Vec<u16> = vec![0; n];
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
    scores.iter().max().copied().unwrap_or(0)
}