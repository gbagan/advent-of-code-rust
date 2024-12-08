use anyhow::*;
use crate::util::grid::Grid;

const XMAS: u32 = 0x584d4153;
const SAMX: u32 = 0x53414d58;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let grid = Grid::parse(input)?;
    let p1 = part1(&grid);
    let p2 = part2(&grid);
    Ok((p1, p2))
}

fn part1(grid: &Grid<u8>) -> u32 {
    let height = grid.height;
    let width = grid.width;
    let mut count = 0;

    assert!(height == width);

    for i in 0..width {
        count += count_line(&grid.vec, i, width, width);
        count += count_line(&grid.vec, i * width, width, 1);
    }

    for i in 0..width-3 {
        count += count_line(&grid.vec, i,width - i, width + 1);
        count += count_line(&grid.vec, width - i - 1, width - i, width - 1);
    }

    for i in 1..width-3 {
        count += count_line(&grid.vec, i * width, width - i, width + 1);
        count += count_line(&grid.vec, i * width + width - 1, width - i, width - 1);
    }

    count
}

fn count_line(grid: &[u8], start: usize, times: usize, step: usize) -> u32 {
    let end = grid.len().min(start + step * times);
    
    let mut word = 0;
    let mut count = 0;
    let mut i = start;
    
    while i < end {
        word = (word << 8) | grid[i] as u32;
        if word == XMAS || word == SAMX {
            count += 1;
        }
        i += step;
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
            if a.abs_diff(d) == 6 && b.abs_diff(c) == 6 {
                count += 1;
            }
        }
    }
    count
}