use anyhow::*;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(u64, u64)> {
    let (line1, line2) = input.split_once('\n')
        .ok_or_else(|| anyhow!("Parse error"))?;
    let times: Vec<_> = line1.iter_unsigned().collect();
    let distances: Vec<_> = line2.iter_unsigned().collect();
    let p1 = part1(&times, &distances);
    let p2 = part2(line1, line2);
    Ok((p1, p2))
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

pub fn part2(line1: &str, line2: &str) -> u64 {
    let time = line1.chars().filter(char::is_ascii_digit).collect::<String>().parse().unwrap();
    let distance = line2.chars().filter(char::is_ascii_digit).collect::<String>().parse().unwrap();
    solve_race(time, distance)
}