use rayon::prelude::*;
use crate::util::{coord::Coord, grid::Grid, iter::AOCIter};

pub struct Input {
    grid: Grid<u8>,
    north: Grid<i32>,
    south: Grid<i32>,
    west: Grid<i32>,
    east: Grid<i32>,
}

const NORTH: u8 = 0;
const EAST: u8 = 1;
const SOUTH: u8 = 2;
const WEST: u8 = 3;

#[inline]
fn next_directions (c: u8, dir: u8) -> Vec<u8> {
    match c {
        b'/' => vec!(dir ^ 1),
        b'\\' => vec!(3 - dir),
        b'-' if dir & 1 == 0 => vec!(WEST, EAST),
        b'|' if dir & 1 != 0 => vec!(NORTH, SOUTH),
        _ => vec!(dir)
    }
}

pub fn parse(input: &str) -> Option<Input>{
    let grid = Grid::parse(input);
    let mut north = grid.map(|_| 0);
    let mut south = grid.map(|_| 0);
    let mut west = grid.map(|_| 0);
    let mut east = grid.map(|_| 0);


    for x in 0..grid.width {
        let mut last: i32 = -1;

        for y in 0..grid.height {
            north[(x, y)] = last;

            if matches!(grid[(x, y)], b'/' | b'\\' | b'-') {
                last = y as i32;
            }
        }

        last = grid.height as i32;
        for y in (0..grid.height).rev() {
            south[(x, y)] = last;

            if matches!(grid[(x, y)], b'/' | b'\\' | b'-') {
                last = y as i32;
            }
        }
    }

    for y in 0..grid.height {
        let mut last: i32 = -1;

        for x in 0..grid.width {
            west[(x, y)] = last;

            if matches!(grid[(x, y)], b'/' | b'\\' | b'|') {
                last = x as i32;
            }
        }

        last = grid.width as i32;
        for x in (0..grid.height).rev() {
            east[(x, y)] = last;

            if matches!(grid[(x, y)], b'/' | b'\\' | b'|') {
                last = x as i32;
            }
        }
    }

    Some(Input { grid, north, south, west, east})
}

fn count_energized(input: &Input, start_pos: Coord, start_dir: u8) -> usize {
    let Input { grid, north, south, west, east } = input;
    let mut energized = input.grid.map(|_| false);
    let mut visited: Grid<u8> = input.grid.map(|_|  0);
    let mut stack = vec!((start_pos, start_dir));

    while let Some((pos, dir)) = stack.pop() {
        let mask = 1 << dir;
        if visited[pos] & mask != 0 {
            continue;
        }
        visited[pos] |= mask;
        for dir in next_directions(grid[pos], dir) {
            match dir {
                NORTH => {
                    let next_y = north[pos];
                    for y in next_y+1..=pos.y {
                        energized[(pos.x, y)] = true;
                    }
                    if next_y >= 0 {
                        stack.push((Coord::new(pos.x, next_y), dir));
                    }
                }
                SOUTH => {
                    let next_y = south[pos];
                    for y in pos.y..next_y {
                        energized[(pos.x, y)] = true;
                    }
                    if next_y < grid.height as i32 {
                        stack.push((Coord::new(pos.x, next_y), dir));
                    }
                }
                WEST => {
                    let next_x = west[pos];
                    for x in next_x+1..=pos.x {
                        energized[(x, pos.y)] = true;
                    }
                    if next_x >= 0 {
                        stack.push((Coord::new(next_x, pos.y), dir));
                    }
                }
                EAST => {
                    let next_x = east[pos];
                    for x in pos.x..next_x {
                        energized[(x, pos.y)] = true;
                    }
                    if next_x < grid.width as i32 {
                        stack.push((Coord::new(next_x, pos.y), dir));
                    }
                }
                _ => unreachable!()
            }
        }
    }
    energized.vec.iter().count_by(|&x| x)
}


pub fn part1(input: &Input) -> Option<usize> {
    Some(count_energized(input, Coord::ORIGIN, EAST))
}

pub fn part2(input: &Input) -> Option<usize> {
    let Input {grid, ..} = input;
    let mut starts = vec!();
    for x in 0..grid.width as i32 {
        starts.push((Coord::new(x, 0), SOUTH));
        starts.push((Coord::new(0, grid.height as i32-1), NORTH));
    }
    for y in 0..grid.height as i32 {
        starts.push((Coord::new(0, y), EAST));
        starts.push((Coord::new(grid.width as i32 -1, y), WEST));
    }

    starts.par_iter().map(|(pos, dir)| count_energized(&input, *pos, *dir)).max()
}