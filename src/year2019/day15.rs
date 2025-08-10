use ahash::{HashSet, HashSetExt};
use crate::util::coord::Coord;
use crate::year2019::intcode::*;

type Point = Coord<i32>;
use Status::*;

pub fn solve(input: &str) -> (u32, u32) {
    let mut machine = IntCode::new(input);
    let mut grid = HashSet::with_capacity(1000);
    let mut position = Point::ORIGIN;
    let mut direction = Point::NORTH;
    let mut blocked = false;
    let mut oxygen_system = None;

    loop {
        direction = if blocked { direction.turn_left() } else { direction.turn_right() };
        let instruction = match direction {
            Point::NORTH => 1,
            Point::SOUTH => 2,
            Point::WEST => 3,
            _ => 4
        };
        machine.input(instruction);
        match machine.run() {
            Output(0) => { blocked = true },
            Output(v) => {
                position += direction;
                blocked = false;
                grid.insert(position);
                if v == 2 {
                    oxygen_system = Some(position);
                }
                if position == Point::ORIGIN {
                    break;
                }
            }
            _ => panic!("Unexpected machine state")
        }
    }

    let oxygen_system = oxygen_system.unwrap();

    let mut queue1 = Vec::with_capacity(1000);
    let mut queue2 = Vec::with_capacity(1000);
    queue1.push(oxygen_system);
    grid.remove(&oxygen_system);
    let mut distance = 0;

    let directions = [Point::NORTH, Point::EAST, Point::SOUTH, Point::WEST];

    grid.remove(&oxygen_system);

    let mut p1 = 0;

    while !queue1.is_empty() {
        for &current in &queue1 {
            if current == Point::ORIGIN {
                p1 = distance;
            }
            for direction in directions {
                let next = current + direction;
                if grid.remove(&next) {
                    queue2.push(next);
                }
            }
        }
        distance += 1;
        std::mem::swap(&mut queue1, &mut queue2);
        queue2.clear();
    }

    (p1, distance-1)
}
