use itertools::Itertools;
use ordered_float::OrderedFloat;
use crate::util::{math::solve_linear_system, parser::*};

pub struct Hailstone {
    pub px: i64,
    pub py: i64,
    pub pz: i64,
    pub vx: i64,
    pub vy: i64,
    pub vz: i64,
} 

fn parse_line(line: &str) -> Option<Hailstone>{
    let (px, py, pz, vx, vy, vz) = line.iter_unsigned().next_tuple()?;
    Some(Hailstone {px, py, pz, vx, vy, vz})
}

pub fn solve(input: &str) -> Option<(u32, i64)> {
    let hailstones: Vec<_> = input.lines().filter_map(parse_line).collect();
    let p1 = part1(&hailstones);
    let p2 = part2(&hailstones)?;
    Some((p1, p2))
}

fn crosses_inside_test_area(start: i64, end: i64, h1: &Hailstone, h2: &Hailstone) -> bool {
    let d = h1.vy * h2.vx - h1.vx * h2.vy;
    if d == 0 {
        return false;
    }

    let t1 = (h2.vx * (h2.py - h1.py) - h2.vy * (h2.px - h1.px)) as f64 / d as f64;
    let t2 = (h1.vx * (h2.py - h1.py) - h1.vy * (h2.px - h1.px)) as f64 / d as f64;
    
    if t1 < 0.0 || t2 < 0.0 {
        return false;
    }
    let x = h1.px as f64 + t1 * h1.vx as f64;
    let y = h1.py as f64 + t1 * h1.vy as f64;

    let start = start as f64;
    let end = end as f64;

    x >= start && y >= start && x <= end && y <= end
}

pub fn part1(hailstones: &[Hailstone]) -> u32 {
    let start = 200_000_000_000_000i64;
    let end = 400_000_000_000_000i64;
    let mut counter = 0;
    for (i, h1) in hailstones.iter().enumerate() {
        for h2 in &hailstones[i+1..] {
            if crosses_inside_test_area(start, end, h1, h2) {
                counter += 1;
            }
        }
    }
    counter
}

fn build_equations(h: &Hailstone) -> Vec<Vec<i64>> {
    let Hailstone {px, py, pz, vx, vy, vz} = h;
    vec!(vec!(*vy, -vx, 0, -py, *px, 0, px * vy - py * vx),
         vec!(*vz, 0, -vx, -pz, 0, *px, px * vz - pz * vx),
         vec!(0, *vz, -vy, 0, -pz, *py, py * vz - pz * vy)
        )
}

fn diff_equations(e1: &[Vec<i64>], e2: &[Vec<i64>]) -> Vec<Vec<OrderedFloat<f64>>> {
    let n = e1.len();
    let mut res = vec!();
    for i in 0..n {
        res.push(e1[i]
            .iter()
            .zip(&e2[i])
            .map(|(x, y)| OrderedFloat((x - y) as f64))
            .collect());
    }
    res
}

pub fn part2(hs: &[Hailstone]) -> Option<i64> {
    let e1 = build_equations(&hs[0]);
    let e2 = build_equations(&hs[1]);
    let e3 = build_equations(&hs[2]);
    let mut eqs = diff_equations(&e1, &e2);
    eqs.append(&mut diff_equations(&e2, &e3));
    let sol = solve_linear_system(&eqs)?;
    Some((sol[0].into_inner() + sol[1].into_inner() + sol[2].into_inner()).round() as i64)
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
    assert_eq!(crosses_inside_test_area(7, 27, &a, &d), false);
    assert_eq!(crosses_inside_test_area(7, 27, &a, &e), false);
    assert_eq!(crosses_inside_test_area(7, 27, &b, &c), false);
    assert_eq!(crosses_inside_test_area(7, 27, &b, &d), false);
    assert_eq!(crosses_inside_test_area(7, 27, &b, &e), false);
}