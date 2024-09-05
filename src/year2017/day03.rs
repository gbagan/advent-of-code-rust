use std::collections::HashMap;
use crate::util::coord::Coord;

type Point = Coord<i32>;

pub fn solve(input: &str) -> Option<(i32, i32)> {
    let n = input.trim().parse().ok()?;
    Some((part1(n), part2(n)))
}

fn part1(n: i32) -> i32 {
    let n = n - 1;
    if n <= 0 {
        return 0;
    }
    let cycle = (n as f64).sqrt().floor() as i32;
    let start = n - cycle * cycle;
    let period = (cycle + 1) & !1;
    (cycle+1)/2 + (start % period).abs_diff(cycle/2) as i32
}

fn fill(grid: &HashMap<Point,i32>, c: &Point) -> i32 {
    c.surrounding().iter().filter_map(|a| grid.get(a)).sum()
}

fn part2(n: i32) -> i32 {
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
    let output: Vec<i32> = (1..27).map(|v| part1(v)).collect();
    let res = vec!(0, 1, 2, 1, 2, 1, 2, 1, 2, 3, 2, 3, 4, 3, 2, 3, 4, 3, 2, 3, 4, 3, 2, 3, 4, 5);
    assert_eq!(output, res);
}