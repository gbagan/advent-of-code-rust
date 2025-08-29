use crate::util::{iter::*, parser::*, range::*};

pub fn solve(input: &str) -> (u64, u64) {
    let ranges: Vec<_> = input
        .iter_unsigned::<u64>()
        .tuples()
        .map(|(x, y)| Range::new(x, y))
        .to_disjoint_union();
    let p1 = ranges[0].end + 1;

    let p2 = 0x1_0000_0000 - ranges.iter().map(|r| r.length()).sum::<u64>();
    (p1, p2)
}