use aoc::aoc_with_parser;
use nom::{
    bytes::complete::{take_till, tag},
    character::complete::{line_ending,space1,i64},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, preceded, tuple},
    IResult,
};
use itertools::Itertools;
use aoc::range::Range;

#[derive(Debug)]
struct ARange {
    dest: i64,
    source: i64,
    length: i64,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Vec<ARange>>
}

fn input_parser(input: &str) -> IResult<&str, Almanac> {
    let seeds = preceded(tag("seeds: "), separated_list1(space1, i64));
    let range = map(
        tuple((i64, space1, i64, space1, i64)),
        |(dest, _, source, _, length)| ARange { dest, source, length});
    let skip_line = pair(take_till(|c| c=='\n'), line_ending);
    let amap = preceded(skip_line, separated_list1(line_ending, range));

    map(
        tuple((seeds, line_ending, line_ending, separated_list1(pair(line_ending, line_ending), amap))),
        |(seeds, _, _, maps)| Almanac { seeds, maps }
    )(input)
}

fn next_range(arange: &ARange, range: &Range) -> Option<Range> {
    let inter = (range & &Range::new(arange.source, arange.source + arange.length - 1))?;
    Some(inter.translate(arange.dest - arange.source))
}

fn next_ranges(ranges: &Vec<Range>, map: &Vec<ARange>) -> Vec<Range> {
    let mut next = vec!();
    for arange in map {
        for range in ranges {
            if let Some(r) = next_range(arange, range) {
                next.push(r);
            }
        }
    };
    next
}

fn solve(seeds: &Vec<Range>, maps: &Vec<Vec<ARange>>) -> Option<i64> {
    maps.iter().fold(seeds.clone(), |acc, map| next_ranges(&acc, map)).iter().map(|r| r.lower).min()
}

fn part1(almanac: &Almanac) -> Option<i64> {
    let seeds: Vec<_> = almanac.seeds.iter().map(|&s| Range::new(s, s)).collect();
    solve(&seeds, &almanac.maps)
}

fn part2(almanac: &Almanac) -> Option<i64> {
    let seeds: Vec<_> = almanac.seeds.iter().tuples().map(|(&start, &len)| Range::new(start, start+len-1)).collect();
    solve(&seeds, &almanac.maps)
}

fn main() {
    let input = include_str!("../../inputs/2023/05");
    aoc_with_parser(input, input_parser, |almanac| (part1(&almanac), part2(&almanac)))
}