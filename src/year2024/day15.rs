use std::collections::VecDeque;

use anyhow::*;
use crate::util::{grid::*, parser::*};

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let (grid, directions) = input.trim().try_split_once("\n\n")?;
    let grid = Grid::parse(grid)?;
    let start = grid.vec.iter().position(|&c| c == b'@').context("No start symbol found")?;

    let p1 = part1(&grid, start, directions.as_bytes());
    let p2 = part2(&grid, start, directions.as_bytes());
    Ok((p1, p2))
}

fn part1(grid: &Grid<u8>, start: usize, directions: &[u8]) -> usize {
    let width = grid.width;
    let mut grid = grid.vec.clone();
    let mut position = start;
    let up = 0usize.wrapping_sub(width);
    for &direction in directions {
        let direction = match direction {
            b'^' => up,
            b'v' => width,
            b'<' => usize::MAX,
            b'>' => 1,
            _ => continue,
        };
        position = push(&mut grid, position, direction);
    }
    score(&grid, width, b'O')
}

fn score(grid: &[u8], width: usize, c: u8) -> usize {
    let mut sum = 0;
    let mut i = 0;
    let height = grid.len() / width;

    for y in 0..height {
        for x in 0..width {
            if grid[i] == c {
                sum += 100 * y + x;
            }
            i += 1;
        }
    }
    sum
}


fn push(grid: &mut [u8], position: usize, direction: usize) -> usize {
    let next = position.wrapping_add(direction);
    let mut x = next;
    while grid[x] == b'O' {
        x = x.wrapping_add(direction);
    }
    if grid[x] == b'#' {
        position
    } else if x == next {
        next
    } else {
        grid[x] = b'O';
        grid[next] = b'.';
        next
    }
}

fn part2(grid: &Grid<u8>, start: usize, directions: &[u8]) -> usize {
    let width = grid.width;
    let mut grid = large_grid(&grid.vec);
    let mut position = 2 * start;
    let width = 2 * width;
    let up = 0usize.wrapping_sub(width);

    let mut seen = vec![0u16; grid.len()];
    let mut todo = VecDeque::with_capacity(16);
    let mut tomove = Vec::with_capacity(16);

    for (i, &direction) in directions.iter().enumerate() {
        position = match direction {
            b'^' => push_vertical(&mut grid, &mut seen, &mut todo, &mut tomove, i as u16 + 1, position, up),
            b'v' => push_vertical(&mut grid, &mut seen, &mut todo, &mut tomove, i as u16 + 1, position, width),
            b'<' => push_horizontal(&mut grid, position, usize::MAX),
            b'>' => push_horizontal(&mut grid, position, 1),
            _ => continue,
        };
    }

    score(&grid, width, b'[')
}


fn large_grid(grid: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(2*grid.len());
    for c in grid {
        let (c1, c2) = match c {
            b'O' => (b'[', b']'),
            b'#' => (b'#', b'#'),
            _ => (b'.', b'.'),
        };
        res.push(c1);
        res.push(c2);
    }
    res
}

#[inline]
fn push_horizontal(grid: &mut [u8], position: usize, direction: usize) -> usize {
    let next = position.wrapping_add(direction);
    let mut x = next;
    while grid[x] == b'[' || grid[x] == b']' {
        x = x.wrapping_add(direction);
    }
    if grid[x] == b'#' {
        position
    } else if x == next {
        next
    } else {
        let mut y = x;
        while y != position {
            let z = y.wrapping_sub(direction);
            grid[y] = grid[z];
            y = z;
        }
        next
    }
}

#[inline]
fn push_vertical(
    grid: &mut [u8],
    seen: &mut [u16],
    todo: &mut VecDeque<usize>,
    tomove: &mut Vec<usize>,
    iter: u16,
    position: usize,
    direction: usize) -> usize {
    let next = position.wrapping_add(direction);
    match grid[next] {
        b'.' => return next,
        b'#' => return position,
        b'[' => { todo.clear(); todo.push_back(next); todo.push_back(next+1) },
        b']' => { todo.clear(); todo.push_back(next); todo.push_back(next-1) },
        _ => unreachable!()
    }
    tomove.clear();
    while let Some(pos) = todo.pop_front() {
        if seen[pos] == iter {
            continue;
        }
        seen[pos] = iter;
        tomove.push(pos);
        let next = pos.wrapping_add(direction);
        match grid[next] {
            b'#' => return position,
            b'.' => {},
            b'[' => { todo.push_back(next); todo.push_back(next+1) },
            b']' => { todo.push_back(next); todo.push_back(next-1) },
            _ => unreachable!(),
        }
    }
    for &pos in tomove.iter().rev() {
        grid[pos.wrapping_add(direction)] = grid[pos];
        grid[pos] = b'.';
    }
    next
}