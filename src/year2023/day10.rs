// Shoelace formula and Pick theorem for Part 2

use crate::util::{coord::Coord, grid::Grid};

type Point = Coord::<i32>;


pub fn solve(input: &str) -> Option<(i32,i32)> {
    let grid = Grid::parse(input);
    let mut start = None;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid[(x, y)] == b'S' {
                start = Some(Point::new(x as i32, y as i32))
            }
        }
    }
    let start = start?;
    let mut current = start.adjacent().iter().copied().find(|&p| grid[p] != b'.')?;
    let mut length = 1;
    let mut dir = current - start;
    let mut prev = start;
    let mut area = current.y * prev.x - current.x * prev.y;
    while current != start {
        dir = match grid[current] {
            b'L' => if dir == Point::SOUTH {Point::EAST} else {Point::NORTH} 
            b'J' => if dir == Point::SOUTH {Point::WEST} else {Point::NORTH}
            b'7' => if dir == Point::NORTH {Point::WEST} else {Point::SOUTH}
            b'F' => if dir == Point::NORTH {Point::EAST} else {Point::SOUTH}
            _ => dir
        };
        length += 1;
        prev = current;
        current += dir;
        area += current.y * prev.x - current.x * prev.y;
    }
    let p1 = length / 2;
    let p2 = (area.abs() - length) / 2 + 1;
    Some((p1, p2))
}