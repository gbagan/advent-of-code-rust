use anyhow::*;
use std::collections::HashSet;
use crate::util::{coord::Coord, parser::*};

type Point = Coord<i32>;

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let input = input.trim();
    let dirs_iter = input.bytes().filter(|c| c.is_ascii_uppercase());
    let blocks_iter = input.iter_unsigned();
    let instrs: Vec<_> = dirs_iter.zip(blocks_iter).collect();

    let p1 = part1(&instrs);
    let p2 = part2(&instrs);
    Ok((p1, p2))
}

fn part1(instrs: &[(u8, i32)]) -> i32 {
    let mut position = Point::ORIGIN;
    let mut direction = Point::NORTH;

    for &(dir,  blocks) in instrs {
        if dir == b'L' {
            direction = direction.turn_left();
        } else {
            direction = direction.turn_right();
        }
        position += direction * blocks;
    }
    position.manhattan(Point::ORIGIN)
}

fn part2(instrs: &[(u8, i32)]) -> i32 {
    let mut seen = HashSet::new();
    let mut position = Point::ORIGIN;
    let mut direction = Point::NORTH;
    seen.insert(position);

    for &(dir,  blocks) in instrs {
        if dir == b'L' {
            direction = direction.turn_left();
        } else {
            direction = direction.turn_right();
        }
        for _ in 0..blocks {
            position += direction;
            if !seen.insert(position) {
                return position.manhattan(Point::ORIGIN);
            }
        }
    }
    unreachable!()
}