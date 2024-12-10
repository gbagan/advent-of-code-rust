use anyhow::*;
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
    func: fn(&String) -> Result<(String, String)>,
}

macro_rules! solution {
    ($year:tt, $day:tt) => {{
        let year = stringify!($year).trim_matches(char::is_alphabetic).to_string(); 
        let day = stringify!($day).trim_matches(char::is_alphabetic).to_string();
        let func = |input: &String| {
            use $year::$day::*;

            let (p1, p2) = solve(input)?;
            Ok((p1.to_string(), p2.to_string()))
        };

        Solution { year, day, func }
    }};
}


fn main() {
    let (command, arg_year, arg_day) = (args().nth(1), args().nth(2), args().nth(3));
    match command.as_deref() {
        Some("solve") => solve(arg_year, arg_day, true),
        Some("time") => solve(arg_year, arg_day, false),
        Some("download") => download(&arg_year, &arg_day),
        _ => println!("Invalid command."),
    }
}

fn solve(arg_year: Option<String>, arg_day: Option<String>, display_solution: bool) {
    let solutions = solutions();

    let mut solved = 0;
    let mut duration = Duration::ZERO;

    for Solution { year, day, func } in &solutions {
        if arg_year.as_ref().map(|x| x != year).unwrap_or(false)
            || arg_day.as_ref().map(|x| x != day).unwrap_or(false) {
            continue
        }
        
        let path = Path::new("inputs").join(year).join(day);
        if let Result::Ok(data) = read_to_string(&path) {
            let instant = Instant::now();
            let res = func(&data);
            let mut elapsed = instant.elapsed();
            let microseconds = elapsed.as_micros();

            let mut iterations = 0;

            if !display_solution {
                elapsed = Duration::ZERO;
                iterations = if microseconds < 5000 {100} else {10};
                for _ in 0..iterations {
                    let data = data.clone();
                    let instant = Instant::now();
                    let _ = func(&data);
                    elapsed += instant.elapsed();
                }
                elapsed /= iterations;
            }

            solved += 1;
            duration += elapsed;
            
            let text = format!("{microseconds} μs");
            let text =
                if microseconds < 1000 {
                    Green.paint(text)
                } else if microseconds < 100_000 {
                    Yellow.bold().paint(text)
                } else {
                Red.bold().paint(text)
                };
            if display_solution {
                println!("{year} Day {day} in {text}.");
            } else {
                println!("{year} Day {day} in {text} on average over {iterations} iterations.");
            }
            match res  {
                Err(err) => println!("{err:?}"),
                Result::Ok((part1, part2)) => { 
                    if display_solution {
                        println!("    Part 1: {part1}");
                        println!("    Part 2: {part2}");
                    }
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
            std::result::Result::Ok(output) => {
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

        solution!(year2016, day01),
        solution!(year2016, day02),
        solution!(year2016, day03),
        solution!(year2016, day04),
        solution!(year2016, day05),
        solution!(year2016, day06),
        solution!(year2016, day07),
        solution!(year2016, day08),
        solution!(year2016, day09),
        solution!(year2016, day10),
        solution!(year2016, day11),
        solution!(year2016, day12),
        solution!(year2016, day13),
        // todo
        solution!(year2016, day15),
        solution!(year2016, day16),
        solution!(year2016, day20),

        solution!(year2017, day01),
        solution!(year2017, day02),
        solution!(year2017, day03),
        solution!(year2017, day04),
        solution!(year2017, day05),
        solution!(year2017, day06),
        solution!(year2017, day07),
        solution!(year2017, day08),
        solution!(year2017, day09),
        solution!(year2017, day10),
        solution!(year2017, day11),
        solution!(year2017, day12),
        solution!(year2017, day13),
        solution!(year2017, day14),
        solution!(year2017, day15),
        solution!(year2017, day16),
        solution!(year2017, day17),
        // todo
        solution!(year2017, day19),
        solution!(year2017, day20),
        solution!(year2017, day21),
        solution!(year2017, day22),
        solution!(year2017, day23),
        solution!(year2017, day24),
        solution!(year2017, day25),

        solution!(year2020, day01),
        solution!(year2020, day02),
        solution!(year2020, day03),
        solution!(year2020, day04),
        solution!(year2020, day05),
        solution!(year2020, day06),
        solution!(year2020, day07),
        solution!(year2020, day08),
        solution!(year2020, day09),
        solution!(year2020, day10),
        solution!(year2020, day11),
        solution!(year2020, day18),

        solution!(year2021, day01),
        solution!(year2021, day02),

        solution!(year2022, day01),
        solution!(year2022, day02),
        solution!(year2022, day03),
        solution!(year2022, day04),
        solution!(year2022, day05),
        solution!(year2022, day06),
        solution!(year2022, day07),
        solution!(year2022, day08),
        solution!(year2022, day09),
        solution!(year2022, day10),
        solution!(year2022, day11),
        solution!(year2022, day12),
        solution!(year2022, day13),
        solution!(year2022, day14),
        solution!(year2022, day15),

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

        solution!(year2024, day01),
        solution!(year2024, day02),
        solution!(year2024, day03),
        solution!(year2024, day04),
        solution!(year2024, day05),
        solution!(year2024, day06),
        solution!(year2024, day07),
        solution!(year2024, day08),
        solution!(year2024, day09),
        solution!(year2024, day10),
        solution!(year2024, day11),
    ]
}