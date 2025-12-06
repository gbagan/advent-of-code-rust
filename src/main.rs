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
    func: fn(&String) -> (String, String, Duration),
}

macro_rules! solution {
    ($year:tt, $day:tt) => {{
        let year = stringify!($year).strip_prefix("year").unwrap().to_string(); 
        let day = stringify!($day).strip_prefix("day").unwrap().to_string();
        let func = |input: &String| {
            let instant = Instant::now();
            let (p1, p2) = $year::$day::solve(input);
            let elapsed = instant.elapsed();
            (p1.to_string(), p2.to_string(), elapsed)
        };

        Solution { year, day, func }
    }};
}


fn main() {
    let (command, arg_year, arg_day) = (args().nth(1), args().nth(2), args().nth(3));
    match command.as_deref() {
        Some("solve") => solve(arg_year, arg_day, Cmd::Solve),
        Some("time") => solve(arg_year, arg_day, Cmd::Time),
        Some("slowest") => solve(arg_year, arg_day, Cmd::Slowest),
        Some("markdown") => markdown(arg_year.unwrap()),
        Some("download") => download(&arg_year, &arg_day),
        _ => println!("Invalid command."),
    }
}

#[derive(PartialEq, Eq)]
enum Cmd { Time, Solve, Slowest }


fn solve(arg_year: Option<String>, arg_day: Option<String>, cmd: Cmd) {
    let solutions = solutions();

    let mut durations = Vec::new();

    let mut solved = 0;
    let mut duration = Duration::ZERO;

    for Solution { year, day, func } in &solutions {
        if arg_year.as_ref().map(|x| x != year).unwrap_or(false)
            || arg_day.as_ref().map(|x| x != day).unwrap_or(false) {
            continue
        }
        
        let path = Path::new("inputs").join(year).join(day);
        if let Ok(data) = read_to_string(&path) {
            let (p1, p2, mut elapsed) = func(&data);
            let microseconds = elapsed.as_micros();

            solved += 1;
                    

            if cmd == Cmd::Solve {
                println!("{year} Day {day}.");
            } else {
                let mut elapsed_vec = Vec::new();
                let iterations = if microseconds < 5000 {100} else {20};
                for _ in 0..iterations {
                    let data = data.clone();
                    elapsed_vec.push(func(&data).2);
                }
                elapsed = *elapsed_vec.select_nth_unstable(iterations/2-1).1;
                duration += elapsed;
                
                if cmd == Cmd::Time {
                    let microseconds = elapsed.as_micros();
                    let nanoseconds = elapsed.as_nanos();

                    let text = if microseconds <= 5 {
                        format!("{nanoseconds} ns")
                    } else {
                        format!("{microseconds} μs")
                    };
                    let text =
                        if microseconds < 1000 {
                            Green.paint(text)
                        } else if microseconds < 100_000 {
                            Yellow.bold().paint(text)
                        } else {
                            Red.bold().paint(text)
                        };

                    println!("{year} Day {day} in {text}, median over {iterations} iterations.");
                } else if cmd == Cmd::Slowest {
                    durations.push((elapsed, year, day));
                }
            }
            if cmd == Cmd::Solve {
                println!("    Part 1: {p1}");
                println!("    Part 2: {p2}");
            }
        } else {
            eprintln!("{year} Day {day:02}");
            eprintln!("    Missing input!");
        }
    }

    println!("Solved: {solved}");
    match cmd {
        Cmd::Solve => {},
        Cmd::Time => println!("Duration: {} μs", duration.as_micros()),
        Cmd::Slowest => {
            durations.sort_unstable_by_key(|p| p.0);
            for (i, &(elapsed, year, day)) in durations.iter().rev().take(10).enumerate() {
                println!("{}: {}ms year {} day {} {}%", i+1, elapsed.as_millis(), year, day,
                    elapsed.as_micros() * 100 / duration.as_micros()
                );
            }
        }
    
    }
}

