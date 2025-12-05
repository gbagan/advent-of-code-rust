use crate::util::{iter::*, parser::*, range::*};

pub fn solve(input: &str) -> (usize, u64) {
    let mut lines = input.lines();
    
    let ranges = lines.by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (a, b) = line.iter_unsigned::<u64>().next_tuple().unwrap();
            Range::new(a, b)
        })
        .to_disjoint_union();

    let p1 = lines
        .remainder()
        .unwrap()
        .iter_unsigned::<u64>()
        .filter(|&id| is_fresh_ingredient(&ranges, id))
        .count();

    let p2 = ranges.iter().map(|r| r.length()).sum();

    (p1, p2)
}

fn is_fresh_ingredient(mut ranges: &[Range<u64>], ingredient: u64) -> bool {
    while !ranges.is_empty() {
        let mid = ranges.len() / 2;
        let mid_range = ranges[mid];
        if ingredient < mid_range.start {
            ranges = &ranges[..mid];
        } else if ingredient > mid_range.end {
            ranges = &ranges[mid+1..];
        } else {
            return true;
        }
    }

    false
}