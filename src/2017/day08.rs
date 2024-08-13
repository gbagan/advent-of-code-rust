use aoc::aoc_with_parser;
use std::collections::HashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, i32, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser
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
    let cmp_parser = alt((
            tag("==").map(|_: &str| Cmp::Eq),
            tag("!=").map(|_| Cmp::Neq),
            tag("<=").map(|_| Cmp::Le),
            tag(">=").map(|_| Cmp::Ge),
            tag("<").map(|_| Cmp::Lt),
            tag(">").map(|_| Cmp::Gt),
        ));

    let command = alt((
                tag("inc").map(|_| Command::Inc),
                tag("dec").map(|_| Command::Dec),
            ));

    let instr =
        tuple((alpha1, space1, command, space1, i32, tag(" if "), alpha1, space1, cmp_parser, space1, i32))
        .map(|(var1, _, cmd, _, val1, _, var2, _, cmp, _, val2)|
            Instr { var1: var1.to_string(), cmd, val1, var2: var2.to_string(), cmp, val2 });

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
    aoc_with_parser(input, input_parser, |instrs| run(&instrs))
}