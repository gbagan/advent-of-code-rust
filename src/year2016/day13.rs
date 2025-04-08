// breadth first search

use std::collections::VecDeque;
use crate::util::{coord::Coord, grid::Grid, parser::*};

type Point = Coord::<u32>;

fn is_wall(p: Point, n: u32) -> bool {
    let Point {x, y} = p;
    (x*x + 3*x + 2*x*y + y + y*y + n).count_ones() % 2 == 1
}

pub fn solve(input: &str) -> (u32, u32) {
    let mut p2 = 0;
    let n = input.try_unsigned().unwrap();
    let end = Point::new(31, 39);
    let mut seen = Grid::new(52, 52, false);
    let mut queue = VecDeque::new();
    queue.push_back((Point::new(1, 1), 0));
    while let Some((node, dist)) = queue.pop_front() {
        if seen[node] || is_wall(node, n) {
           continue; 
        }
        seen[node] = true;
        if node == end {
            return (dist, p2);
        } else if dist <= 50 {
            p2 += 1;
        }
        queue.push_back((node.right(), dist+1));
        queue.push_back((node.below(), dist+1));
        if node.x > 0 {
            queue.push_back((node.left(), dist+1));
        }
        if node.y > 0 {
            queue.push_back((node.above(), dist+1));
        }
    }
    unreachable!();
}