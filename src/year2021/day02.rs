use anyhow::*;
use itertools::Itertools;
use crate::util::parser::*;

enum Command {
    Forward(i32),
    Down(i32),
}

fn parse_command(cmd: &str, amount: &str) -> Result<Command> {
    let amount: i32 = amount.try_unsigned()?;
    match cmd {
        "forward" => Ok(Command::Forward(amount)),
        "down" => Ok(Command::Down(amount)),
        "up" => Ok(Command::Down(-amount)),
        _ => bail!("Invalid command {cmd}")
    }
}

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let commands: Vec<Command> = input
        .split_ascii_whitespace()
        .tuples()
        .map(|(cmd, amount)| parse_command(cmd, amount))
        .try_collect()?;

    let p1 = part1(&commands);
    let p2 = part2(&commands);

    Ok((p1, p2))
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