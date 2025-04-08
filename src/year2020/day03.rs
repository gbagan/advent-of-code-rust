use crate::util::grid::Grid;

pub fn solve(input: &str) -> (u64, u64) {
    let grid = Grid::parse(input).unwrap();
    
    let p1 = toboggan(&grid, 3, 1);
    let p2 = p1
                * toboggan(&grid, 1, 1)
                * toboggan(&grid, 5, 1)
                * toboggan(&grid, 7, 1)
                * toboggan(&grid, 1, 2);

    (p1, p2)
}

fn toboggan(grid: &Grid<u8>, slope_right: usize, slope_down: usize) -> u64 {
    let mut counter = 0;
    let mut x = 0;
    for y in (0..grid.height).step_by(slope_down) {
        if grid[(x, y)] == b'#' {
            counter += 1;
        }
        x = (x + slope_right) % grid.width;
    }
    counter
}