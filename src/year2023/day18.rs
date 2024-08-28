// shoelace formula and Pick theorem

use itertools::Itertools;
use crate::util::coord::Coord;

type Input = (Vec<(Coord, i32)>, Vec<(Coord, i32)>);

fn parse_line(line: &str) -> Option<((Coord, i32), (Coord, i32))> {
    let (dir1, len1, hex) = line.split_ascii_whitespace().next_tuple()?;
    let dir1 = match dir1 {
        "L" => Coord::WEST,
        "R" => Coord::EAST,
        "U" => Coord::NORTH,
        "D" => Coord::SOUTH,
        _ => panic!("unexcepted character: {dir1}")
    };
    let len1 = len1.parse().ok()?;
    let mut hex = hex.trim_matches(['(', ')', '#']).to_string();
    let dir2 = hex.pop()?;
    let dir2 = match dir2 {
        '0' => Coord::EAST,
        '1' => Coord::SOUTH,
        '2' => Coord::WEST,
        '3'   => Coord::NORTH,
        _ => panic!("unexcepted character: {dir2}")
    };
    let len2 = i32::from_str_radix(&hex, 16).ok()?;
    Some(((dir1, len1), (dir2, len2)))
}

pub fn parse(input: &str) -> Option<Input> {
    let mut p1 = vec!();
    let mut p2 = vec!();
    for (t1, t2) in input.lines().filter_map(parse_line) {
        p1.push(t1);
        p2.push(t2);
    }
    Some((p1, p2))
}


fn lava(instrs: &Vec<(Coord, i32)>) -> i64 {
    let mut boundary = 0;
    let mut area = 0;
    let mut prev;
    let mut current = Coord::ORIGIN;
    for (direction, length) in instrs {
        boundary += length;
        prev = current;
        current += *direction * *length;
        area += current.x as i64 * prev.y as i64 - current.y as i64 * prev.x as i64;
    }

    (area.abs() + boundary as i64) / 2 + 1
}

pub fn part1(input: &Input) -> Option<i64> {
    Some(lava(&input.0))
}

pub fn part2(input: &Input) -> Option<i64> {
    Some(lava(&input.1))
}