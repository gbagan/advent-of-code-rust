use crate::util::{math::chinese_remainder3, parser::*};

pub fn solve(input: &str) -> (i64, i64) {
    let (line1, line2) = input.trim().split_once('\n').unwrap();
    let earliest = line1.to_unsigned::<i64>();
    let periods: Vec<_> = line2.split(',').map(|w| {
        if w == "x" { None } else { Some(w.to_unsigned::<i64>()) }
    }).collect();

    let (id, minutes) = periods
        .iter()
        .filter_map(|p| p.map(|p| ((-earliest).rem_euclid(p), p)))
        .min_by_key(|p| p.0).unwrap();

    let p1 = id * minutes;

    let pairs: Vec<_> = periods
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.map(|p| (-(i as i64), p)))
        .collect();
    let p2 = chinese_remainder3(&pairs).unwrap().0;

    (p1, p2)
}