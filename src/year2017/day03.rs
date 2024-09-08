use anyhow::*;
use std::collections::HashMap;
use crate::util::{coord::Coord, parser::*};

type Point = Coord<i32>;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let n = input.next_unsigned()?;
    let p1 = part1(n);
    let p2 = part2(n);
    Ok((p1, p2))
}

fn part1(n: u32) -> u32 {
    if n <= 1 {
        return 0;
    }
    let n = n - 1;
    let cycle = (n as f64).sqrt().floor() as u32;
    let start = n - cycle * cycle;
    let period = (cycle + 1) & !1;
    (cycle+1)/2 + (start % period).abs_diff(cycle/2)
}

fn fill(grid: &HashMap<Point,u32>, c: &Point) -> u32 {
    c.surrounding().iter().filter_map(|a| grid.get(a)).sum()
}

fn part2(n: u32) -> u32 {
    let mut grid = HashMap::new();
    let mut c = Point::ORIGIN;
    grid.insert(c, 1);
    let mut k = 1;
    loop {
        for (steps, dir) in [(k, Point::EAST), (k, Point::NORTH), (k+1, Point::WEST), (k+1, Point::WEST)] {
            for _ in 0..steps {
                c += dir;
                let r = fill(&grid, &c);
                if r > n {
                    return r;
                }
                grid.insert(c, r);
            }
        }
        k += 2;
    }
}

#[test]
fn part1_test() {
    let output: Vec<u32> = (1..27).map(|v| part1(v)).collect();
    let res = vec!(0, 1, 2, 1, 2, 1, 2, 1, 2, 3, 2, 3, 4, 3, 2, 3, 4, 3, 2, 3, 4, 3, 2, 3, 4, 5);
    assert_eq!(output, res);
}