use aoc::aoc_with_parser;
use std::collections::{HashMap, VecDeque};
use nom::{
    bytes::complete::tag,
    branch::alt,
    character::is_alphabetic,
    character::complete::{satisfy, line_ending, i64, space1},
    multi::separated_list1,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
enum Value {
    Reg(char),
    Val(i64),
}


#[derive(Debug)]
enum Instr {
    Snd (Value),
    Set (char, Value),
    Add (char, Value),
    Mul (char, Value),
    Mod (char, Value),
    Rcv (char),
    Jgz (Value, Value),
}

fn input_parser(input: &str) -> IResult<&str, Vec<Instr>> {
    fn register(input: &str) -> IResult<&str, char> {
        satisfy(|c| c.is_ascii_lowercase())(input)
    }
    
    fn value(input: &str) -> IResult<&str, Value> {
        alt((
            map(i64, Value::Val),
            map(register, Value::Reg),
        ))(input)
    }
    let sound = map(preceded(tag("snd "), value), Instr::Snd);
    let set = map(tuple((tag("set "), register, space1, value)),
                    |(_, x, _, y)| Instr::Set(x, y));
    let add = map(tuple((tag("add "), register, space1, value)),
                    |(_, x, _, y)| Instr::Add(x, y));
    let mul = map(tuple((tag("mul "), register, space1, value)),
                    |(_, x, _, y)| Instr::Mul(x, y));
    let mod_ = map(tuple((tag("mod "), register, space1, value)),
                    |(_, x, _, y)| Instr::Mod(x, y));
    let recover = map(preceded(tag("rcv "), register), Instr::Rcv);
    let jgz = map(tuple((tag("jgz "), value, space1, value)),
                    |(_, x, _, y)| Instr::Jgz(x, y));
    let instr = alt((sound, set, add, mul, mod_, recover, jgz));

    separated_list1(line_ending, instr)(input)
}

struct Program {
    regs: HashMap<char, i64>,
    offset: i64,
    last_played: i64,
    sends: u32,
    has_progressed: bool,
}

impl Program {
    fn new(id: i64) -> Self {
        let mut regs = HashMap::new();
        regs.insert('p', id);
        Program {
            regs,
            offset: 0,
            last_played: 0,
            sends: 0,
            has_progressed: false,
        }
    }

    fn read(&self, reg: char) -> i64 {
        *self.regs.get(&reg).unwrap_or(&0)
    }

    fn write(&mut self, reg: char, val: i64) {
        self.regs.insert(reg, val);
    }

    fn eval(&self, val: &Value) -> i64 {
        match val {
            Value::Val(n) => *n,
            Value::Reg(reg) => self.read(*reg),
        }
    }

    fn run_part1(&mut self,  instrs: &Vec<Instr>) -> i64 {
        loop {
            match &instrs[self.offset as usize] {
                Instr::Snd(val) => self.last_played = self.eval(val),
                Instr::Set(reg, val) => self.write(*reg, self.eval(val)),
                Instr::Add(reg, val) => self.write(*reg, self.read(*reg) + self.eval(val)),
                Instr::Mul(reg, val) => self.write(*reg, self.read(*reg) * self.eval(val)),
                Instr::Mod(reg, val) => self.write(*reg, self.read(*reg) % self.eval(val)),
                Instr::Rcv(reg) => {
                    if self.read(*reg) != 0 {
                        return self.last_played
                    }
                }
                Instr::Jgz(val, o) => {
                    if self.eval(val) > 0 {
                        self.offset += self.eval(o) - 1;
                    }
                }
            }
            self.offset += 1;
        }
    }

    fn run(&mut self, instrs: &Vec<Instr>, in_: &mut VecDeque<i64>, out: &mut VecDeque<i64>) {
        self.has_progressed = false;
        loop {
            if self.offset < 0 || self.offset >= instrs.len() as i64 {
                return;
            }
            match &instrs[self.offset as usize] {
                Instr::Snd(val) => {
                    self.sends += 1;
                    out.push_back(self.eval(val));
                },
                Instr::Set(reg, val) => self.write(*reg, self.eval(val)),
                Instr::Add(reg, val) => self.write(*reg, self.read(*reg) + self.eval(val)),
                Instr::Mul(reg, val) => self.write(*reg, self.read(*reg) * self.eval(val)),
                Instr::Mod(reg, val) => self.write(*reg, self.read(*reg) % self.eval(val)),
                Instr::Rcv(reg) => {
                    match in_.pop_front() {
                        None => return,
                        Some(v) => self.write(*reg, v),
                    }
                }
                Instr::Jgz(val, o) => {
                    if self.eval(val) > 0 {
                        self.offset += self.eval(o) - 1;
                    }
                }
            }
            self.offset += 1;
            self.has_progressed = true;
        }
    }
}

fn part2(instrs: &Vec<Instr>) -> u32 {
    let mut program0 = Program::new(0);
    let mut program1 = Program::new(1);
    let mut queue1 = VecDeque::new();
    let mut queue2 = VecDeque::new();
    loop {
        program0.run(&instrs, &mut queue1, &mut queue2);
        program1.run(&instrs, &mut queue2, &mut queue1);
        if !program0.has_progressed && !program1.has_progressed {
            return program1.sends;
        }
    }
}

fn main() {
    let input = include_str!("../../inputs/2017/18");
    aoc_with_parser(input, input_parser, |instrs| {
        let mut p = Program::new(0);
        let p1 = p.run_part1(&instrs);
        let p2 = part2(&instrs);
        (p1, p2)
    })
}