use anyhow::*;

const MASK: u32 = 0b1000_1000_1000_1000_1000;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let mut bot = vec!();
    let mut top = vec!();
    let mut grid = input.as_bytes();

    loop {
        if grid[0] == b'#' {
            let mut encoding = 0u32;
            for i in 0..5 {
                let j = (0..6).find(|&j|
                    grid[6 + i + 6 * j] == b'.'
                ).unwrap();
                encoding |= (j << (4 * i)) as u32;
            }
            top.push(encoding);
        } else {
            let mut encoding = 0u32;
            for i in 0..5 {
                let j = (0..6).find(|&j|
                    grid[6 + i + 6 * j] == b'#'
                ).unwrap();
                encoding |= ((7 - j) << (4 * i)) as u32;
            }
            bot.push(encoding);
        }

        if grid.len() <= 42 {
            break;
        }
        grid = &grid[43..];
    }

    let mut p1 = 0;

    for x in top {
        for &y in &bot {
            p1 += ((x + y) & MASK == 0) as u32;
        }
    }

    Ok((p1, 0))
}