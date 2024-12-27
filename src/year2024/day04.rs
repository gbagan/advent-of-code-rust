use memchr::memchr;

const XMAS: u32 = 0x584d4153;
const SAMX: u32 = 0x53414d58;

pub fn solve(input: &str) -> (u32, u32) {
    let grid = input.as_bytes();
    let size = memchr(b'\n', grid).unwrap();
    let p1 = part1(&grid, size);
    let p2 = part2(&grid, size);
    (p1, p2)
}

fn part1(grid: &[u8], size: usize) -> u32 {
    let width = size + 1;
    let mut count = 0;

    for i in 0..size {
        count += count_line(&grid, i, size, width);
        count += count_line(&grid, i * width, size, 1);
    }

    for i in 0..width-3 {
        count += count_line(&grid, i,size - i, width + 1);
        count += count_line(&grid, size - i - 1, size - i, width - 1);
    }

    for i in 1..width-3 {
        count += count_line(&grid, i * width, size - i, width + 1);
        count += count_line(&grid, i * width + size - 1, size - i, width - 1);
    }

    count
}

fn count_line(grid: &[u8], start: usize, times: usize, step: usize) -> u32 {
    let end = grid.len().min(start + step * times);
    let mut word = 0;
    let mut count = 0;
    
    for &c in grid[start..end].iter().step_by(step) {
        word = (word << 8) | c as u32;
        count += (word == XMAS || word == SAMX) as u32;
    }

    count
}


fn part2(grid: &[u8], size: usize) -> u32 {
    let width = size + 1;
    let mut count = 0;
    for i in 1..size - 1 {
        for j in 1..size - 1 {
            let index = i * width + j;
            if grid[index] != b'A' {
                continue
            }
            let a = grid[index - width - 1];
            let b = grid[index - width + 1];
            let c = grid[index + width - 1];
            let d = grid[index + width + 1];
            count += (a.abs_diff(d) == 6 && b.abs_diff(c) == 6) as u32;
        }
    }
    count
}