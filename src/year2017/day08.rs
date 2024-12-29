use anyhow::*;
use std::collections::HashMap;
use crate::util::parser::*;

struct Instr<'a> {
    var1: &'a str,
    cmd: &'a str,
    val1: i32,
    var2: &'a str,
    cmp: &'a str,
    val2: i32,
}

pub fn solve(input: &str) -> (i32, i32) {
    let mut vars: HashMap<&str, i32> = HashMap::new();
    let mut max_value = 0;
    // todo
    for instr in input.lines().flat_map(parse_line) {
        let var2 = *vars.get(instr.var2).unwrap_or(&0);
        if compare(var2, instr.cmp, instr.val2) {
            let var1 = *vars.get(instr.var1).unwrap_or(&0);
            let var1 = match instr.cmd {
                "inc" => var1 + instr.val1,
                "dec" => var1 - instr.val1,
                _ => panic!("Unexpected characters {}", instr.cmd),
            };
            max_value = max_value.max(var1);
            vars.insert(instr.var1, var1);
        }
    }
    let max_final_value = *vars.values().max().unwrap();
    (max_final_value, max_value)
}


fn parse_line(line: &str) -> Result<Instr> {
    let (var1, cmd, val1, _, var2, cmp, val2) = line.try_split_into_tuple(' ')?;
    let val1 = val1.try_signed()?;
    let val2 = val2.try_signed()?;
    Ok(Instr {var1, cmd, val1, var2, cmp, val2})
}

fn compare(a: i32, cmp: &str, b: i32) -> bool {
    match cmp {
        "==" => a == b,
        "!=" => a != b,
        "<=" => a <= b,
        ">=" => a >= b,
        "<" => a < b,
        ">" => a > b,
        _ => panic!("Unexpected character {cmp}") 
    }
}