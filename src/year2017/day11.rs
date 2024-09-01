use crate::util::coord::Coord;

type Point = Coord<i32>;

fn to_coord(s: &str) -> Point {
    match s {
        "nw" => Point::new(-1, -1),
        "ne" => Point::new(1, 0),
        "n" => Point::new(0, -1),
        "sw" => Point::new(-1, 0),
        "se" => Point::new(1, 1),
        "s" => Point::new(0, 1),
        _ => panic!("unexpected characters: {s}")
    }
}

fn distance(Point {x, y}: Point) -> i32 {
    x.abs().max(y.abs()).max((x-y).abs())
}

pub fn solve(input: &str) -> Option<(i32, i32)> {
    let directions = input.trim().split(',').map(to_coord);
    let coords = directions.scan(Point::ORIGIN, |acc, dir| {
        *acc += dir;
        Some (*acc)
    });
    let mut p1 = 0;
    let mut p2 = 0;
    for coord in coords {
        let d = distance(coord);
        p1 = d;
        p2 = p2.max(d);
    }

    Some((p1, p2))
}