use anyhow::*;
use crate::util::grid::*;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let mut grid = Grid::parse_with_padding(input, b'.')?;
    let width = grid.width;
    let grid = &mut grid.vec;
    let mut stack = Vec::with_capacity(500);
    let mut p1 = 0;
    let mut p2 = 0;

    for i in 0..grid.len() {
        let start = grid[i];
        if start.is_ascii_uppercase() {
            let mut area = 0;
            let mut perimeter = 0;
            let mut sides = 0;
            stack.push(i);
            while let Some(current) = stack.pop() {
                if grid[current] != start {
                    continue;
                }
                grid[current] |= 128;
                area += 1;
                for (next, side) in [(current-1, width), (current+1, width), (current-width, 1), (current+width, 1)] {
                    stack.push(next);
                    if grid[next] & 127 != start {
                        perimeter += 1;
                        let c1 = grid[current+side];
                        let c2 = grid[next+side];
                        if c1 & 127 != start || c2 & 127 == start {
                            sides += 1;
                        }
                    }
                }
            }
            p1 += area * perimeter;
            p2 += area * sides;
            stack.clear();
        }
    }
    Ok((p1, p2))
}