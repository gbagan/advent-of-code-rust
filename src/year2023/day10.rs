// Shoelace formula and Pick theorem for Part 2

use memchr::memchr;

enum Dir {
    North, South, West, East
}

pub fn solve(input: &str) -> (i32,i32) {
    let grid = input.as_bytes();
    let width = memchr(b'\n', grid).unwrap() + 1;
    let start = memchr(b'S', grid).unwrap();
    let mut current = start;
    let mut dir =
        if matches!(grid[start - 1], b'-' | b'L' | b'F' ) {
            Dir::West 
        } else if matches!(grid[start + 1], b'-' | b'J' | b'7') {
            Dir::East
        } else {
            Dir::North
        };

    let mut length = 0;
    let mut area = 0i32;
    let mut x = 0;
    let mut y = 0;

    loop {
        let mut steps = 1;
        match dir {
            Dir::West => {
                current -= 1;
                while grid[current] == b'-' {
                    steps += 1;
                    current -= 1;
                }
                length += steps;
                x -= steps;
                area += steps * y;
                dir = if grid[current] == b'L' { Dir::North } else { Dir::South }
            },
            Dir::East => {
                current += 1;
                while grid[current] == b'-' {
                    steps += 1;
                    current += 1;
                }
                length += steps;
                x += steps;
                area -= steps * y;
                dir = if grid[current] == b'J' { Dir::North } else { Dir::South }
            },
            Dir::North => {
                current -= width;
                while grid[current] == b'|' {
                    steps += 1;
                    current -= width;
                }
                length += steps;
                y -= steps;
                area -= steps * x;
                dir = if grid[current] == b'7' { Dir::West } else { Dir::East }
            },
            Dir::South => {
                current += width;
                while grid[current] == b'|' {
                    steps += 1;
                    current += width;
                }
                length += steps;
                y += steps;
                area += steps * x;
                dir = if grid[current] == b'J' { Dir::West } else { Dir::East }
            },
        }
        if current == start {
            break;
        }
    }
    let p1 = length / 2;
    let p2 = (area.abs() - length) / 2 + 1;
    (p1, p2)
}