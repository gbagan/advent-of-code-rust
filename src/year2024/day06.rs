use anyhow::*;
use crate::util::{parallel::*, grid::*};

pub fn solve(input: &str) -> Result<(usize, u32)> {
    let grid = Grid::parse(input)?;
    let mut start = (0, 0);
    'outer: for i in 0..grid.height {
        for j in 0..grid.width {
            if grid[(j, i)] == b'^' {
                start = (j, i);
                break 'outer;
            }       
        }
    }

    let mut seen = Grid::new(grid.width, grid.height, false);

    let (mut currentx, mut currenty) = start;

    seen[(currentx, currenty)] = true;

    'outer: loop {
        loop {
            let nexty = currenty.wrapping_sub(1);
            if nexty >= grid.height {
                break 'outer;
            }
            if grid[(currentx, nexty)] == b'#' {
                break;
            }
            currenty = nexty;
            seen[(currentx, currenty)] = true;
        }

        loop {
            let nextx = currentx + 1;
            if nextx >= grid.width {
                break 'outer;
            }
            if grid[(nextx, currenty)] == b'#' {
                break;
            }
            currentx = nextx;
            seen[(currentx, currenty)] = true;
        }
        loop {
            let nexty = currenty + 1;
            if nexty >= grid.height {
                break 'outer;
            }
            if grid[(currentx, nexty)] == b'#' {
                break;
            }
            currenty = nexty;
            seen[(currentx, currenty)] = true;
        }

        loop {
            let nextx = currentx.wrapping_sub(1);
            if nextx >= grid.width {
                break 'outer;
            }
            if grid[(nextx, currenty)] == b'#' {
                break;
            }
            currentx = nextx;
            seen[(currentx, currenty)] = true;
        }
    }

    let mut vseen: Vec<(usize, usize)> = Vec::new();

    for j in 0..grid.height {
        for i in 0..grid.width {
            if seen[(i, j)] {
                vseen.push((i, j));
            }       
        }
    }
    
    let p1 = vseen.len();

    let p2 = vseen
        .into_par_iter()
        .map(|(obsx, obsy)| {
            has_cycle(&grid, start, *obsx, *obsy) as u32 
        })
        .sum();


    Ok((p1, p2))
}

fn has_cycle(grid: &Grid<u8>, start: (usize, usize), obsx: usize, obsy: usize) -> bool {
    let mut seen = Grid::new(grid.width, grid.height, 0u8);

    let (mut currentx, mut currenty) = start;

    seen[(currentx, currenty)] = 1;

    loop {
        loop {
            let nexty = currenty.wrapping_sub(1);
            if nexty >= grid.height {
                return false;
            }
            if grid[(currentx, nexty)] == b'#' || (currentx, nexty) == (obsx, obsy) {
                break;
            }
            currenty = nexty;
            if seen[(currentx, currenty)] & 1 != 0 {
                return true;
            }
            seen[(currentx, currenty)] |= 1;
        }

        loop {
            let nextx = currentx + 1;
            if nextx >= grid.width {
                return false;
            }
            if grid[(nextx, currenty)] == b'#' || (nextx, currenty) == (obsx, obsy) {
                break;
            }
            currentx = nextx;
            if seen[(currentx, currenty)] & 2 != 0 {
                return true;
            }
            seen[(currentx, currenty)] |= 2;
        }
        loop {
            let nexty = currenty + 1;
            if nexty >= grid.height {
                return false;
            }
            if grid[(currentx, nexty)] == b'#' || (currentx, nexty) == (obsx, obsy) {
                break;
            }
            currenty = nexty;
            if seen[(currentx, currenty)] & 4 != 0 {
                return true;
            }
            seen[(currentx, currenty)] |= 4;
        }

        loop {
            let nextx = currentx.wrapping_sub(1);
            if nextx >= grid.width {
                return false;
            }
            if grid[(nextx, currenty)] == b'#' || (nextx, currenty) == (obsx, obsy) {
                break;
            }
            currentx = nextx;
            if seen[(currentx, currenty)] & 8 != 0 {
                return true;
            }
            seen[(currentx, currenty)] |= 8;
        }
    }
}