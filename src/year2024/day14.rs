use anyhow::*;
use num_integer::Integer;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(u32, i32)> {
    let robots: Vec<_> =
        input
        .iter_signed::<i32>()
        .array_chunks()
        .collect();

    let p1 = part1(&robots);
    let p2 = part2(&robots).context("Part 2: No solution found")?;

    Ok((p1, p2))
}

fn part1(robots: &[[i32; 4]]) -> u32 {
    let mut quadrant1 = 0;
    let mut quadrant2 = 0;
    let mut quadrant3 = 0;
    let mut quadrant4= 0;

    for [px, py, vx, vy] in robots {
        let px = (px + 100 * vx).mod_floor(&101);
        let py = (py + 100 * vy).mod_floor(&103);
        if px < 50 {
            if py < 51 {
                quadrant1 += 1;
            } else if py > 51 {
                quadrant2 += 1;
            }
        } else if px > 50 {
            if py < 51 {
                quadrant3 += 1;
            } else if py > 51 {
                quadrant4 += 1;
            }
        }
    }

    quadrant1 * quadrant2 * quadrant3 * quadrant4
}

fn part2(robots: &[[i32; 4]]) -> Option<i32> {
    const PERIOD: i32 = 101 * 103;
    const PX: i32 = 51 * 103;
    const PY: i32 = 51 * 101;

    let tx = (0..101).find(|t| {
        let mut left = 0;   
        for [px, _, vx, _] in robots {
            if (px + t * vx).mod_floor(&101) < 51 {
                left += 1
            }
        }
        left < 150 || robots.len() as u32 - left < 150
    })?;

    let ty = (0..103).find(|t| {
        let mut top = 0;   
        for [_, py, _, vy] in robots {
            if (py + t * vy).mod_floor(&103) < 52 {
                top += 1
            }
        }
        top < 150 || robots.len() as u32 - top < 150
    })?;

    // we search for the moment t such that t = tx [mod 101] and t = ty [mod 103] 
    
    Some((PX * tx + PY * ty) % PERIOD)
}