use std::collections::HashMap;
use itertools::Itertools;

struct Instr<'a> {
    var1: &'a str,
    cmd: &'a str,
    val1: i32,
    var2: &'a str,
    cmp: &'a str,
    val2: i32,
}

fn parse_line(line: &str) -> Option<Instr> {
    let (var1, cmd, val1, _, var2, cmp, val2) = line.split_ascii_whitespace().next_tuple()?;
    let val1 = val1.parse().ok()?;
    let val2 = val2.parse().ok()?;
    Some(Instr {var1, cmd, val1, var2, cmp, val2})
}

fn compare(a: i32, cmp: &str, b: i32) -> bool {
    match cmp {
        "==" => a == b,
        "!=" => a != b,
        "<=" => a <= b,
        ">=" => a >= b,
        "<" => a < b,
        ">" => a > b,
        _ => panic!("unexcepted character {cmp}") 
    }
}

pub fn solve(input: &str) -> Option<(i32, i32)> {
    let mut vars: HashMap<&str, i32> = HashMap::new();
    let mut max_value = 0;
    for instr in input.lines().filter_map(parse_line) {
        let var2 = *vars.get(instr.var2).unwrap_or(&0);
        if compare(var2, instr.cmp, instr.val2) {
            let var1 = *vars.get(instr.var1).unwrap_or(&0);
            let var1 = match instr.cmd {
                "inc" => var1 + instr.val1,
                "dec" => var1 - instr.val1,
                _ => panic!("unexcepted characters {}", instr.cmd),
            };
            max_value = max_value.max(var1);
            vars.insert(instr.var1, var1);
        }
    }
    let max_final_value = *vars.values().max().unwrap();
    Some((max_final_value, max_value))
}
