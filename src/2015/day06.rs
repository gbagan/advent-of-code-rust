use aoc::aoc_with_parser;
use std::cmp::max;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char,space1,u64},
    combinator::map,
    multi::separated_list1,
    IResult,
};
use aoc::iter::AOCIter;

enum Command {
    On, Off, Toggle
}
struct Instruction {
    cmd: Command,
    x1: u64,
    y1: u64,
    x2: u64,
    y2: u64,
}

fn input_parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    fn command(input: &str) -> IResult<&str,  Command> {
        alt((map(tag("turn on"), |_| Command::On), 
             map(tag("turn off"), |_| Command::Off),
             map(tag("toggle"), |_| Command::Toggle)
            ))(input)
    }

    fn instr(input: &str) -> IResult<&str, Instruction> {
        let (input, cmd) = command(input)?;
        let (input, _) = space1(input)?;
        let (input, x1) = u64(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y1) = u64(input)?;
        let (input, _) = tag(" through ")(input)?;
        let (input, x2) = u64(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y2) = u64(input)?;
        Ok((input, Instruction { cmd, x1, y1, x2, y2 }))
    }
            
    separated_list1(char('\n'), instr)(input)
}

#[inline]
fn do_cmd <A,F>(a: &mut[A], x1: u64, y1: u64, x2: u64, y2: u64, f: F)
    where F: Fn(&mut A),
{
    for i in (x1*1000 ..= x2*1000).step_by(1000) {
        for j in i+y1 ..= i+y2 {
            f(&mut a[j as usize])
        }
    }
}

fn part1 (instrs: &Vec<Instruction>) -> usize {
    let mut a = [false; 1_000_000];
    for Instruction {cmd, x1, y1, x2, y2} in instrs {
        match cmd {
            Command::On     => do_cmd (&mut a, *x1, *y1, *x2, *y2, |x| *x = true),
            Command::Off    => do_cmd (&mut a, *x1, *y1, *x2, *y2, |x| *x = false),
            Command::Toggle => do_cmd (&mut a, *x1, *y1, *x2, *y2, |x| *x = !*x),
        }
    }
    a.iter().count_by(|x| *x)
}

fn part2 (instrs: &Vec<Instruction>) -> u64 {
    let mut a = [0; 1_000_000];
    for Instruction {cmd, x1, y1, x2, y2} in instrs {
        match cmd {
            Command::On  => do_cmd (&mut a, *x1, *y1, *x2, *y2, |x| *x+=1),
            Command::Off  => do_cmd (&mut a, *x1, *y1, *x2, *y2, |x| *x = max(*x,1)-1),
            Command::Toggle  => do_cmd (&mut a, *x1, *y1, *x2, *y2, |x| *x+=2),
        }
    }
    a.iter().sum()
}

fn main() {
    let input = include_str!("../../inputs/2015/06");
    aoc_with_parser(input, input_parser, |instrs| {
        (part1(&instrs), part2(&instrs))
    })
}