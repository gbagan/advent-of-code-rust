use std::simd::prelude::*;

const HEIGHT: usize = 210;
const START: usize = 70;

#[derive(Clone)]
pub struct Input {
    grid: [u64x4; HEIGHT],
    north: [u64x4; HEIGHT],
    south: [u64x4; HEIGHT],
    west: [u64x4; HEIGHT],
    east: [u64x4; HEIGHT],
}

enum Direction {
    North,
    South,
    West,
    East,
}
use Direction::*;

pub fn solve(input: &str) -> (u64, u32) {
    let north = [u64x4::splat(0); HEIGHT];
    let south = [u64x4::splat(0); HEIGHT];
    let west = [u64x4::splat(0); HEIGHT];
    let east = [u64x4::splat(0); HEIGHT];

    let mut grid = [[0; 4]; HEIGHT];
    
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            let j = START + j;
            if c == b'#' {
                if j < 64 {
                    grid[i+START][0] |= 1 << (63 - j)
                } else if j < 128 {
                    grid[i+START][1] |= 1 << (127 - j)
                } else if j < 192 {
                    grid[i+START][2] |= 1 << (191 - j)
                } else {
                    grid[i+START][3] |= 1 << (255 - j)
                }
            }
        }
    }
    let grid = grid.map(u64x4::from_array);

    let mut input = Input {grid, north, south, west, east};
    let mut directions = [North, South, West, East];

    for _ in 0..10 {
        step(&mut input, &mut directions);
    }

    let zero = u64x4::splat(0);
    let v64 = u64x4::splat(64);

    let elves= input.grid.iter().map(|s| s.count_ones()).sum::<u64x4>().reduce_sum();
    let mut leading_zeros = v64;
    let mut trailing_zeros = v64;
    for &v in &input.grid[70..150] {
        leading_zeros = leading_zeros.simd_min(v.leading_zeros());
        trailing_zeros = trailing_zeros.simd_min(v.trailing_zeros());
    }
    let i = leading_zeros.simd_ne(v64).first_set().unwrap();
    let xmin = i as u64 * 64 + leading_zeros.as_array()[i];
    
    trailing_zeros = trailing_zeros.reverse();
    let i = trailing_zeros.simd_ne(v64).first_set().unwrap();
    let xmax = 255 - i as u64 * 64 - trailing_zeros.as_array()[i];

    let ymin = input.grid.iter().position(|&v| v != zero).unwrap() as u64;
    let ymax = input.grid.iter().rposition(|&v| v != zero).unwrap() as u64;
    let p1 = (xmax - xmin + 1) * (ymax - ymin + 1) - elves;

    let mut i = 10;
    let p2 = loop {
        i += 1;
        if !step(&mut input, &mut directions) {
            break i;
        }
    };

    (p1, p2)
}

fn step(input: &mut Input, directions: &mut [Direction]) -> bool {
    let zero = u64x4::splat(0);
    let Input { grid, north, south, west, east } = input;
    let start = grid.iter().position(|&s| s != zero).unwrap() - 1;
    let end = grid.iter().rposition(|&s| s != zero).unwrap() + 2;

    let mut prev = !grid[start-1];
    let mut cur = prev;

    for i in start..end {
        let next = !(shl(grid[i + 1]) | grid[i + 1] | shr(grid[i + 1]));
        let mut can_up = prev;
        let mut can_down = next;
        let vertical = !(grid[i-1] | grid[i] | grid[i+1]);
        let mut can_left = shr(vertical);
        let mut can_right = shl(vertical);
        let mut can_move = grid[i] & !(can_up & can_down & can_left & can_right);

        for dir in directions.iter() {
            match dir {
                North => {
                    can_up &= can_move;
                    can_move &= !can_up;
                }
                South => {
                    can_down &= can_move;
                    can_move &= !can_down;
                }
                West => {
                    can_left &= can_move;
                    can_move &= !can_left;
                }
                East => {
                    can_right &= can_move;
                    can_move &= !can_right;
                }
            }
        }

        north[i - 1] = can_up;
        south[i + 1] = can_down;
        west[i] = shl(can_left);
        east[i] = shr(can_right);

        prev = cur;
        cur = next;
    }

    for i in start..end {
        let up = north[i];
        let down = south[i];
        let left = west[i];
        let right = east[i];
        north[i] &= !down;
        south[i] &= !up;
        west[i] &= !right;
        east[i] &= !left;
    }

    let mut change = false;
    for i in start..end {
        let stay = grid[i] & !(north[i - 1] | south[i + 1] | shr(west[i]) | shl(east[i]));
        let moved = north[i] | south[i] | west[i] | east[i];
        grid[i] = stay | moved;
        change |= moved != zero;
    }
    
    directions.rotate_left(1);

    change
}

#[inline]
fn shl(v: u64x4) -> u64x4 {
    v << 1 | v.rotate_elements_left::<1>() >> 63
}

#[inline]
fn shr(v: u64x4) -> u64x4 {
    v >> 1 | v.rotate_elements_right::<1>() << 63
}