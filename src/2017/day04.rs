use aoc::aoc;
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

    aoc(|| (
        passwords.iter().count_by(is_valid),
        passwords.iter().count_by(is_valid2),
    ))
}