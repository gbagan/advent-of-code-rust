use anyhow::*;
use crate::util::grid::Grid;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let grid = Grid::parse(input)?;
    let p1 = part1(&grid);
    let p2 = part2(&grid);
    Ok((p1, p2))
}

const DIRECTIONS: [(usize, usize); 8] = [(0, 1), (1, 0), (1, 1), (1, usize::MAX)
                , (0, usize::MAX), (usize::MAX, 0), (usize::MAX, usize::MAX), (usize::MAX, 1)];

fn part1(grid: &Grid<u8>) -> u32 {
    let height = grid.height;
    let width = grid.width;
    let mut count = 0;

    for x in 0..height {
        for y in 0..width {
            let cur = grid[(x, y)];
            if cur == b'X' {
                for (dx, dy) in DIRECTIONS {
                    if x+3*dx < height && y+3*dy < width 
                        && grid[(x+dx, y+dy)] == b'M' 
                        && grid[(x+2*dx, y+2*dy)] == b'A' 
                        && grid[(x+3*dx, y+3*dy)] == b'S' {
                            count += 1;
                    }
                }
            }
        }
    }

    count
}

fn part2(grid: &Grid<u8>) -> u32 {
    let vec = &grid.vec;
    let height = grid.height;
    let width = grid.width;
    let mut count = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let index = i * width + j;
            if vec[index] != b'A' {
                continue
            }
            let a = vec[index - width - 1];
            let b = vec[index - width + 1];
            let c = vec[index + width - 1];
            let d = vec[index + width + 1];
            if (a == b'M' && d == b'S' || a == b'S' && d == b'M') && (b == b'M' && c == b'S' || b == b'S' && c == b'M') {
                count += 1;
            }
        }
    }
    count
}