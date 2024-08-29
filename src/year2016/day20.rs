use crate::util::range::Range;

fn parse_range(line: &str) -> Option<Range> {
    let (lower, upper) = line.split_once('-')?;
    let lower = lower.parse().ok()?;
    let upper = upper.parse().ok()?;
    Some(Range::new(lower, upper))
}

pub fn solve(input: &str) -> Option<(i64, i64)> {
    let ranges: Vec<_> = input.lines().filter_map(parse_range).collect();
    let ranges = Range::disjoint_union(&ranges);
    let p1 = ranges[0].upper + 1;
    let p2 = (1 << 32) - ranges.iter().map(|r| r.length()).sum::<i64>();
    Some((p1, p2))

}