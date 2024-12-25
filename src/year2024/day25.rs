use anyhow::*;

const MASK: u32 = 0b1000_1000_1000_1000_1000;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let mut bot = Vec::with_capacity(256);
    let mut top = Vec::with_capacity(256);
    let mut grid = input.as_bytes();

    loop {
        if grid[0] == b'#' {
            let mut encoding = 0u32;
            for i in 6..11 {
                let j = grid[i..].iter().step_by(6).position(|&c|
                    c == b'.'
                ).unwrap();
                encoding = encoding << 4 | j as u32;
            }
            top.push(encoding);
        } else {
            let mut encoding = 0u32;
            for i in 6..11 {
                let j = grid[i..].iter().step_by(6).position(|&c|
                    c == b'#'
                ).unwrap();
                encoding = encoding << 4 | (7 - j as u32);
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