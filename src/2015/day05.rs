use std::time::Instant;
use itertools::Itertools;
use aoc::iter::AOCIter;

fn is_vowel (c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn is_nice_string (s: &str) -> bool {
    s.chars().count_by(is_vowel) >= 3
    && s.chars().tuple_windows().any(|(x, y)| x == y)
    && ! (s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy"))
}

fn is_nice_string2 (s: &str) -> bool {
    s.chars().tuple_windows().any(|(x, _, z)| x == z)
    && s.chars().tuple_windows().any(|(x, y) | s.matches(&[x, y].iter().collect::<String>()).count() >= 2)
}

fn main() {
    let input = include_str!("../../inputs/2015/05");
    let start = Instant::now();

    let p1 = input.lines().count_by(is_nice_string);
    let p2 = input.lines().count_by(is_nice_string2);

    let end = start.elapsed().as_micros();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Time: {} μs", end);
}