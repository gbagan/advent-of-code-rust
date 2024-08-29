use crate::util::coord::Coord;

fn to_coord(s: &str) -> Coord {
    match s {
        "nw" => Coord::new(-1, -1),
        "ne" => Coord::new(1, 0),
        "n" => Coord::new(0, -1),
        "sw" => Coord::new(-1, 0),
        "se" => Coord::new(1, 1),
        "s" => Coord::new(0, 1),
        _ => panic!("unexpected characters: {s}")
    }
}

fn distance(Coord {x, y}: Coord) -> i32 {
    x.abs().max(y.abs()).max((x-y).abs())
}

pub fn solve(input: &str) -> Option<(i32, i32)> {
    let directions = input.trim().split(',').map(to_coord);
    let coords = directions.scan(Coord::ORIGIN, |acc, dir| {
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