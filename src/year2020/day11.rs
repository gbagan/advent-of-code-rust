use anyhow::*;
use crate::util::grid::Grid;

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let grid = Grid::parse_with_padding(input, b'B')?;
    let p1 = part1(&grid); 

    Ok((p1, 0))
}

fn step1(grid: &Grid<u8>) -> Grid<u8> {
    let w = grid.width;
    let directions = [1, w, w-1, w+1, usize::MAX, usize::MAX-w
                                , 0usize.wrapping_sub(w), 1usize.wrapping_sub(w)];
    grid.map_with_indices(|i, &v| match v {
        b'L' => if directions.iter().all(|d| grid[i+d] != b'#') {
            b'#'
        } else {
            b'L'
        }
        b'#' => if directions.iter().filter(|&&d| grid[i+d] == b'#').count() >= 4 {
            b'L'
        } else {
            b'#'
        }
        _ => v,
    })
}

fn part1(grid: &Grid<u8>) -> usize {
    let mut grid = grid.clone();
    loop {
        let next = step1(&grid);
        if next.vec[grid.width..] == grid.vec[grid.width..] {
            return grid.vec.iter().filter(|&&c| c == b'#').count()
        }
        grid = next;
    }
}