use std::time::Instant;
use aoc::iter::AOCIter;
use itertools::Itertools;

fn is_valid(pwd: &Vec<&str>) -> bool {
    pwd.iter().all_distinct()
}

fn is_valid2(pwd: &Vec<&str>) -> bool {
    pwd.iter().map(|word| word.chars().sorted().collect::<Vec<_>>()).all_distinct()
}

fn main() {
    let input = include_str!("../../inputs/2017/04");
    let passwords: Vec<Vec<&str>> = input
                                    .lines()
                                    .map(|line| line.split_whitespace().collect())
                                    .collect();

    let start = Instant::now();
    let p1 = passwords.iter().count_by(is_valid);
    let p2 = passwords.iter().count_by(is_valid2);
    let end = start.elapsed().as_micros();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Time: {} μs", end);
}