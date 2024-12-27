use crate::util::parser::*;

pub fn solve(input: &str) -> (u64, u64) {
    let (line1, line2) = input.split_once('\n').unwrap();
    let line1 = &line1[10..];
    let line2 = &line2[10..];
    let p1 = part1(line1, line2);
    let p2 = part2(line1, line2);
    (p1, p2)
}

pub fn part1(line1: &str, line2: &str) -> u64 {
    line1
        .iter_unsigned::<u64>()
        .zip(line2.iter_unsigned::<u64>())
        .map(|(t, d)| solve_race(t, d))
        .product()
}

pub fn part2(line1: &str, line2: &str) -> u64 {
    let mut time = 0;
    for c in line1[10..].bytes().filter(u8::is_ascii_digit) {
        time = time * 10 + (c - b'0') as u64;
    }
    let mut distance = 0;
    for c in line2[10..].bytes().filter(u8::is_ascii_digit) {
        distance = distance * 10 + (c - b'0') as u64;
    }
    solve_race(time, distance)
}

fn solve_race(time: u64, distance: u64) -> u64 {
    let t = time as f64;
    let d = distance as f64;
    let delta = (t*t - 4.0*d).max(0.0).sqrt();
    let root1 = (t - delta) / 2.0;
    let root2 = (t + delta) / 2.0;
    root2.ceil() as u64 - root1 as u64 - 1
}