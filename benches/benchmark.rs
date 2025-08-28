#![allow(unstable_features)]
#![feature(test)]
extern crate test;

macro_rules! benchmark {
    ($year:tt $($day:tt),*) => {
        mod $year {$(
            mod $day {
                use aoc::$year::$day::*;
                use std::fs::read_to_string;
                use std::path::Path;
                use test::Bencher;

                #[bench]
                fn parse_bench(b: &mut Bencher) {
                    let year = stringify!($year).strip_prefix("year").unwrap().to_string(); 
                    let day = stringify!($day).strip_prefix("day").unwrap().to_string();
                    let path = Path::new("inputs").join(year).join(day);
                    let data = read_to_string(path).unwrap();
                    b.iter(|| solve(&data));
                }
            }
        )*}
    }
}

benchmark!(year2015
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

benchmark!(year2016
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day15, day16, day18, day19, day20, day21, day22, day23, day24, day25
);

benchmark!(year2020
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12,
    day13, day14, day15, day16, day17, day23, day25
);

benchmark!(year2022
    day20, day21, day22
);

benchmark!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);