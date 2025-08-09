use std::collections::BTreeMap;
use crate::util::{parser::*, coord::*};

type Point = Coord::<i32>;

struct Segment {
    start: Point,
    end: Point,
    distance: i32, 
}

impl Segment {
    fn contains(&self, other: Point) -> bool {
        if self.start.y == self.end.y {
            self.start.x <= other.x && other.x <= self.end.x
            || self.end.x <= other.x && other.x <= self.start.x
        } else {
            self.start.y <= other.y && other.y <= self.end.y
            || self.end.y <= other.y && other.y <= self.start.y
        }
    }
}


pub fn solve(input: &str) -> (i32, i32) {
    let (line1, line2) = input.split_once('\n').unwrap();
    let (horizontal, vertical) = parse_segments(line1);

    let directions = line2.bytes().filter(|c| c.is_ascii_uppercase());
    let lengths = line2.iter_unsigned::<i32>();


    let mut p1 = i32::MAX;
    let mut p2 = i32::MAX;

    let mut current = Point::ORIGIN;
    let mut distance = 0;
    
    for (dir, length) in directions.zip(lengths) {
        let mut update = |segment: &Segment, candidate: Point| {
            if segment.contains(candidate) && candidate != Point::ORIGIN {
                p1 = p1.min(candidate.manhattan(Point::ORIGIN));
                p2 = p2.min(
                    distance
                    + segment.distance
                    + candidate.manhattan(segment.start)
                    + candidate.manhattan(current)
                );
            }
        };
        
        let delta = match dir {
            b'L' => Point::WEST,
            b'R' => Point::EAST,
            b'U' => Point::NORTH,
            _  => Point::SOUTH,
        };

        let start = current;
        let end = current + delta * length;

        match dir {
            b'L' => {
                for (&x, segment) in vertical.range(end.x..=start.x) {
                    update(segment, Point::new(x, start.y));
                }
            }
            b'R' => {
                for (&x, segment) in vertical.range(start.x..=end.x) {
                    update(segment, Point::new(x, start.y));
                }
            }
            b'U' => {
                for (&y, segment) in horizontal.range(end.y..=start.y) {
                    update(segment, Point::new(start.x, y));
                }
            }
            _ => {
                for (&y, segment) in horizontal.range(start.y..=end.y) {
                    update(segment, Point::new(start.x, y));
                }
            }
        };

        current = end;
        distance += length;
    }

    (p1, p2)
}

fn parse_segments(line: &str) -> (BTreeMap<i32, Segment>, BTreeMap<i32, Segment>) {
    let mut horizontal = BTreeMap::new();
    let mut vertical = BTreeMap::new();
    let directions = line.bytes().filter(|c| c.is_ascii_uppercase());
    let lengths = line.iter_unsigned::<i32>();
    let mut current = Point::ORIGIN;
    let mut distance = 0;
    for (dir, length) in directions.zip(lengths) {
        let delta = match dir {
            b'L' => Point::WEST,
            b'R' => Point::EAST,
            b'U' => Point::NORTH,
            _  => Point::SOUTH,
        };
        let next = current + delta * length;
        let segment = Segment { start: current, end: next, distance: distance };
        if next.y == current.y {
            horizontal.insert(current.y, segment);
        } else {
            vertical.insert(current.x, segment);
        }
        current = next;
        distance += length;
    }
    (horizontal, vertical)

}