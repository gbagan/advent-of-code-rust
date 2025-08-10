use num_integer::Integer;
use crate::util::coord::*;
use std::cmp::Ordering;

type Point = Coord<i32>;

pub fn solve(input: &str) -> (i32, i32) {
    let input = input.as_bytes();
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let height = input.len() / (width + 1);
    let mut asteroids = Vec::new();
    for j in 0..height {
        for i in 0..width {
            if input[j * (width + 1) + i] == b'#' {
                asteroids.push(Point::new(i as i32, j as i32));
            }
        }
    }

    let (station_index, p1) = part1(&asteroids, width as i32, height as i32);
    let p2 = part2(&mut asteroids, station_index);
    
    (p1, p2)
}

fn part1(asteroids: &[Point], width: i32, height: i32) -> (usize, i32) {
    let mut seen = vec![0; (4 * width * height) as usize];
    let mut visible = vec![0; asteroids.len()];
    for i in 0..asteroids.len() - 1 {
        for j in i+1..asteroids.len() {
            let mut delta = asteroids[j] - asteroids[i];
            let gcd = delta.x.gcd(&delta.y).abs();
            delta.x /= gcd;
            delta.y /= gcd;
            let index = (2 * width * (delta.y + height) + delta.x + width) as usize;
            if seen[index] <= i {
                seen[index] = i+1;
                visible[i] += 1;
                visible[j] += 1;
            }
        }
    }
    let (i, visible) = visible.iter().enumerate().max_by_key(|p| p.1).unwrap();
    (i, *visible)
}

fn part2(asteroids: &mut Vec<Point>, station_index: usize) -> i32 {
    let station = asteroids.swap_remove(station_index);
    
    for point in asteroids.iter_mut() {
        *point -= station;
    }

    asteroids.sort_unstable_by(|&a, &b| clockwise(a, b));

    let mut groups = Vec::with_capacity(asteroids.len());
    let mut group_position: u32 = 0;
    groups.push((group_position, 0));
    for i in 1..asteroids.len() {
        if angle(asteroids[i-1], asteroids[i]) == Ordering::Equal {
            group_position += 1;
        } else {
            group_position = 0;
        }
        groups.push((group_position, i));
    }

    let idx = groups.select_nth_unstable(199).1.1;
    let asteroid200 = station + asteroids[idx];
    
    100 * asteroid200.x + asteroid200.y
}

fn clockwise(p1: Point, p2: Point) -> Ordering {
    quadrant(p1).cmp(&quadrant(p2))
        .then(angle(p1, p2))
        .then(norm(p1).cmp(&norm(p2)))
}

fn quadrant(point: Point) -> u32 {
    match (point.x >= 0, point.y >= 0) {
        (true, false) => 0,
        (true, true) => 1,
        (false, true) => 2,
        (false, false) => 3,
    }
}

fn angle(p1: Point, p2: Point) -> Ordering {
    (p1.y * p2.x).cmp(&(p1.x * p2.y))
}

fn norm(point: Point) -> i32 {
    point.x * point.x + point.y * point.y
}