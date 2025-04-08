use itertools::Itertools;
use crate::util::{parser::*, range::Range};

pub fn solve(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;
    for (x1, y1, x2, y2) in input.iter_unsigned::<u32>().tuples() {
        let r1 = Range::new(x1, y1);
        let r2 = Range::new(x2, y2);
        if r1.fully_contains(&r2) || r2.fully_contains(&r1) {
            p1 += 1;
        }
        if r1.overlaps(&r2) {
            p2 += 1;
        }
    }

    (p1, p2)
}