fn markdown(arg_year: String) {
    let solutions = solutions();

    let mut duration = Duration::ZERO;

    println!("| Day   | Source | Benchmark | Thread | SIMD | unsafe |");
    println!("|:--------:|:--------:|:------:|:--------:|:-----:|:------:|");

    for Solution { year, day, func } in &solutions {
        if arg_year.as_str() != year {
            continue
        }
        
        let path = Path::new("inputs").join(year).join(day);
        if let Result::Ok(data) = read_to_string(&path) {
            let (_, _, elapsed) = func(&data);
            let microseconds = elapsed.as_micros();
            let mut elapsed_vec = Vec::new();
            let iterations = if microseconds < 5000 {1000} else {20};
            for _ in 0..iterations {
                let data = data.clone();
                elapsed_vec.push(func(&data).2);
            }
            let elapsed = *elapsed_vec.select_nth_unstable(iterations/2-1).1;

            duration += elapsed;
            let microseconds = elapsed.as_micros();
            let nanoseconds = elapsed.as_nanos();

            let text = if microseconds <= 5 {
                format!("{nanoseconds} ns")
            } else {
                format!("{microseconds} μs")
            };
            let tmp = day.as_bytes();
            let day2 = if tmp[0] == b'0' { (tmp[1] as char).to_string() } else { day.clone() };

            println!("| [{day2}](https://adventofcode.com/{year}/day/{day2}) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year{year}/day{day}.rs) | {text} | | | |");
        } else {
            eprintln!("{year} Day {day:02}");
            eprintln!("    Missing input!");
        }
    }
    println!(" | Total |     | {} μs | | | |", duration.as_micros());
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
        // todo
        solution!(year2016, day18),
        solution!(year2016, day19),
        solution!(year2016, day20),
        solution!(year2016, day21),
        solution!(year2016, day22),
        solution!(year2016, day23),
        solution!(year2016, day24),
        solution!(year2016, day25),

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
        solution!(year2017, day18),
        solution!(year2017, day19),
        solution!(year2017, day20),
        solution!(year2017, day21),
        solution!(year2017, day22),
        solution!(year2017, day23),
        solution!(year2017, day24),
        solution!(year2017, day25),

        solution!(year2018, day01),
        solution!(year2018, day02),
        solution!(year2018, day03),
        solution!(year2018, day04),
        solution!(year2018, day05),
        solution!(year2018, day06),
        solution!(year2018, day07),
        solution!(year2018, day08),
        solution!(year2018, day09),
        solution!(year2018, day10),
        solution!(year2018, day11),
        solution!(year2018, day12),
        solution!(year2018, day13),
        // todo
        solution!(year2018, day15),
        solution!(year2018, day16),
        solution!(year2018, day17),
        // todo
        solution!(year2018, day19),
        // todo
        solution!(year2018, day23),
        // todo
        solution!(year2018, day25),

        solution!(year2019, day01),
        solution!(year2019, day02),
        solution!(year2019, day03),
        solution!(year2019, day04),
        solution!(year2019, day05),
        solution!(year2019, day06),
        solution!(year2019, day07),
        solution!(year2019, day08),
        solution!(year2019, day09),
        solution!(year2019, day10),
        solution!(year2019, day11),
        solution!(year2019, day12),
        solution!(year2019, day13),
        solution!(year2019, day14),
        solution!(year2019, day15),
        solution!(year2019, day16),
        solution!(year2019, day17),
        solution!(year2019, day18),
        solution!(year2019, day19),
        // todo
        solution!(year2019, day22),
        // todo
        solution!(year2019, day24),
        // todo

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
        solution!(year2020, day12),
        solution!(year2020, day13),
        solution!(year2020, day14),
        solution!(year2020, day15),
        solution!(year2020, day16),
        solution!(year2020, day17),
        solution!(year2020, day18),
        //todo
        solution!(year2020, day23),
        // todo
        solution!(year2020, day25),

        solution!(year2021, day01),
        solution!(year2021, day02),
        solution!(year2021, day03),
        solution!(year2021, day04),
        solution!(year2021, day05),
        solution!(year2021, day06),
        solution!(year2021, day07),
        solution!(year2021, day08),
        solution!(year2021, day09),
        solution!(year2021, day10),
        solution!(year2021, day11),
        solution!(year2021, day12),
        solution!(year2021, day13),
        solution!(year2021, day14),
        solution!(year2021, day15),
        solution!(year2021, day16),
        solution!(year2021, day17),
        solution!(year2021, day18),
        solution!(year2021, day19),
        solution!(year2021, day20),
        solution!(year2021, day21),
        solution!(year2021, day22),
        solution!(year2021, day23),
        solution!(year2021, day24),
        solution!(year2021, day25),

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
        solution!(year2022, day16),
        solution!(year2022, day17),
        solution!(year2022, day18),
        solution!(year2022, day19),
        solution!(year2022, day20),
        solution!(year2022, day21),
        solution!(year2022, day22),
        solution!(year2022, day23),
        solution!(year2022, day24),
        solution!(year2022, day25),

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
        solution!(year2024, day12),
        solution!(year2024, day13),
        solution!(year2024, day14),
        solution!(year2024, day15),
        solution!(year2024, day16),
        solution!(year2024, day17),
        solution!(year2024, day18),
        solution!(year2024, day19),
        solution!(year2024, day20),
        solution!(year2024, day21),
        solution!(year2024, day22),
        solution!(year2024, day23),
        solution!(year2024, day24),
        solution!(year2024, day25),

        solution!(year2025, day01),
        solution!(year2025, day02),
        solution!(year2025, day03),
        solution!(year2025, day04),
        solution!(year2025, day05),
        solution!(year2025, day06),
    ]
}