use itertools::Itertools;
use crate::util::{coord::Coord, grid::Grid, parser::*};

type Pixel = Coord<i32>;

pub fn solve(input: &str) -> (usize, String) {
    let mut pixels = vec!();
    for line in input.lines() {
        if let Some(suffix) = line.strip_prefix("rect") {
            let (width, height) = suffix.iter_unsigned().next_tuple().unwrap();
            for x in 0..width {
                for y in 0..height {
                    pixels.push(Pixel::new(x, y));
                }
            }
        } else if let Some(suffix) = line.strip_prefix("rotate row") {
            let (row, shifts) = suffix.iter_unsigned().next_tuple().unwrap();
            pixels.iter_mut().for_each(|p| {
                if p.y == row {
                    p.x = (p.x + shifts) % 50;
                }
            })
        } else if let Some(suffix) = line.strip_prefix("rotate column") {
            let (column, shifts) = suffix.iter_unsigned().next_tuple().unwrap();
            pixels.iter_mut().for_each(|p| {
                if p.x == column {
                    p.y = (p.y + shifts) % 6;
                }
            })
        } else {
            panic!("Expecting 'rect', 'rotate row' or 'rotate column'");
        }
    }
    
    let p1 = pixels.len();

    let mut grid = Grid::new(50, 6, ' ');
    for pixel in pixels {
        grid[pixel] = '#';
    }

    let p2 = grid.draw();

    (p1, p2)
}