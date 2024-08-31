use crate::util::{coord::Coord, grid::Grid, parser::*};

pub fn solve(input: &str) -> Option<(u32, u32)> {    
    let lengths = input.iter_unsigned::<i32>();
    let directions = input.bytes().filter(u8::is_ascii_uppercase).map(to_dir);
    let instrs: Vec<_> = directions.zip(lengths).collect();

    let mut xmin = 0;
    let mut ymin = 0;
    let mut xmax = 0;
    let mut ymax = 0;

    let mut position = Coord::ORIGIN;
    for &(direction, length) in &instrs {
        position += direction * length;
        xmin = xmin.min(position.x);
        ymin = ymin.min(position.y);
        xmax = xmax.max(position.x);
        ymax = ymax.max(position.y);
    }
        
    let p1 = simulate::<2>(&instrs, xmin, ymin, xmax, ymax);
    let p2 = simulate::<10>(&instrs, xmin, ymin, xmax, ymax);
    Some((p1, p2))
}

fn to_dir(c: u8) -> Coord {
    match c {
        b'U' => Coord::NORTH,
        b'L' => Coord::WEST,
        b'R' => Coord::EAST,
        b'D' => Coord::SOUTH,
        _ => panic!("unexpected character {c}"),
    }
}

fn simulate<const N: usize>(instrs: &[(Coord, i32)], xmin: i32, ymin: i32, xmax: i32, ymax: i32) -> u32 {
    let mut distinct = 0;
    let mut seen = Grid::new((xmax - xmin + 1) as usize, (ymax - ymin + 1) as usize, false);
    let head = Coord::new(-xmin, -ymin);
    let mut rope = [head; N];
    for &(direction, length) in instrs {
        for _ in 0..length {
            rope[0] += direction;
            for i in 0..N-1 {
                if !pull_knot(rope[i], &mut rope[i+1]) {
                    break;
                }
            }
            let tail = rope[N-1];
            if !seen[tail] {
                distinct += 1;
                seen[tail] = true;
            }
        }

    }
    distinct
}

#[inline]
fn pull_knot(puller: Coord, pulled: &mut Coord) -> bool {
    let diff = puller - *pulled;
    let sign = Coord::new(diff.x.signum(), diff.y.signum());
    if diff != sign {
        *pulled += sign;
        true
    } else {
        false
    }
}