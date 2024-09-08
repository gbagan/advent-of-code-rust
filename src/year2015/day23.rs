use anyhow::*;
use crate::util::parser::*;

pub enum Instr {
    Hlf(bool),
    Tpl(bool),
    Inc(bool),
    Jmp(i8),
    Jie(bool, i8),
    Jio(bool, i8)
}

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let program: Vec<_> = input.try_parse_lines_and_collect(parse_instr)?;
    let p1 = run_program(&program, 0);
    let p2 = run_program(&program, 1);
    Ok((p1, p2))
}

fn parse_instr(line: &str) -> Result<Instr> {
    let (name, rest) = line.try_split_once(' ')?;
    match name {
        "hlf" => Ok(Instr::Hlf(rest == "a")),
        "tpl" => Ok(Instr::Tpl(rest == "a")),
        "inc" => Ok(Instr::Inc(rest == "a")),
        "jmp" => Ok(Instr::Jmp(rest.parse()?)),
        "jie" | "jio" => {
            let (reg, offset) = rest.try_split_once(", ")?;
            let reg = reg == "a";
            let offset = offset.parse()?;
            if name == "jie" {
                Ok(Instr::Jie(reg, offset))
            } else {
                Ok(Instr::Jio(reg, offset))
            }
        }
        _ => bail!("Expecting hlf, tpl, inc, jmp, jie")
    }
}

fn run_program(program: &[Instr], a: u32) -> u32 {
    let n = program.len() as i8;
    let mut offset: i8 = 0;
    let mut a = a;
    let mut b = 0;
    while offset < n {
        match program[offset as usize] {
            Instr::Hlf(i) => {if i {a /= 2} else {b /= 2}}
            Instr::Tpl(i) => {if i {a *= 3} else {b *= 3}}
            Instr::Inc(i) => {if i {a += 1} else {b += 1}},
            Instr::Jmp(o) => offset += o - 1,
            Instr::Jie(i, o) => if (if i {a} else {b}) % 2 == 0 {offset += o - 1},
            Instr::Jio(i, o) => if (if i {a} else {b}) == 1 {offset += o - 1},
        }
        offset += 1;
    }
    b
}