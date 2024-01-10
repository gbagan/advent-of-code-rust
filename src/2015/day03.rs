use aoc::aoc;
use std::collections::HashSet;
#[derive(PartialEq)]
pub struct Box {
  pub l: isize,
  pub h: isize,
  pub w: isize,
}

fn part1(input: &str) -> usize {
    let origin: (isize, isize) = (0, 0);

    let positions = input.chars().scan(origin, |acc, dir| {
        match dir {
            '<' => acc.0 -= 1,
            '>' => acc.0 += 1,
            '^' => acc.1 -= 1,
            'v' => acc.1 += 1,
            _ => panic!("invalid direction: {dir}"),
        }
        Some (acc.clone())
    });
    let mut visited: HashSet<(isize, isize)> = HashSet::from_iter(positions);
    visited.insert(origin);
    visited.len()
}

fn part2(input: &str) -> usize {
    let origin: (isize, isize) = (0, 0);

    let positions = input.chars().enumerate().scan((origin, origin), |(acc1, acc2), (i, dir)| {
        if i % 2 == 0 {
            match dir {
                '<' => acc1.0 -= 1,
                '>' => acc1.0 += 1,
                '^' => acc1.1 -= 1,
                'v' => acc1.1 += 1,
                _ => panic!("invalid direction: {dir}"),
            }
            Some (acc1.clone())
        } else {
            match dir {
                '<' => acc2.0 -= 1,
                '>' => acc2.0 += 1,
                '^' => acc2.1 -= 1,
                'v' => acc2.1 += 1,
                _ => panic!("invalid direction: {dir}"),
            }
            Some (acc2.clone())
        }    
    });
    let mut visited: HashSet<(isize, isize)> = HashSet::from_iter(positions);
    visited.insert(origin);
    visited.len()
}

fn main() {
    let input = include_str!("../../inputs/2015/03");
    aoc(|| (part1(input), part2(input)))
}