use crate::util::grid::Grid;
use crate::util::coord::Coord;
use std::str;

pub fn solve(input: &str) -> (u32, u32) {
    let grid = Grid::parse_with_padding(input, b'.');
    let mut number_grid: Grid<Option<u32>> = grid.map(|_| None);
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
                    p1 += v;
                    for ix in x1..x {
                        number_grid[(ix, y)] = Some(v)
                    }
                }
                first_digit = None;
            }
            i += 1;
        }
        if let Some(x1) = first_digit {
            if let Some(v) = part_member(&grid, y, x1, grid.height as i64 -1, i) {
                p1 += v;
                for ix in x1..grid.height as i64 {
                    number_grid[(ix, y)] = Some(v)
                }
            }
            first_digit = None;
        }
    }

    let mut adj_numbers = Vec::new();
    // part 2
    for y in 0..(grid.width as i32) {
        for x in 0..(grid.height as i32) {
            if grid[(x, y)] == b'*' {
                let p = Coord::new(x, y);
                adj_numbers.clear();
                adj_numbers.extend(
                    p.adjacent8()
                     .iter()
                     .filter_map(|&p| number_grid[p])
                );
                adj_numbers.sort_unstable();
                adj_numbers.dedup();
                if adj_numbers.len() == 2 {
                    p2 += adj_numbers[0] * adj_numbers[1];
                }
            }
        }
    }
    (p1, p2)
}



fn is_symbol(c: u8) -> bool {
    c != b'.' && !c.is_ascii_digit()
}

fn bytes_to_int(bytes: &[u8]) -> u32 {
    let mut n = 0;
    for &c in bytes {
        n = n * 10 + (c - b'0') as u32;
    }
    n
}

fn part_member(grid: &Grid<u8>, y: i64, x1: i64, x2: i64, i: usize) -> Option<u32> {
    let mut it_x = (x1-1).max(0)..(x2+2).min(grid.height as i64);
    let it_y = (y-1).max(0)..(y+2).min(grid.height as i64);
    
    if it_x.any(|x| it_y.clone().any(|y| is_symbol(grid[(x, y)]))) {
        let i2 = i - (x2 - x1 + 1) as usize;
        Some(bytes_to_int(&grid.vec[i2..i]))
    } else {
        None
    }
}
