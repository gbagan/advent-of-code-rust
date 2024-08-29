use itertools::Itertools;

pub fn solve(input: &str) -> Option<(u64, u64)> {
    let (line1, line2) = input.lines().next_tuple()?;
    let times: Vec<_> = line1
                .split_ascii_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();
    let distances: Vec<_> = line2
                .split_ascii_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();
    let p1 = part1(&times, &distances);
    let p2 = part2(&times, &distances);
    Some((p1, p2))
}

fn solve_race(time: u64, distance: u64) -> u64 {
    let t = time as f64;
    let d = distance as f64;
    let delta = (t*t - 4.0*d).max(0.0).sqrt();
    let root1 = (t - delta) / 2.0;
    let root2 = (t + delta) / 2.0;
    root2.ceil() as u64 - root1 as u64 - 1
}

pub fn part1(times: &[u64], distances: &[u64]) -> u64 {
    times
        .iter()
        .zip(distances)
        .map(|(t, d)| solve_race(*t, *d)).product()
}

pub fn part2(times: &[u64], distances: &[u64]) -> u64 {
    let time = times.iter().map(|t| t.to_string()).join("").parse().unwrap();
    let distance = distances.iter().map(|t| t.to_string()).join("").parse().unwrap();
    solve_race(time, distance)
}