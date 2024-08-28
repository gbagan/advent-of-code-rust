pub enum Instr {
    Hlf(bool),
    Tpl(bool),
    Inc(bool),
    Jmp(i8),
    Jie(bool, i8),
    Jio(bool, i8)
}

fn parse_instr(line: &str) -> Option<Instr> {
    let (name, rest) = line.split_once(' ')?;
    match name {
        "hlf" => Some(Instr::Hlf(rest == "a")),
        "tpl" => Some(Instr::Tpl(rest == "a")),
        "inc" => Some(Instr::Inc(rest == "a")),
        "jmp" => rest.parse().ok().map(Instr::Jmp),
        "jie" | "jio" => {
            let (reg, offset) = rest.split_once(", ")?;
            let reg = reg == "a";
            let offset = offset.parse().ok()?;
            if name == "jie" {
                Some(Instr::Jie(reg, offset))
            } else {
                Some(Instr::Jio(reg, offset))
            }
        }
        _ => None
    }
}

pub fn parse(input: &str) -> Option<Vec<Instr>> {
    Some(input.lines().filter_map(parse_instr).collect())
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

pub fn part1(program: &[Instr]) -> Option<u32> {
    Some(run_program(program, 0))
}

pub fn part2(program: &[Instr]) -> Option<u32> {
    Some(run_program(program, 1))
}