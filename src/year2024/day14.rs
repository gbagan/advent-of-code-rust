use anyhow::*;
use num_integer::Integer;
use crate::util::parser::*;
use std::thread;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

pub fn solve(input: &str) -> Result<(u32, i32)> {
    let robots: Vec<_> =
        input
        .iter_signed::<i32>()
        .array_chunks()
        .map(|[px, py, vx, vy]| Robot { px, py, vx, vy})
        .collect();

    let p1 = part1(&robots);
    let p2 = part2(&robots);

    Ok((p1, p2))
}

fn part1(robots: &[Robot]) -> u32 {
    let mut quadrant1 = 0;
    let mut quadrant2 = 0;
    let mut quadrant3 = 0;
    let mut quadrant4= 0;

    for robot in robots {
        let px = (robot.px + 100 * robot.vx).mod_floor(&101);
        let py = (robot.py + 100 * robot.vy).mod_floor(&103);
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

pub struct Shared {
    done: AtomicBool,
    counter: AtomicI32,
    p2: AtomicI32,
}

fn part2(robots: &[Robot]) -> i32 {
    let shared = Shared {
        done: AtomicBool::new(false),
        counter: AtomicI32::new(1),
        p2: AtomicI32::new(i32::MAX),
    };

    thread::scope(|scope| {
        for _ in 0..thread::available_parallelism().unwrap().get() {
            scope.spawn(|| worker(robots, &shared));
        }
    });

    shared.p2.load(Ordering::Relaxed)
}

const N: i32 = 20;

fn worker(robots: &[Robot], shared: &Shared) {
    while !shared.done.load(Ordering::Relaxed) {
        let counter = shared.counter.fetch_add(N, Ordering::Relaxed);
        let mut grid = [0i32; 101*103];
        for i in counter..counter+N {        
            let mut distinct = true;
            for robot in robots {
                let px = (robot.px + i * robot.vx).mod_floor(&101);
                let py = (robot.py + i * robot.vy).mod_floor(&103);
                let index = (py * 101 + px) as usize;
                if grid[index] == i {
                    distinct = false;
                    break;
                }
                grid[index] = i;
            }
        
            if distinct {
                shared.p2.fetch_min(i, Ordering::Relaxed);
                shared.done.store(true, Ordering::Relaxed);
                return
            }
        }    
    }
}