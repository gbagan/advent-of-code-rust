use std::path::{Path, PathBuf};
use std::env::args;
use std::fs::read_to_string;
use std::time::{Duration, Instant};
use aoc::*;

struct Solution {
    year: String,
    day: String,
    path: PathBuf,
    wrapper: fn(String) -> (Option<String>, Option<String>),
}

macro_rules! solution {
    ($year:tt, $day:tt) => {{
        let year = stringify!($year).trim_matches(char::is_alphabetic); 
        let day = stringify!($day).trim_matches(char::is_alphabetic);
        let path = Path::new("inputs").join(year).join(day);

        let wrapper = |input: String| {
            use $year::$day::*;

            let parsed = parse(&input);
            let part1 = part1(&parsed);
            let part2 = part2(&parsed);

            (part1.map(|p| p.to_string()), part2.map(|p| p.to_string()))
        };

        Solution { year: year.to_string(), day: day.to_string(), path, wrapper }
    }};
}

fn main() {
    let (arg_year, arg_day) = (args().nth(1), args().nth(2));

    let solutions = year2015();

    let mut solved = 0;
    let mut duration = Duration::ZERO;

    for Solution { year, day, path, wrapper } in &solutions {
        if arg_year.as_ref().map(|x| x != year).unwrap_or(false)
            || arg_day.as_ref().map(|x| x != day).unwrap_or(false) {
            break
        }
        
        if let Ok(data) = read_to_string(&path) {
            let instant = Instant::now();
            let (part1, part2) = wrapper(data);
            let elapsed = instant.elapsed();

            solved += 1;
            duration += elapsed;

            println!("{year} Day {day}");
            let msg1 = if let Some(msg) = part1 {msg} else {"no solution".to_string()};
            let msg2 = if let Some(msg) = part2 {msg} else {"no solution".to_string()};
            println!("    Part 1: {msg1}");
            println!("    Part 2: {msg2}");
            println!("    Elapsed: {} μs", elapsed.as_micros());
        } else {
            eprintln!("{year} Day {day:02}");
            eprintln!("    Missing input!");
        }
    }

    println!("Solved: {solved}");
    println!("Duration: {} ms", duration.as_millis());
}


fn year2015() -> Vec<Solution> {
    vec![
        solution!(year2015, day01),
        solution!(year2015, day02),
        solution!(year2015, day03),
        solution!(year2015, day04),
        solution!(year2015, day05),
        solution!(year2015, day06),
        solution!(year2015, day07),
        solution!(year2015, day08),
        solution!(year2015, day09),
        solution!(year2015, day10),
        solution!(year2015, day11),
        solution!(year2015, day12),
        solution!(year2015, day13),
    ]
}