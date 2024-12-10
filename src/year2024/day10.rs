use anyhow::*;
use crate::util::grid::Grid;
use std::collections::HashSet;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let grid = Grid::parse_with_padding(input, b'#')?;

    let mut p1 = 0;
    let mut p2 = 0;
    let mut stack = Vec::with_capacity(16);
    let mut level9 = HashSet::with_capacity(16);

    for (i, &c) in grid.vec.iter().enumerate() {
        if c == b'0' {
            let (a, b) = hike_score(&grid, i, &mut stack, &mut level9);
            p1 += a;
            p2 += b;
        }
    }
    
    Ok((p1, p2))
}

fn hike_score(grid: &Grid<u8>, start: usize, stack: &mut Vec<(usize, u8)>, level9: &mut HashSet<usize>) -> (u32, u32) {
    let width = grid.width;
    let mut rating = 0;
    stack.push((start , b'0'));
    while let Some((current, level)) = stack.pop() {
        if level == b'9' {
            rating += 1;
            level9.insert(current);
            continue;
        }
        let next = current - 1;
        if grid[next] == level + 1 {
            stack.push((next, level+1));
        }
        
        let next = current + 1;
        if grid[next] == level + 1 {
            stack.push((next, level+1));
        }

        let next = current - width;
        if grid[next] == level + 1 {
            stack.push((next, level+1));
        }
        
        let next = current + width;
        if grid[next] == level + 1 {
            stack.push((next, level+1));
        }
    }
    let score = level9.len();
    level9.clear();
    stack.clear();
    (score as u32, rating)
}