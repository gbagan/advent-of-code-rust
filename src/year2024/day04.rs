use anyhow::*;
use crate::util::grid::Grid;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let grid = Grid::parse(input)?;
    let p1 = part1(&grid);
    let p2 = part2(&grid);
    Ok((p1, p2))
}

fn part1(grid: &Grid<u8>) -> u32 {
    let vec = &grid.vec;
    let height = grid.height;
    let width = grid.width;
    let mut count = 0;

    for i in 0..height {
        for j in 0..width - 3 {
            let index = i * width + j;
            if vec[index] == b'X' && vec[index+1] == b'M' 
                && vec[index+2] == b'A' && vec[index+3] == b'S' {
                count += 1;
            }
            if vec[index] == b'S' && vec[index+1] == b'A' 
                && vec[index+2] == b'M' && vec[index+3] == b'X' {
                count += 1;
            }
        }
    }

    for i in 0..height - 3 {
        for j in 0..width {
            let index = i * width + j;
            if vec[index] == b'X' && vec[index+width] == b'M' 
                && vec[index+2*width] == b'A' && vec[index+3*width] == b'S' {
                count += 1;
            }
            if vec[index] == b'S' && vec[index+width] == b'A' 
                && vec[index+2*width] == b'M' && vec[index+3*width] == b'X' {
                count += 1;
            }
        }
    }

    for i in 0..height - 3 {
        for j in 0..width - 3 {
            let index = i * width + j;
            let d = width + 1;
            if vec[index] == b'X' && vec[index+d] == b'M' 
                && vec[index+2*d] == b'A' && vec[index+3*d] == b'S' {
                count += 1;
            }
            if vec[index] == b'S' && vec[index+d] == b'A' 
                && vec[index+2*d] == b'M' && vec[index+3*d] == b'X' {
                count += 1;
            }
        }
    }

    for i in 0..height - 3 {
        for j in 3..width {
            let index = i * width + j;
            let d = width - 1;
            if vec[index] == b'X' && vec[index+d] == b'M' 
                && vec[index+2*d] == b'A' && vec[index+3*d] == b'S' {
                count += 1;
            }
            if vec[index] == b'S' && vec[index+d] == b'A' 
                && vec[index+2*d] == b'M' && vec[index+3*d] == b'X' {
                count += 1;
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