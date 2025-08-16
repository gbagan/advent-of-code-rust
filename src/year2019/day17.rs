use crate::{util::coord::*, year2019::intcode::*};

type Point = Coord<i32>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {Left, Right}
use Direction::*;
use Status::*;

struct Program<'a> {
    routine: Vec<usize>,
    functions: [&'a [(Direction, usize)]; 3],
}


pub fn solve(input: &str) -> (usize, i64) {
    let mut machine = IntCode::with_extra_capacity(input, 2000);
    machine.set(0, 2);
    let mut grid = Vec::new();
    machine.output_as_bytes(&mut grid);

    let width = grid.iter().position(|&c| c == b'\n').unwrap() + 1;
    let height = grid.len() / width;

    let p1 = part1(&grid, width, height);

    let path = path_instructions(&grid, width, height);
    let mut program = Program {routine: Vec::new(), functions: [&path; 3] };
    compress(&path, &mut program, 0);

    input_program(&mut machine, &program);
    
    let mut p2 = 0;
    while let Output(v) = machine.run() {
        p2 = v;
    }

    (p1, p2)
}

fn part1(grid: &[u8], width: usize, height: usize) -> usize {
    let mut intersections = 0;
    for y in 1..height-1 {
        for x in 1..width-2 {
            let index = y * width + x;
            if grid[index] == b'#' && grid[index-1] == b'#' && grid[index+1] == b'#'
                && grid[index-width] == b'#' && grid[index+width] == b'#' {
                    intersections += x * y;
                }

        }
    }

    intersections
}

fn path_instructions(input: &[u8], width: usize, height: usize) -> Vec<(Direction, usize)> {
    let mut grid = vec![b'.'; (width+1) * (height +2)];
    for y in 0..height {
        let index = (y+1) * (width + 1) + 1;
        let index2 = y * width;
        grid[index..index+width-1].copy_from_slice(&input[index2..index2+width-1]);
    }

    let width = 1 + width as i32;
    let height = 2 + height as i32;

    let mut position = Point::ORIGIN;
    let mut direction = Point::ORIGIN;

    let is_scaffold = |p: Point| grid[(p.y * width + p.x) as usize] == b'#';

    'outer: for y in 1..height-1 {
        for x in 1..width-1 {
            let index = (y * width + x) as usize;
            match grid[index] {
                b'^' => { position = Point::new(x, y); direction = Point::NORTH; break 'outer },
                b'v' => { position = Point::new(x, y); direction = Point::SOUTH; break 'outer },
                b'<' => { position = Point::new(x, y); direction = Point::WEST; break 'outer },
                b'>' => { position = Point::new(x, y); direction = Point::EAST; break 'outer },
                _ => {}
            }
        }
    }

    let mut path = Vec::new();
    let mut amount = 0;

    direction = direction.turn_left();
    let mut instr = Left;
    if !is_scaffold(position+direction) {
        direction -= direction;
        instr = Right;
    }

    loop {
        if is_scaffold(position+direction) {
            position += direction;
            amount += 1;
        } else {
            path.push((instr, amount));
            let next = direction.turn_left();
            if is_scaffold(position+next) {
                direction = next;
                position += direction;
                instr = Left;
                amount = 1;
            } else if is_scaffold(position-next) {
                direction = -next;
                position += direction;
                instr = Right;
                amount = 1;
            } else {
                break;
            }
        }
    }

    path
}

fn compress<'a>(path: &'a [(Direction, usize)], program: &mut Program<'a>, depth: usize) -> bool {
    if path.is_empty() {
        return true;
    }
    for i in 0..depth {
        program.routine.push(i);
        let function = program.functions[i];
        if let Some(path2) = path.strip_prefix(function) && compress(path2, program, depth) {
            return true;
        }
        program.routine.pop();
    }
    if depth < 3 {
        program.routine.push(depth);
        for i in (1..6.min(path.len())).rev() {
            let (prefix, path2) = path.split_at(i);
            program.functions[depth] = prefix;
            if compress(path2, program, depth+1) {
                return true;
            }
        }
        program.routine.pop();
    }

    false
}


fn input_program(machine: &mut IntCode, program: &Program) {
    for (i, &n) in program.routine.iter().enumerate() {
        if i > 0 {
            machine.input(b',' as i64);
        }
        machine.input((b'A' + n as u8) as i64);
    }
    machine.input(b'\n' as i64);
    for function in &program.functions {
        for (i, &(dir, mut dist)) in function.iter().enumerate() {
            if i > 0 {
                machine.input(b',' as i64);
            }
            machine.input( if dir == Left { b'L' as i64 } else { b'R' as i64 });
            machine.input(b',' as i64);
            if dist >= 10 {
                dist -= 10;
                machine.input(b'1' as i64);
            }
            machine.input((dist as u8 + b'0') as i64);
        }

        machine.input(b'\n' as i64);
    }
    machine.input(b'n' as i64);
    machine.input(b'\n' as i64);
}