use crate::util::grid::Grid;

const CLEAN: usize = 1;
const INFECTED: usize = 3;

pub fn solve(input: &str) -> Option<(u32, u32)> {
    let grid = Grid::parse(input);
    let p1 = simulate(&grid, 10_000, 2);
    let p2 = simulate(&grid, 10_000_000, 1);
    Some((p1, p2))
}

fn simulate(inner: &Grid<u8>, bursts: usize, rule: usize) -> u32 {
    let width = 500;
    let middle = 250;
    let directions = [1, width, 0_usize.wrapping_sub(1), 0_usize.wrapping_sub(width)];
    let mut grid: Vec<u8> = vec![CLEAN as u8; width * width];
    let offset = middle - (inner.width / 2);
    for y in 0..inner.height {
        for x in 0..inner.width {
            if inner[(x, y)] == b'#' {
                let index = width * (offset + y) + offset + x;
                grid[index] = INFECTED as u8;
            }
        }
    }
    let mut position = width * middle + middle;
    let mut direction = 3;
    let mut counter = 0;

    for _ in 0..bursts {
        let tile = grid[position] as usize;
        let next_tile = (tile + rule) & 0x3;
        grid[position] = next_tile as u8;
        direction = (direction + tile + 2) & 0x3;
        position = position.wrapping_add(directions[direction]);
        if next_tile == INFECTED {
            counter += 1;
        }
    }

    counter
}