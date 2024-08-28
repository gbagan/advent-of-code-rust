use crate::util::range::Range;

fn parse_range(line: &str) -> Option<Range> {
    let (lower, upper) = line.split_once('-')?;
    let lower = lower.parse().ok()?;
    let upper = upper.parse().ok()?;
    Some(Range::new(lower, upper))
}

pub fn parse(input: &str) -> Option<Vec<Range>> {
    let ranges: Vec<_> = input.lines().filter_map(parse_range).collect();
    Some(Range::disjoint_union(&ranges))
}

pub fn part1(ranges: &[Range]) -> Option<i64> {
    Some(ranges[0].upper + 1)
}

pub fn part2(ranges: &[Range]) -> Option<i64> {
    Some((1 << 32) - ranges.iter().map(|r| r.length()).sum::<i64>())
}