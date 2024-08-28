use itertools::Itertools;

pub fn parse(input: &str) -> Option<(Vec<u64>, Vec<u64>)> {
    let (line1, line2) = input.lines().next_tuple()?;
    let times = line1
                .split_ascii_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();
    let distances = line2
                .split_ascii_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();
    Some((times, distances))
}

fn solve_race(time: u64, distance: u64) -> u64 {
    let t = time as f64;
    let d = distance as f64;
    let delta = (t*t - 4.0*d).max(0.0).sqrt();
    let root1 = (t - delta) / 2.0;
    let root2 = (t + delta) / 2.0;
    root2.ceil() as u64 - root1 as u64 - 1
}

pub fn part1((times, distances): &(Vec<u64>, Vec<u64>)) -> Option<u64> {
    Some(times
        .iter()
        .zip(distances)
        .map(|(t, d)| solve_race(*t, *d)).product()
    )
}

pub fn part2((times, distances): &(Vec<u64>, Vec<u64>)) -> Option<u64> {
    let time = times.iter().map(|t| t.to_string()).join("").parse().ok()?;
    let distance = distances.iter().map(|t| t.to_string()).join("").parse().ok()?;
    Some(solve_race(time, distance))
}