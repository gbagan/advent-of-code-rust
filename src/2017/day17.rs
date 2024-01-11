use aoc::aoc;
use std::str::FromStr;

fn part1(n: usize) -> usize {
    let mut v = vec!(0);
    let mut pos = 0;
    for i in 1..=2017 {
        pos = 1 + (pos + n) % i;
        v.insert(pos, i);
    }
    v[(pos+1)%2018]
}

fn part2(n: usize) -> usize {
    let mut val_after_0 = 0;
    let mut pos = 0;
    for i in 1..=50_000_000 {
        pos = 1 + (pos + n) % i;
        if pos == 1 {
            val_after_0 = i;
        }
    }
    val_after_0
}

fn main() {
    let input = include_str!("../../inputs/2017/17");
    match u64::from_str(input) {
        Err(_) => println!("parsing error"),
        Ok(n) => {
            aoc(|| (part1(n as usize), part2(n as usize)))
        }
    }
}