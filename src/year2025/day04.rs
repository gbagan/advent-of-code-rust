use crate::util::grid::Grid;

pub fn solve(input: &str) -> (usize, u32) {
    let grid = Grid::parse_with_padding(input, b'.');
    let width = grid.width;
    let grid = grid.vec;
    let mut nbors = vec![255; grid.len()];
    let mut stack = Vec::with_capacity(1000);

    for (i, &c) in grid.iter().enumerate() {
        if c == b'.'  {
            continue;
        }
        let nbor = 
            (grid[i-width-1] == b'@') as u8
            + (grid[i-width] == b'@') as u8
            + (grid[i-width+1] == b'@') as u8
            + (grid[i-1] == b'@') as u8
            + (grid[i+1] == b'@') as u8
            + (grid[i+width-1] == b'@') as u8
            + (grid[i+width] == b'@') as u8
            + (grid[i+width+1] == b'@') as u8;
        if nbor < 4 {
            stack.push(i);
        } else {
            nbors[i] = nbor;
        }
    }

    let p1 = stack.len();
    let mut p2 = 0;

    macro_rules! do_next {
        ($i: expr) => {
            nbors[$i] -= 1;
            if nbors[$i] == 3 {
                stack.push($i);
            }
        }
    }


    while let Some(i) = stack.pop() {
        p2 += 1;
        do_next!(i-width-1);
        do_next!(i-width);
        do_next!(i-width+1);
        do_next!(i-1);
        do_next!(i+1);
        do_next!(i+width-1);
        do_next!(i+width);
        do_next!(i+width+1);
    }

    (p1, p2)
}