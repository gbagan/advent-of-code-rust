use std::path::Path;
use std::env::args;
use std::fs::{create_dir, read_to_string};
use std::process::{Command, Stdio};
use std::io;
use std::io::Write;
use std::time::{Duration, Instant};
use ansi_term::Color::{Red, Yellow, Green};
use aoc::*;

struct Solution {
    year: String,
    day: String,
    wrapper: fn(String) -> Result<(String, String), String>,
}

macro_rules! solution {
    ($year:tt, $day:tt) => {{
        let year = stringify!($year).trim_matches(char::is_alphabetic).to_string(); 
        let day = stringify!($day).trim_matches(char::is_alphabetic).to_string();
        let wrapper = |input: String| {
            use $year::$day::*;

            let parsed = parse(&input).ok_or("Parsing has failed")?;
            let p1 = part1(&parsed).ok_or("Part 1 has failed")?;
            let p2 = part2(&parsed).ok_or("Part 2 has failed")?;

            Ok((p1.to_string(), p2.to_string()))
        };

        Solution { year, day, wrapper }
    }};
}

fn main() {
    let (command, arg_year, arg_day) = (args().nth(1), args().nth(2), args().nth(3));
    match command.as_deref() {
        Some("solve") => solve(arg_year, arg_day),
        Some("download") => download(&arg_year, &arg_day),
        _ => println!("Invalid command."),
    }
}

fn solve(arg_year: Option<String>, arg_day: Option<String>) {
    let solutions = solutions();

    let mut solved = 0;
    let mut duration = Duration::ZERO;

    for Solution { year, day, wrapper } in &solutions {
        if arg_year.as_ref().map(|x| x != year).unwrap_or(false)
            || arg_day.as_ref().map(|x| x != day).unwrap_or(false) {
            continue
        }
        
        let path = Path::new("inputs").join(year).join(day);
        if let Ok(data) = read_to_string(&path) {
            let instant = Instant::now();
            let res = wrapper(data);
            let elapsed = instant.elapsed();

            solved += 1;
            duration += elapsed;
            println!("{year} Day {day}");
            match res  {
                Err(str) => println!("  {}", str),
                Ok((part1, part2)) => {
                    println!("    Part 1: {part1}");
                    println!("    Part 2: {part2}");
                    let microseconds = elapsed.as_micros();
                    let text = format!("    Elapsed: {} μs", microseconds);
                    let text =
                        if microseconds < 1000 {
                            Green.paint(text)
                        } else if microseconds < 100_000 {
                            Yellow.bold().paint(text)
                        } else {
                        Red.bold().paint(text)
                        };

                    println!("{text}");
                }
            }
        } else {
            eprintln!("{year} Day {day:02}");
            eprintln!("    Missing input!");
        }
    }

    println!("Solved: {solved}");
    println!("Duration: {} ms", duration.as_millis());
}

fn download(arg_year: &Option<String>, arg_day: &Option<String>) {
    let years_and_days = match (arg_year, arg_day) {
        (Some(year), Some(day)) => vec!((year.to_string(), day.to_string())),
        _ => solutions().iter().map(|sol| (sol.year.clone(), sol.day.clone())).collect()
    };
    for (year, day) in years_and_days {
        let dir_path = Path::new("inputs").join(&year);
        let input_path = format!("inputs/{year}/{day}");
        let _ = create_dir(dir_path);
        let args = vec!(
                "-I".into(),
                "--overwrite".into(),
                "--input-file".into(),
                input_path,
                "--year".into(),
                year.to_string(),
                "--day".into(),
                day.to_string(),
                "download".into(),
        );
        match Command::new("aoc")
            .args(args)
            .stdout(Stdio::null())
            //.stderr(Stdio::null())
            .output()
        {
            Err(_) => {
                println!("aoc is not callable. Please install aoc-cli with \"cargo install aoc-cli\"");
                return
            }
            Ok(output) => {
                if output.status.success() {
                    println!("Successfully wrote input of year {year} day {day}");
                } else {
                    io::stderr().write_all(&output.stderr).unwrap();
                }
            }
        }
    }

}

fn solutions() -> Vec<Solution> {
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
        solution!(year2015, day14),
        solution!(year2015, day15),
        solution!(year2015, day16),
        solution!(year2015, day17),
        solution!(year2015, day18),
        solution!(year2015, day19),
        solution!(year2015, day20),
        solution!(year2015, day21),
        solution!(year2015, day22),
        solution!(year2015, day23),
        solution!(year2015, day24),
        solution!(year2015, day25),

        solution!(year2017, day01),
        solution!(year2017, day02),
        solution!(year2017, day03),
        solution!(year2017, day04),
        solution!(year2017, day05),

        solution!(year2023, day01),
        solution!(year2023, day02),
        solution!(year2023, day03),
        solution!(year2023, day04),
        solution!(year2023, day05),
        solution!(year2023, day06),
        solution!(year2023, day07),
        solution!(year2023, day08),
        solution!(year2023, day09),
        solution!(year2023, day10),
        solution!(year2023, day11),
        solution!(year2023, day12),
        solution!(year2023, day13),
        solution!(year2023, day14),
        solution!(year2023, day15),
        solution!(year2023, day16),
        solution!(year2023, day17),
        solution!(year2023, day18),
        solution!(year2023, day19),
        solution!(year2023, day20),
        solution!(year2023, day21),
        solution!(year2023, day22),
        solution!(year2023, day23),
        solution!(year2023, day24),
        solution!(year2023, day25),
    ]
}