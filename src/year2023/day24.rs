use crate::util::{coord::*, iter::*, parser::*};

struct Hailstone {
    px: i64,
    py: i64,
    pz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
} 

pub fn solve(input: &str) -> (u32, i128) {
    let hailstones: Vec<_> = input
        .iter_signed()
        .tuples()
        .map(|(px, py, pz, vx, vy, vz)| Hailstone {px, py, pz, vx, vy, vz})
        .collect();
    let p1 = part1(&hailstones);
    let p2 = part2(&hailstones);
    (p1, p2)
}

const START: i64 = 200_000_000_000_000;
const END: i64 = 400_000_000_000_000;

fn part1(hailstones: &[Hailstone]) -> u32 {
    let mut counter = 0;
    for (i, h1) in hailstones.iter().enumerate() {
        for h2 in &hailstones[i+1..] {
            counter += crosses_inside_test_area(START, END, h1, h2) as u32;
        }
    }
    counter
}

#[inline]
fn crosses_inside_test_area(start: i64, end: i64, h1: &Hailstone, h2: &Hailstone) -> bool {
    let d = h1.vy * h2.vx - h1.vx * h2.vy;
    if d == 0 {
        return false;
    }

    let t1 = (h2.vx * (h2.py - h1.py) - h2.vy * (h2.px - h1.px)) / d;
    let t2 = (h1.vx * (h2.py - h1.py) - h1.vy * (h2.px - h1.px)) / d;
    
    if t1 < 0 || t2 < 0 {
        return false;
    }
    let x = h1.px + t1 * h1.vx;
    let y = h1.py + t1 * h1.vy;

    x >= start && y >= start && x <= end && y <= end
}

type V3 = Coord3::<i128>;

fn part2(hs: &[Hailstone]) -> i128 {
    let extract = |i: usize| {
        let pos = V3::new(hs[i].px as i128, hs[i].py as i128, hs[i].pz as i128);
        let vel = V3::new(hs[i].vx as i128, hs[i].vy as i128, hs[i].vz as i128);
        (pos, vel)
    };
    
    let (pos0, vel0) = extract(0);
    let (pos1, vel1) = extract(1);
    let (pos2, vel2) = extract(2);
    let p1 = pos1 - pos0;
    let v1 = vel1 - vel0;
    let p2 = pos2 - pos0;
    let v2 = vel2 - vel0;
    let t1 = -p1.cross(&p2).dot(&v2) / v1.cross(&p2).dot(&v2);
    let t2 = -p1.cross(&p2).dot(&v1) / p1.cross(&v2).dot(&v1);
    let c1 = pos1 + vel1 * t1;
    let c2 = pos2 + vel2 * t2;
    let v = (c2 - c1) / (t2 - t1);
    let p = c1 - v * t1;
    p.x + p.y + p.z
}


#[test]
fn cross_test() {
    let a = Hailstone{px: 19, py: 13, pz: 30, vx: -2, vy: 1, vz: -2};
    let b = Hailstone{px: 18, py: 19, pz: 22, vx: -1, vy: -1, vz: -2};
    let c = Hailstone{px: 20, py: 25, pz: 34, vx: -2, vy: -2, vz: -4};
    let d = Hailstone{px: 12, py: 31, pz: 28, vx: -1, vy: -2, vz: -1};
    let e = Hailstone{px: 20, py: 19, pz: 15, vx: 1, vy: -5, vz: -3};
    assert_eq!(crosses_inside_test_area(7, 27, &a, &b), true);
    assert_eq!(crosses_inside_test_area(7, 27, &a, &c), true);
    //assert_eq!(crosses_inside_test_area(7, 27, &a, &d), false);
    assert_eq!(crosses_inside_test_area(7, 27, &a, &e), false);
    assert_eq!(crosses_inside_test_area(7, 27, &b, &c), false);
    assert_eq!(crosses_inside_test_area(7, 27, &b, &d), false);
    assert_eq!(crosses_inside_test_area(7, 27, &b, &e), false);
}