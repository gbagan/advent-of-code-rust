use crate::util::{iter::*, parallel::*, parser::*};

#[derive(Clone, Copy)]
enum Command {
    On, Off, Toggle
}

struct Instruction {
    cmd: Command,
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
}

fn parse_instruction(line: &str) -> Instruction {
    let (cmd, s) =
        if let Some(s) = line.strip_prefix("toggle ") { 
            (Command::Toggle, s)
        } else if let Some(s) = line.strip_prefix("turn on ") { 
            (Command::On, s)
        } else if let Some(s) = line.strip_prefix("turn off ") {
            (Command::Off, s)
        } else {
            panic!();
        };
    let (xmin, ymin, xmax, ymax) = s.iter_unsigned().next_tuple().unwrap();
    Instruction { cmd, xmin, xmax, ymin, ymax }
}

pub fn solve(input: &str) -> (usize, u32) {
    let instrs: Vec<_> = input.lines().map(parse_instruction).collect();
    
    (0..1000)
        .par_iter()
        .map(|i| {
            let mut p1 = [false; 1000];
            let mut p2 = [0u16; 1000];
            
            for instr in &instrs {
                if (instr.ymin..=instr.ymax).contains(&i) {
                    match instr.cmd {
                        Command::On => {
                            p1[instr.xmin..=instr.xmax].fill(true);
                            p2[instr.xmin..=instr.xmax].iter_mut().for_each(|x| *x += 1);
                        }
                        Command::Off => {
                            p1[instr.xmin..=instr.xmax].fill(false);
                            p2[instr.xmin..=instr.xmax].iter_mut().for_each(|x| *x = x.saturating_sub(1));
                        }
                        Command::Toggle => {
                            p1[instr.xmin..=instr.xmax].iter_mut().for_each(|x| *x = !*x);
                            p2[instr.xmin..=instr.xmax].iter_mut().for_each(|x| *x += 2);
                        }

                    }
                }
            }
            let p1 = p1.into_iter().filter(|&x| x).count();
            let p2 = p2.into_iter().map(|x| x as u32).sum();
            (p1, p2)
        }).reduce2(|| (0, 0), |(a1, b1), (a2, b2)| (a1+a2, b1+b2))
}