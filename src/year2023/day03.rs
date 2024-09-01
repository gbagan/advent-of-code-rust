use crate::util::grid::Grid;
use crate::util::coord::Coord;
use itertools::Itertools;
use std::str;

fn is_symbol(c: u8) -> bool {
    c != b'.' && !c.is_ascii_digit()
}

fn part_member(grid: &Grid<u8>, y: i64, x1: i64, x2: i64, i: usize) -> Option<u32> {
    let it_x = (x1-1).max(0)..=(x2+1).min(grid.height as i64 - 1);
    let it_y = (y-1).max(0)..=(y+1).min(grid.height as i64 - 1);
    if it_x
        .cartesian_product(it_y)
        .any(|(x, y)| is_symbol(grid[(x, y)])) {
            let i2 = i - (x2 - x1 + 1) as usize;
            str::from_utf8(&grid.vec[i2..i]).ok().and_then(|str| str.parse().ok())
    } else {
        None
    }
}

pub fn solve(input: &str) -> Option<(u32, u32)> {
    let grid = Grid::parse(input);
    let mut number_grid = grid.map::<Option<u32>>(|_| None);
    let mut i = 0;
    let mut first_digit = None;
    let mut p1 = 0;
    let mut p2 = 0;

    // compute number_grid which at each position associates the number written at this position or None otherwise
    for y in 0..(grid.width as i64) {
        for x in 0..(grid.height as i64) {
            let c = grid[i];
            if c.is_ascii_digit() {
                if first_digit.is_none() {
                    first_digit = Some(x)
                }
            } else if let Some(x1) = first_digit {
                if let Some(v) = part_member(&grid, y, x1, x-1, i) {
                    for ix in x1..x {
                        p1 += v;
                        number_grid[(ix, y)] = Some(v)
                    }
                }
                first_digit = None;
            }
            i += 1;
        }
        if let Some(x1) = first_digit {
            if let Some(v) = part_member(&grid, y, x1, grid.height as i64 -1, i) {
                for ix in x1..grid.height as i64 {
                    p1 += v;
                    number_grid[(ix, y)] = Some(v)
                }
            }
            first_digit = None;
        }
    }

    // part 2
    for y in 0..(grid.width as i32) {
        for x in 0..(grid.height as i32) {
            if grid[(x, y)] == b'*' {
                let p = Coord::new(x, y);
                let adj_numbers: Vec<_> =
                    p.surrounding()
                     .iter()
                     .filter_map(|&p| if grid.contains(p) {number_grid[p]} else {None})
                     .unique()
                     .collect();
                if adj_numbers.len() == 2 {
                    p2 += adj_numbers[0] * adj_numbers[1];
                }
            }
        }
    }
    Some((p1, p2))
}
