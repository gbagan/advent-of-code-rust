use crate::util::grid::Grid;

const CLEAN: usize = 1;
const INFECTED: usize = 3;
const WIDTH: usize = 500;
const MIDDLE: usize = WIDTH / 2;

pub fn solve(input: &str) -> (u32, u32) {
    let grid = Grid::parse(input);
    let p1 = simulate::<2>(&grid, 10_000);
    let p2 = simulate::<1>(&grid, 10_000_000);
    (p1, p2)
}

fn simulate<const RULE: usize>(inner: &Grid<u8>, bursts: usize) -> u32 {
    const DIRECTIONS: [usize; 4] = [1, WIDTH, 0_usize.wrapping_sub(1), 0_usize.wrapping_sub(WIDTH)];
    let mut grid = [CLEAN as u8; WIDTH * WIDTH];
    let offset = MIDDLE - (inner.width / 2);
    for y in 0..inner.height {
        for x in 0..inner.width {
            if inner[(x, y)] == b'#' {
                let index = WIDTH * (offset + y) + offset + x;
                grid[index] = INFECTED as u8;
            }
        }
    }
    let mut position = WIDTH * MIDDLE + MIDDLE;
    let mut direction = 3;
    let mut counter = 0;

    for _ in 0..bursts {
        let tile = grid[position] as usize;
        let next_tile = (tile + RULE) & 3;
        grid[position] = next_tile as u8;
        direction = (direction + tile + 2) & 3;
        position = position.wrapping_add(DIRECTIONS[direction]);
        counter += (next_tile == INFECTED) as u32;
    }

    counter
}