const SIZE: usize = 202;

pub fn solve(input: &str) -> (u32, u32) {
    let algo: Vec<_> = input[..512].bytes().map(|c| (c == b'#') as u8).collect();

    let input = &input[514..];

    let width = input.lines().next().unwrap().len();
    let height = input.len() / (width + 1);

    let input = input.as_bytes();

    let mut grid = [algo[511]; SIZE*SIZE];
    let mut grid2 = [algo[0]; SIZE*SIZE];

    let start_y = (SIZE - height) / 2;
    let start_x = (SIZE - width) / 2;
    let mut j = start_y * SIZE + start_x;
    let mut j2 = 0;
    for _ in 0..height {
        for k in 0..width {
            grid[j+k] = (input[j2+k] == b'#') as u8;
        }
        j += SIZE;
        j2 += width + 1;
    }

    for i in 0..2 {
        enhance(&algo, &grid, &mut grid2, width+2*i+2);
        std::mem::swap(&mut grid, &mut grid2);
    }

    let p1 = grid.iter().map(|&c| c as u32).sum();

    for i in 2..50 {
        enhance(&algo, &grid, &mut grid2, width+2*i+2);
        std::mem::swap(&mut grid, &mut grid2);
    }

    let p2 = grid.iter().map(|&c| c as u32).sum();

    (p1, p2)
}

fn enhance(algo: &[u8], input: &[u8], output: &mut [u8], size: usize) {
    let start_y = (SIZE - size) / 2;
    let start_x = (SIZE - size) / 2;
    let start = start_y * SIZE + start_x;

    for j in (start..start+SIZE*size).step_by(SIZE) {
        let mut k = if input[j-1] == 1 { usize::MAX } else { 0 };
        for i in j..j+size {
            k = (k & 0b11011011) << 1
                | (input[i-SIZE+1] as usize) << 6
                | (input[i+1] as usize) << 3
                | input[i+SIZE+1] as usize;
            output[i] = algo[k];
        }
    }
}