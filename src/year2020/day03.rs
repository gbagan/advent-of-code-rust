pub fn solve(input: &str) -> (u64, u64) {
    let grid = input.as_bytes();
    let width = grid.iter().position(|&c| c == b'\n').unwrap() + 1;
    let height = grid.len() / width;

    let p1 = toboggan(grid, width, height, 3, 1);
    let p2 = p1 * toboggan(grid, width, height, 1, 1)
                * toboggan(grid, width, height, 5, 1)
                * toboggan(grid, width, height, 7, 1)
                * toboggan(grid, width, height, 1, 2);

    (p1, p2)
}

#[inline]
fn toboggan(grid: &[u8], width: usize, height: usize, slope_right: usize, slope_down: usize) -> u64 {
    let mut counter = 0;
    let mut x = 0;
    for y in (0..height).step_by(slope_down) {
        counter += (grid[y*width+x] == b'#') as u64;
        x += slope_right;
        if x >= width-1 {
            x -= width-1;
        }
    }
    counter
}