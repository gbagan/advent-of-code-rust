use crate::util::parser::*;

pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

pub fn solve(input: &str) -> (i32, i32) {
    let instructions: Vec<_> =
        input
        .as_bytes()
        .split(|c| !c.is_ascii_lowercase())
        .filter(|c| !c.is_empty())
        .zip(input.iter_signed::<i32>())
        .map(|(a, b)| to_instr(a, b))
        .collect();
    let p1 = part1(&instructions);
    let p2 = part2(&instructions); 
    (p1, p2)
}

fn to_instr(a: &[u8], b: i32) -> Instruction {
    match a {
        b"acc" => Instruction::Acc(b),
        b"jmp" => Instruction::Jmp(b),
        b"nop" => Instruction::Nop(b),
        _ => panic!("Invalid instruction"),
    }
}

fn part1(instructions: &[Instruction]) -> i32 {
    let mut seen = vec![false; instructions.len()];
    let mut idx = 0;
    let mut acc = 0;
    loop {
        if idx >= instructions.len() {
            panic!("Part1: The program does not loop")
        }
        if seen[idx] {
            return acc;
        }
        seen[idx] = true;
        match instructions[idx] {
            Instruction::Nop(_) => idx += 1,
            Instruction::Jmp(n) => idx = idx.wrapping_add(n as usize),
            Instruction::Acc(n) => {idx += 1; acc += n},
        }
    }
}

fn part2(instructions: &[Instruction]) -> i32 {
    let mut seen = vec![false; instructions.len()];
    let mut idx = 0;
    let mut acc = 0;
    loop {
        if idx >= instructions.len() {
            panic!("Part2: The program does not loop")
        }
        seen[idx] = true;
        match instructions[idx] {
            Instruction::Nop(n) => {
                let idx2 = idx.wrapping_add(n as usize);
                if let Some(acc) = terminates(instructions, idx2, acc, &mut seen) {
                    return acc
                }
                idx += 1
            }
            Instruction::Jmp(n) => {
                if let Some(acc) = terminates(instructions, idx + 1, acc, &mut seen) {
                    return acc
                }
                idx = idx.wrapping_add(n as usize)
            }
            Instruction::Acc(n) => {idx += 1; acc += n},
        }
    }
}

fn terminates(instructions: &[Instruction], mut idx: usize, mut acc: i32, seen: &mut [bool]) -> Option<i32> {
    loop {
        if idx >= instructions.len() {
            return Some(acc)
        }
        if seen[idx] {
            return None
        }
        seen[idx] = true;
        match instructions[idx] {
            Instruction::Nop(_) => idx += 1,
            Instruction::Jmp(n) => idx = idx.wrapping_add(n as usize),
            Instruction::Acc(n) => {idx += 1; acc += n},
        }
    }
}