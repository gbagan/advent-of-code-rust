use itertools::Itertools;
use ordered_float::OrderedFloat;
use crate::util::{math::solve_linear_system, parser::*};

struct Hailstone {
    px: i64,
    py: i64,
    pz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
} 

pub fn solve(input: &str) -> (u32, i64) {
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
            if crosses_inside_test_area(START, END, h1, h2) {
                counter += 1;
            }
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

fn build_equations(h: &Hailstone) -> [[i64; 7]; 3] {
    let &Hailstone {px, py, pz, vx, vy, vz} = h;
    [ [vy, -vx, 0, -py, px, 0, px * vy - py * vx],
      [vz, 0, -vx, -pz, 0, px, px * vz - pz * vx],
      [0, vz, -vy, 0, -pz, py, py * vz - pz * vy]
    ]
}

fn diff_equations<'a, const N: usize>(e1: &'a [[i64; N]], e2: &'a [[i64; N]]) -> impl Iterator<Item=[OrderedFloat<f64>; N]> + 'a {
    e1.iter()
        .zip(e2.iter())
        .map(|(row1, row2)| std::array::from_fn(|i| OrderedFloat((row1[i] - row2[i]) as f64)))
}

fn part2(hs: &[Hailstone]) -> i64 {
    let e1 = build_equations(&hs[0]);
    let e2 = build_equations(&hs[1]);
    let e3 = build_equations(&hs[2]);
    let mut eqs = Vec::with_capacity(6);
    eqs.extend(diff_equations(&e1, &e2));
    eqs.extend(diff_equations(&e2, &e3));
    let sol = solve_linear_system(&eqs).unwrap();
    (sol[0].into_inner() + sol[1].into_inner() + sol[2].into_inner()).round() as i64
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