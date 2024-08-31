use itertools::Itertools;
use crate::util::{parser::*, range::Range};

pub fn solve(input: &str) -> Option<(u32, u32)> {
    let ranges: Vec<_> = input
                            .iter_unsigned::<u32>()
                            .tuples()
                            .map(|(x, y)| Range::new(x, y))
                            .collect();
    let ranges = Range::disjoint_union(&ranges);
    let p1 = ranges[0].upper as u32 + 1;
    let p2 = 0u32.wrapping_sub(ranges.iter().map(|r| r.length()).sum::<u32>());
    Some((p1, p2))

}