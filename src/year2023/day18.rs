// shoelace formula and Pick theorem

use itertools::Itertools;
use crate::util::coord::Coord;

type Point = Coord<i32>;

pub fn solve(input: &str) -> (i64, i64) {
    let mut instrs1 = vec!();
    let mut instrs2 = vec!();
    for line in input.lines() {
        let (t1, t2) =  parse_line(line);
        instrs1.push(t1);
        instrs2.push(t2);
    }
    let p1 = lava(&instrs1);
    let p2 = lava(&instrs2);
    (p1, p2)
}


fn parse_line(line: &str) -> ((Point, i32), (Point, i32)) {
    let (dir1, len1, hex) = line.split_ascii_whitespace().next_tuple().unwrap();
    let dir1 = match dir1 {
        "L" => Point::WEST,
        "R" => Point::EAST,
        "U" => Point::NORTH,
        "D" => Point::SOUTH,
        _ => panic!()
    };
    let len1 = len1.parse().unwrap();
    let mut hex = hex.trim_matches(['(', ')', '#']).to_string();
    let dir2 = hex.pop().unwrap();
    let dir2 = match dir2 {
        '0' => Point::EAST,
        '1' => Point::SOUTH,
        '2' => Point::WEST,
        '3'   => Point::NORTH,
        _ => panic!()
    };
    let len2 = i32::from_str_radix(&hex, 16).unwrap();
    ((dir1, len1), (dir2, len2))
}


fn lava(instrs: &Vec<(Point, i32)>) -> i64 {
    let mut boundary = 0;
    let mut area = 0;
    let mut prev;
    let mut current = Point::ORIGIN;
    for &(direction, length) in instrs {
        boundary += length;
        prev = current;
        current += direction * length;
        area += current.x as i64 * prev.y as i64 - current.y as i64 * prev.x as i64;
    }

    (area.abs() + boundary as i64) / 2 + 1
}