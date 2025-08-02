pub fn solve(input: &str) -> (u32, u32) {    
    let mut grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let width = grid[0].len();

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..width {
        let ones = count_ones(&grid, i);
        let majority = (ones + ones >= grid.len()) as u32;
        gamma = gamma << 1 | majority;
        epsilon = epsilon << 1 | (1 - majority);
    }

    let p1 = gamma * epsilon;

    let mut grid2 = grid.clone();
    filter::<true>(&mut grid, width);
    filter::<false>(&mut grid2, width);

    let p2 = to_int(grid[0]) * to_int(grid2[0]);

    (p1, p2)
}

#[inline]
fn count_ones(grid: &[&[u8]], idx: usize) -> usize {
    grid.iter().filter(|row| row[idx] == b'1').count()
}

fn to_int(bytes: &[u8]) -> u32 {
    let mut n = 0;
    for &b in bytes {
        n = (2 * n) | (b == b'1') as u32;
    }
    n
}

fn filter<const PART1: bool>(grid: &mut Vec<&[u8]>, width: usize) { 
    for i in 0..width {
        let count = count_ones(grid, i);
        let bit = if PART1 {
            if count + count >= grid.len() { b'1' } else { b'0' }
        } else {
            if count + count < grid.len() { b'1' } else { b'0' }
        };
        let mut j = 0;
        while j < grid.len() {
            if grid[j][i] == bit {
                j += 1;
            } else {
                grid.swap_remove(j);
            }
        }
        if grid.len() == 1 {
            break;
        }
    }
}