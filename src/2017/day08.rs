use aoc::aoc;
use std::collections::HashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, i32, space1},
    combinator::map,
    multi::separated_list1,
    IResult,
};
enum Command { Inc, Dec }

enum Cmp {Eq, Neq, Le, Ge, Lt, Gt}

struct Instr {
    var1: String,
    cmd: Command,
    val1: i32,
    var2: String,
    cmp: Cmp,
    val2: i32,
}

fn input_parser(input: &str) -> IResult<&str, Vec<Instr>> {
    fn instr(input: &str) -> IResult<&str, Instr> {
        let (input, var1) = alpha1(input)?;
        let (input, _) = space1(input)?;
        let (input, cmd) = command(input)?;
        let (input, _) = space1(input)?;
        let (input, val1) = i32(input)?;
        let (input, _) = tag(" if ")(input)?;
        let (input, var2) = alpha1(input)?;
        let (input, _) = space1(input)?;
        let (input, cmp) = cmp_parser(input)?;
        let (input, _) = space1(input)?;
        let (input, val2) = i32(input)?;
        Ok((input, Instr {
            var1: var1.to_string(),
            cmd: cmd,
            val1: val1,
            var2: var2.to_string(),
            cmp: cmp,
            val2: val2
        }))
    }

    fn command(input: &str) -> IResult<&str, Command> {
        alt((
            map(tag("inc"), |_| Command::Inc),
            map(tag("dec"), |_| Command::Dec),
        ))(input)
    }

    fn cmp_parser(input: &str) -> IResult<&str, Cmp> {
        alt((
            map(tag("=="), |_| Cmp::Eq),
            map(tag("!="), |_| Cmp::Neq),
            map(tag("<="), |_| Cmp::Le),
            map(tag(">="), |_| Cmp::Ge),
            map(tag("<"), |_| Cmp::Lt),
            map(tag(">"), |_| Cmp::Gt),
        ))(input)
    }

    separated_list1(line_ending, instr)(input)
}

fn compare(a: i32, cmp: &Cmp, b: i32) -> bool {
    match cmp {
        Cmp::Eq => a == b,
        Cmp::Neq => a != b,
        Cmp::Le => a <= b,
        Cmp::Ge => a >= b,
        Cmp::Lt => a < b,
        Cmp::Gt => a > b, 
    }
}

fn run(instrs: &Vec<Instr>) -> (i32, i32) {
    let mut vars: HashMap<String, i32> = HashMap::new();
    let mut max_value = 0;
    for instr in instrs {
        let var2 = *vars.get(&instr.var2).unwrap_or(&0);
        if compare(var2, &instr.cmp, instr.val2) {
            let var1 = *vars.get(&instr.var1).unwrap_or(&0);
            let var1 = match instr.cmd {
                Command::Inc => var1 + instr.val1,
                Command::Dec => var1 - instr.val1,
            };
            max_value = max_value.max(var1);
            vars.insert(instr.var1.clone(), var1);
        }
    }
    let max_final_value = *vars.values().max().unwrap();
    (max_final_value, max_value)
}

fn main() {
    let input = include_str!("../../inputs/2017/08");
    match input_parser(input) {
        Err(_) => println!("parsing error"),
        Ok ((_, instrs)) => aoc(|| run(&instrs)),
    }
}