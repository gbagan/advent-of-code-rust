use crate::util::{coord::*, parser::*};

type Point = Coord<i32>; 

pub fn solve(input: &str) -> (i32, i32) {    
    let iter = input
        .bytes()
        .filter(|&c| c.is_ascii_uppercase())
        .zip(input.iter_unsigned::<i32>());

    let mut position = Point::ORIGIN;
    let mut direction = Point::EAST;
    let mut position2 = Point::ORIGIN;
    let mut waypoint = Point::new(10, -1);

    for (action, value) in iter {
        match action {
            b'N' => {
                position.y -= value;
                waypoint.y -= value;
            }
            b'S' => {
                position.y += value;
                waypoint.y += value;
            }
            b'E' => {
                position.x += value;
                waypoint.x += value;
            }
            b'W' => {
                position.x -= value;
                waypoint.x -= value;
            }
            b'L' => {
                direction = move_left(direction, value);
                waypoint = move_left(waypoint, value);
            }
            b'R' => {
                direction = move_right(direction, value);
                waypoint = move_right(waypoint, value);
            }        
            _ => {
                position += direction * value;
                position2 += waypoint * value;
            }
        }

    }

    let p1 = position.manhattan(Point::ORIGIN);
    let p2 = position2.manhattan(Point::ORIGIN);
    
    (p1, p2)
}

fn move_left(direction: Point, value: i32) -> Point {
    match value {
        90 => direction.turn_left(),
        180 => -direction,
        _ => direction.turn_right(),
    }
}

fn move_right(direction: Point, value: i32) -> Point {
    match value {
        90 => direction.turn_right(),
        180 => -direction,
        _ => direction.turn_left(),
    }
}