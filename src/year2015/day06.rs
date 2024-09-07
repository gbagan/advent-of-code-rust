use anyhow::*;
use itertools::Itertools;
use crate::util::{boxes::Box, parser::UnsignedIter, TryParseLines};

#[derive(Clone, Copy)]
enum Command {
    On, Off, Toggle
}

struct Instruction {
    cmd: Command,
    rectangle: Box,
}

pub struct Input {
    instrs: Vec<Instruction>,
    rect_xs: Vec<i32>,
    rect_ys: Vec<i32>,
    x_index: [usize; 1001],
    y_index: [usize; 1001],
}


fn parse_instruction(line: &str) -> Result<Instruction> {
    let (cmd, s) =
        if let Some(s) = line.strip_prefix("toggle ") { 
            Some ((Command::Toggle, s))
        } else if let Some(s) = line.strip_prefix("turn on ") { 
            Some ((Command::On, s))
        } else {
            line.strip_prefix("turn off ").map(|s| (Command::Off, s))
        }.context("Line does not start with 'toggle', 'turn on' or 'turn off'")?;
    let (xmin, ymin, xmax, ymax) = s.iter_unsigned().collect_tuple()
                                    .context("Line must contains  4 integers")?;
    let rectangle = Box { xmin, ymin, xmax, ymax };
    Ok(Instruction { cmd, rectangle })
}

fn do_cmd1(cmd: Command, v: &mut bool) {
    match cmd {
        Command::On     => *v = true,
        Command::Off    => *v = false,
        Command::Toggle => *v = !*v,
    }
}

fn do_cmd2(cmd: Command, v: &mut i32) {
    match cmd {
        Command::On     => *v += 1,
        Command::Off    => *v = (*v-1).max(0),
        Command::Toggle => *v += 2,
    }
}

pub fn solve(input: &str) -> Result<(i32, i32)>
{
    let instrs: Vec<_> = input.try_parse_lines_and_collect(parse_instruction)?;
    
    let mut rect_xs: Vec<i32> = Vec::with_capacity(2*instrs.len());
    let mut rect_ys: Vec<i32> = Vec::with_capacity(2*instrs.len());
    
    for instr in &instrs {
        rect_xs.push(instr.rectangle.xmin);
        rect_xs.push(instr.rectangle.xmax+1);
        rect_ys.push(instr.rectangle.ymin);
        rect_ys.push(instr.rectangle.ymax+1);
    }
    rect_xs.sort_unstable();
    rect_xs.dedup();
    rect_ys.sort_unstable();
    rect_ys.dedup();

    let mut x_index = [0; 1001];
    for (i, &x) in rect_xs.iter().enumerate() {
        x_index[x as usize] = i;
    }

    let mut y_index = [0; 1001];
    for (i, &y) in rect_ys.iter().enumerate() {
        y_index[y as usize] = i;
    }

    let input = Input {instrs, rect_xs, rect_ys, x_index, y_index };

    let p1 = part1(&input);
    let p2 = part2(&input);
    Ok((p1, p2))

}

pub fn part1(input: &Input) -> i32 {
    let width = input.rect_xs.len();
    let size = width * input.rect_ys.len();

    let mut grid = vec![false; size];

    for instr in &input.instrs {
        let xmin = input.x_index[instr.rectangle.xmin as usize];
        let xmax = input.x_index[instr.rectangle.xmax as usize + 1];
        let ymin = input.y_index[instr.rectangle.ymin as usize];
        let ymax = input.y_index[instr.rectangle.ymax as usize + 1];
        for x in xmin..xmax {
            for y in ymin..ymax {
                do_cmd1(instr.cmd,&mut grid[y * width + x]);
            }
        }
    }

    let mut total = 0;
    for x in 0..input.rect_xs.len()-1 {
        for y in 0..input.rect_ys.len()-1 {
            if grid[y * width + x] {
                total += (input.rect_xs[x+1] - input.rect_xs[x]) * (input.rect_ys[y+1] - input.rect_ys[y])
            }
        }
    }
    total
}

pub fn part2(input: &Input) -> i32 {
    let width = input.rect_xs.len();
    let size = width * input.rect_ys.len();

    let mut grid = vec![0; size];

    for instr in &input.instrs {
        let xmin = input.x_index[instr.rectangle.xmin as usize];
        let xmax = input.x_index[instr.rectangle.xmax as usize + 1];
        let ymin = input.y_index[instr.rectangle.ymin as usize];
        let ymax = input.y_index[instr.rectangle.ymax as usize + 1];
        for x in xmin..xmax {
            for y in ymin..ymax {
                do_cmd2(instr.cmd,&mut grid[y * width + x]);
            }
        }
    }

    let mut total = 0;
    for x in 0..input.rect_xs.len()-1 {
        for y in 0..input.rect_ys.len()-1 {
            let s = grid[y * width + x];
            if s > 0 {
                total += s * (input.rect_xs[x+1] - input.rect_xs[x]) * (input.rect_ys[y+1] - input.rect_ys[y])
            }
        }
    }
    total
}