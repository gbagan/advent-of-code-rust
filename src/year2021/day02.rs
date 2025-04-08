use itertools::Itertools;
use crate::util::parser::*;

enum Command {
    Forward(i32),
    Down(i32),
}

fn parse_command(cmd: &str, amount: &str) -> Command {
    let amount: i32 = amount.try_unsigned().unwrap();
    match cmd {
        "forward" => Command::Forward(amount),
        "down" => Command::Down(amount),
        "up" => Command::Down(-amount),
        _ => panic!("Invalid command {cmd}")
    }
}

pub fn solve(input: &str) -> (i32, i32) {
    let commands: Vec<_> = input
        .split_ascii_whitespace()
        .tuples()
        .map(|(cmd, amount)| parse_command(cmd, amount))
        .collect();

    let p1 = part1(&commands);
    let p2 = part2(&commands);

    (p1, p2)
}


fn part1(commands: &[Command]) -> i32 {
    let mut position = 0;
    let mut depth = 0;
    for command in commands {
        match command {
            Command::Forward(n) => position += n,
            Command::Down(n) => depth += n
        }
    }

    position * depth
}

fn part2(commands: &[Command]) -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        match command {
            Command::Forward(n) => { position += n; depth += aim * n },
            Command::Down(n) => aim += n
        }
    }

    position * depth
}