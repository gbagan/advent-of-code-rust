use std::collections::HashMap;
use crate::util::coord::Coord;

pub fn parse(input: &str) -> Option<i32> {
    input.trim().parse().ok()
}

// todo: can be simplified
pub fn part1(n: &i32) -> Option<i32> {
    let n = *n - 1;
    if n <= 0 {
        return Some(0);
    }
    let cycle = (n as f64).sqrt().floor() as i32;
    let start = n - cycle * cycle;
    let period = (cycle + 1) & !1;
    Some((cycle+1)/2 + (start % period).abs_diff(cycle/2) as i32)
}

fn fill(grid: &HashMap<Coord,i32>, c: &Coord) -> i32 {
    c.surrounding().iter().filter_map(|a| grid.get(a)).sum()
}

pub fn part2(n: &i32) -> Option<i32> {
    let mut grid = HashMap::new();
    let mut c = Coord::ORIGIN;
    grid.insert(c, 1);
    let mut k = 1;
    loop {
        for (steps, dir) in [(k, Coord::EAST), (k, Coord::NORTH), (k+1, Coord::WEST), (k+1, Coord::WEST)] {
            for _ in 0..steps {
                c += dir;
                let r = fill(&grid, &c);
                if r > *n {
                    return Some(r);
                }
                grid.insert(c, r);
            }
        }
        k += 2;
    }
}

#[test]
fn part1_test() {
    let output: Vec<i32> = (1..=26).filter_map(|v| part1(&v)).collect();
    let res = vec!(0, 1, 2, 1, 2, 1, 2, 1, 2, 3, 2, 3, 4, 3, 2, 3, 4, 3, 2, 3, 4, 3, 2, 3, 4, 5);
    assert_eq!(output, res);
}