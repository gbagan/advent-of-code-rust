use anyhow::*;
use crate::util::{coord::Coord, grid::Grid};
use std::collections::HashSet;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let grid = Grid::parse(input)?;

    let mut p1 = 0;
    let mut p2 = 0;
    let mut stack = Vec::with_capacity(16);
    let mut level9 = HashSet::with_capacity(16);

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid[(x, y)] == b'0' {
                let (a, b) = hike_score(&grid, Coord::new(x as i32, y as i32), &mut stack, &mut level9);
                p1 += a;
                p2 += b;                
            }
        }
    }

    Ok((p1, p2))
}

fn hike_score(grid: &Grid<u8>, start: Coord<i32>, stack: &mut Vec<(Coord<i32>, u8)>, level9: &mut HashSet<Coord<i32>>) -> (u32, u32) {
    let width = grid.width as i32;
    let height = grid.height as i32;
    let mut rating = 0;
    stack.push((start , b'0'));
    while let Some((current, level)) = stack.pop() {
        if level == b'9' {
            rating += 1;
            level9.insert(current);
            continue;
        }
        if current.x > 0 {
            let next = current.left();
            if grid[next] == level + 1 {
                stack.push((next, level+1));
            }
        }
        if current.x < width - 1 {
            let next = current.right();
            if grid[next] == level + 1 {
                stack.push((next, level+1));
            }
        }
        if current.y > 0 {
            let next = current.above();
            if grid[next] == level + 1 {
                stack.push((next, level+1));
            }
        }
        if current.y < height - 1 {
            let next = current.below();
            if grid[next] == level + 1 {
                stack.push((next, level+1));
            }
        }
    }
    let score = level9.len();
    level9.clear();
    stack.clear();
    (score as u32, rating)
}