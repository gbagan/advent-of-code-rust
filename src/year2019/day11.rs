use crate::{util::coord::*, year2019::intcode::*};
use ahash::{HashMap, HashMapExt};

type Point = Coord<i32>;

use Status::*;

pub fn solve(input: &str) -> (usize, String) {
    let mut machine = IntCode::new(input);
    let mut machine2 = machine.clone();

    let mut painted = HashMap::new();
    paint(&mut machine, &mut painted);
    let p1 = painted.len();
    let p2 = part2(&mut machine2);

    (p1, p2)
}

fn paint(machine: &mut IntCode, painted: &mut HashMap<Point, i64>) {
    let mut position = Point::new(0, 0);
    let mut direction = Point::NORTH;
   
    loop {
        machine.input(*painted.get(&position).unwrap_or(&0));
        let Output(color) = machine.run() else { break };
        painted.insert(position, color);
        let Output(turn) = machine.run() else { break };
        direction = if turn == 0 { direction.turn_left() } else { direction.turn_right() };
        position += direction;
    }
}

fn part2(machine: &mut IntCode) -> String {
    let mut painted = HashMap::new();
    painted.insert(Point::ORIGIN, 1);
    paint(machine, &mut painted);
    let whites: Vec<_> = painted.iter().filter(|&(_, &color)| color == 1).map(|(&k, _)| k).collect();
    let mut xmin = i32::MAX;
    let mut xmax = i32::MIN;
    let mut ymin = i32::MAX;
    let mut ymax = i32::MIN;
    
    for point in &whites {
        xmin = xmin.min(point.x);
        xmax = xmax.max(point.x);
        ymin = ymin.min(point.y);
        ymax = ymax.max(point.y);
    }
    let width = xmax - xmin + 2;
    let height = ymax - ymin + 1;
    let mut message = vec![b'.'; (width * height) as usize];
    for c in message.iter_mut().step_by(width as usize) {
        *c = b'\n';
    }
    for point in &whites {
        message[((point.y - ymin) * width + point.x - xmin + 1) as usize] = b'#';
    }

    String::from_utf8(message).unwrap()
}