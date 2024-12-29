use std::simd::{u64x8, Simd};

type Lights = [u64x8; 100];

pub fn solve(input: &str) -> (u32, u32) {
    let mut grid: Lights = [Simd::splat(0); 100];
 
    for (y, row) in input.lines().enumerate() {
        let mut t = [0; 8];
        for (x, c) in row.chars().enumerate() {
            let offset = 4 * (15 - (x % 16));
            let bit = if c == '#' {1u64} else {0u64};
            t[x / 16] |= bit << offset;
        }
        grid[y] = Simd::from_array(t);
    }
    
    let p1 = game_of_life(&grid, false);
    let p2 = game_of_life(&grid, true);
    (p1, p2)
}

fn game_of_life(input: &Lights, part_two: bool) -> u32 {
    let bit_mask = Simd::from_array([
        0x1111_1111_1111_1111,
        0x1111_1111_1111_1111,
        0x1111_1111_1111_1111,
        0x1111_1111_1111_1111,
        0x1111_1111_1111_1111,
        0x1111_1111_1111_1111,
        0x1111_0000_0000_0000,
        0
    ]);
    let left_mask = Simd::from_array([
        0,
        0xf000_0000_0000_0000,
        0xf000_0000_0000_0000,
        0xf000_0000_0000_0000,
        0xf000_0000_0000_0000,
        0xf000_0000_0000_0000,
        0xf000_0000_0000_0000,
        0xf000_0000_0000_0000,
    ]);
    
    let right_mask = Simd::from_array([
        0x0000_0000_0000_000f,
        0x0000_0000_0000_000f,
        0x0000_0000_0000_000f,
        0x0000_0000_0000_000f,
        0x0000_0000_0000_000f,
        0x0000_0000_0000_000f,
        0x0000_0000_0000_000f,
        0
    ]);

    let part2_mask = Simd::from_array([
        1 << 60, 0, 0, 0, 0, 0, 1 << 48, 0
    ]);

    let mut grid = *input;
    let mut hsum: [u64x8; 100] = [Simd::splat(0); 100];
    let mut next: [u64x8; 100] = [Simd::splat(0); 100];

    if part_two {
        grid[0] |= part2_mask;
        grid[99] |= part2_mask;
    }

    for _ in 0..100 {
        for y in 0..100 {
            let v = grid[y];
            let left = v.rotate_elements_right::<1>() << 60 & left_mask;
            let right = v.rotate_elements_left::<1>() >> 60 & right_mask;
            hsum[y] = v + (v >> 4) + (v << 4) + left + right;
        }

        for y in 0..100 {
            let mut sum = hsum[y] - grid[y];
            if y > 0 {
                sum += hsum[y-1];
            }
            if y < 99 {
                sum += hsum[y+1];
            }

            let a = sum >> 3;
            let b = sum >> 2;
            let c = sum >> 1;
            let d = sum | grid[y];

            next[y] = !a & !b & c & d & bit_mask;
        }

        if part_two {
            next[0] |= part2_mask;
            next[99] |= part2_mask;
        }

        (grid, next) = (next, grid);
    }
    grid.iter().map(|n| n.to_array().iter().map(|x| x.count_ones()).sum::<u32>()).sum()
}
