use itertools::Itertools;
use crate::util::parser::*;

pub fn solve(input: &str) -> Option<(u32, u32)> {
    let mut p1 = 0;
    let mut p2 = 0;
    for (l, h, w) in input.iter_unsigned::<u32>().tuples() {
        let areas = [l*w, l*h, w*h];
        let sum_areas: u32 = areas.into_iter().sum();
        p1 += 2 * sum_areas + areas.into_iter().min().unwrap();
        p2 += l * h * w + 2 * (l+w).min(l+h).min(w+h);
    }
    Some((p1, p2))
}