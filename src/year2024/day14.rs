use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, i32) {
    let robots: Vec<_> =
        input
        .iter_signed::<i32>()
        .array_chunks()
        .map(|[px, py, vx, vy]| [px, py, vx+101, vy+103])
        .collect();

    let p1 = part1(&robots);
    let p2 = part2(&robots);

    (p1, p2)
}

fn part1(robots: &[[i32; 4]]) -> u32 {
    let mut quadrant1 = 0;
    let mut quadrant2 = 0;
    let mut quadrant3 = 0;
    let mut quadrant4 = 0;

    for [px, py, vx, vy] in robots {
        let px = (px + 100 * vx) % 101;
        let py = (py + 100 * vy) % 103;
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

fn part2(robots: &[[i32; 4]]) -> i32 {
    const PERIOD: i32 = 101 * 103;
    const PX: i32 = 51 * 103;
    const PY: i32 = 51 * 101;

    let tx = (0..101).map(|t| {
        let mut left = 0;
        let mut right = 0;  
        for [px, _, vx, _] in robots {
            let index = (px + t * vx) % 101;
            if index < 25 {
                left += 1;
            } else if index > 101-25 {
                right += 1;
            }
        }
        let max = left.min(right);
        (t, max)
    }).min_by_key(|x| x.1).unwrap().0;

    let ty = (0..103).map(|t| {
        let mut top = 0;
        let mut bot = 0;  
        for [_, py, _, vy] in robots {
            let index = (py + t * vy) % 103;
            if index < 25 {
                top += 1;
            } else if index > 103-25 {
                bot += 1;
            }
        }
        let max = top.min(bot);
        (t, max)
    }).min_by_key(|x| x.1).unwrap().0;

    // we search for the moment t such that t = tx [mod 101] and t = ty [mod 103] 
    
    (PX * tx + PY * ty) % PERIOD
}