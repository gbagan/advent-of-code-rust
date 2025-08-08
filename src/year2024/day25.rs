use std::simd::prelude::*;

pub fn solve(input: &str) -> (u32, u32) {
    let mut top = [0; 250];
    let mut bot = [u32::MAX; 256];
    let mut grid = input.as_bytes();

    let mut i = 0;
    let mut j = 0;

    let mask = u8x32::splat(b'#');
    loop {
        let encoding = u8x32::from_slice(&grid[6..38]).simd_eq(mask).to_bitmask() as u32;

        if grid[0] == b'#' {
            top[i] = encoding;
            i += 1;
        } else {
            bot[j] = encoding;
            j += 1;
        }
        if grid.len() <= 42 {
            break;
        }
        grid = &grid[43..];
    }

    let mut p1 = i32x16::splat(0);
 
    for x in top {
        let x = u32x16::splat(x);
        for &y in bot.as_chunks::<16>().0 {
            let y = u32x16::from_array(y);
            p1 += (x & y).simd_eq(u32x16::splat(0)).to_int();
        }
    }

    (p1.reduce_sum() as u32, 0)
}
