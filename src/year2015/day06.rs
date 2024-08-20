use itertools::Itertools;
use crate::util::boxes::Box;

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
    rect_xs: Vec<i64>,
    rect_ys: Vec<i64>,
    x_index: [usize; 1001],
    y_index: [usize; 1001],
}


fn parse_instruction(line: &str) -> Option<Instruction> {
    let (cmd, s) =
        if let Some(s) = line.strip_prefix("toggle ") { 
            Some ((Command::Toggle, s))
        } else if let Some(s) = line.strip_prefix("turn on ") { 
            Some ((Command::On, s))
        } else if let Some(s) = line.strip_prefix("turn off ") { 
            Some ((Command::Off, s))
        } else {
            None
        }?;
    let (w1, _, w2) = s.split(' ').next_tuple()?;
    let (xmin, ymin) = w1.split_once(',')?;
    let (xmax, ymax) = w2.split_once(',')?;
    let xmin = xmin.parse().ok()?;
    let ymin = ymin.parse().ok()?;
    let xmax = xmax.parse().ok()?;
    let ymax = ymax.parse().ok()?;
    let rectangle = Box { xmin, ymin, xmax, ymax };
    Some(Instruction { cmd, rectangle })
}

fn do_cmd1(cmd: Command, v: &mut bool) {
    match cmd {
        Command::On     => *v = true,
        Command::Off    => *v = false,
        Command::Toggle => *v = !*v,
    }
}

fn do_cmd2(cmd: Command, v: &mut i64) {
    match cmd {
        Command::On     => *v += 1,
        Command::Off    => *v = (*v-1).max(0),
        Command::Toggle => *v += 2,
    }
}


pub fn parse(input: &str) -> Input
{
    let instrs: Vec<_> = input.lines().filter_map(parse_instruction).collect();
    
    let mut rect_xs: Vec<i64> = Vec::with_capacity(2*instrs.len());
    let mut rect_ys: Vec<i64> = Vec::with_capacity(2*instrs.len());
    
    for instr in &instrs {
        rect_xs.push(instr.rectangle.xmin);
        rect_xs.push(instr.rectangle.xmax+1);
        rect_ys.push(instr.rectangle.ymin);
        rect_ys.push(instr.rectangle.ymax+1);
    }
    rect_xs.sort();
    rect_xs.dedup();
    rect_ys.sort();
    rect_ys.dedup();

    let mut x_index = [0; 1001];
    for (i, &x) in rect_xs.iter().enumerate() {
        x_index[x as usize] = i;
    }

    let mut y_index = [0; 1001];
    for (i, &y) in rect_ys.iter().enumerate() {
        y_index[y as usize] = i;
    }

    Input {instrs, rect_xs, rect_ys, x_index, y_index }
}

pub fn part1(input: &Input) -> Option<i64> {
    let width = input.rect_xs.len();
    let size = width * input.rect_ys.len();

    let mut grid = Vec::with_capacity(size);
    for _ in 0..size {
        grid.push(false);
    }

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
    Some(total)
}

pub fn part2(input: &Input) -> Option<i64> {
    let width = input.rect_xs.len();
    let size = width * input.rect_ys.len();

    let mut grid = Vec::with_capacity(size);
    for _ in 0..size {
        grid.push(0);
    }

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
    Some(total)
}