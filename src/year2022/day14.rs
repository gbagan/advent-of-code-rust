use anyhow::*;
use itertools::Itertools;
use crate::util::{grid::Grid, parser::*};

fn parse_line(line: &str) -> Vec<(usize, usize)> {
    line.iter_unsigned().tuples().collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Kind {Empty, Rock, Falling, Sand}

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let coords: Vec<_> = input
        .lines()
        .map(parse_line)
        .collect();

    let mut ymax = 0;
    for row in &coords {
        for coord in row {
            ymax = ymax.max(coord.1);
        }
    }

    let height = ymax + 2;
    let width = 2*height + 1;
    
    let mut grid = Grid::new(width, height, Kind::Empty);

    for row in &coords {
        for (&(x1, y1), &(x2, y2)) in row.iter().tuple_windows() {
            for x in x1.min(x2)..x1.max(x2)+1 {
                for y in y1.min(y2)..y1.max(y2)+1 {
                    grid[(x + height - 500, y)] = Kind::Rock;
                }
            }
        }
    }
    let mut p1 = 0;
    let height = grid.height;
    fall(&mut grid, &mut p1, height);
    let p2 = p1 + part2(&mut grid);

    Ok((p1, p2))
}

fn fall(grid: &mut Grid<Kind>, counter: &mut u32, index: usize) -> Kind {
    let index2 = index + grid.width;
    let test = is_stable(grid, counter, index2)
        && is_stable(grid, counter, index2 - 1)
        && is_stable(grid, counter, index2 + 1);
    if test {
        *counter += 1;
        grid[index] = Kind::Sand;
        Kind::Sand
    } else {
        grid[index] = Kind::Falling;
        Kind::Falling
    }
}

fn is_stable(grid: &mut Grid<Kind>, counter: &mut u32, index: usize) -> bool {
    if index >= grid.height * grid.width {
        false
    } else if grid[index] == Kind::Empty {
        fall(grid, counter, index) == Kind::Sand
    } else {
        matches!(grid[index], Kind::Sand | Kind::Rock)
    }
}

fn part2(grid: &mut Grid<Kind>) -> u32 {
    let mut counter = 1;
    let width = grid.width;
    let height = grid.height;
    grid[height] = Kind::Sand;

    for y in 1..height {
        for index in y*width+height-y..y*width+height+y+1 {
            let test = matches!(grid[index], Kind::Empty | Kind::Falling)
                    && (grid[index - width] == Kind::Sand
                    || grid[index - width - 1] == Kind::Sand 
                    || grid[index - width + 1] == Kind::Sand);
            if test {
                grid[index] = Kind::Sand;
                counter += 1;           
            }
        }
    }
    counter
}