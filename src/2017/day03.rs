use std::time::Instant;
use std::collections::HashMap;
use std::str::FromStr;
use aoc::coord::Coord;

fn part1(n: i64) -> i64 {
    let n = n - 1;
    let cycle = ((n as f64).sqrt().floor() as i64 + 1) / 2;
    let length = 8 * cycle;
    let first = (2 * cycle - 1).pow(2); 
    let sector = 4 * (n - first) / length;
    let offset = n - first - sector * length / 4;
    (cycle - offset - 1).abs() + cycle
}

fn fill(grid: &HashMap<Coord,u64>, c: &Coord) -> u64 {
    c.surrounding().iter().filter_map(|a| grid.get(a)).sum()
}

fn part2(n: u64) -> u64 {
    let mut grid = HashMap::new();
    let mut c = Coord::origin();
    grid.insert(c, 1);
    let mut k = 1;
    loop {
        for (steps, dir) in [(k, Coord::east()), (k, Coord::north()), (k+1, Coord::west()), (k+1, Coord::south())] {
            for _ in 0..steps {
                c = c + dir;
                let r = fill(&grid, &c);
                println!("{:?} {}", c, r);
                if r > n {
                    return r;
                }
                grid.insert(c, r);
            }
        }
        k += 2;
    }
}

fn main() {
    let input = include_str!("../../inputs/2017/03");
    match u64::from_str(input) {
        Err(_) => println!("parsing error"),
        Ok(n) => {
            let start = Instant::now();
            let p1 = part1(n as i64);
            let p2 = part2(n);
            let end = start.elapsed().as_micros();

            println!("Part 1: {}", p1);
            println!("Part 2: {}", p2);
            println!("Time: {} μs", end);
        }
    }
}