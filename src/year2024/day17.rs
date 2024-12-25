use anyhow::*;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(String, u64)> {
    let mut it = input.iter_unsigned::<u64>();
    let [a, b, c] = it.next_chunk().ok().context("Invalid input")?;
    let program: Vec<_> = it.collect();
    
    let p1 = run(&program, a, b, c);
    let p1 = String::from_utf8(p1).unwrap();
    let p2 = quine(&program, 0, program.len()).context("Part 2: No solution found")?;

    Ok((p1, p2))
}

fn run(program: &[u64], mut a: u64, mut b: u64, mut c: u64) -> Vec<u8> {
    let mut output = Vec::with_capacity(20);

    let mut ip = 0;
    
    while ip < program.len() {
        let literal = program[ip+1];
        let combo = match literal {
            4 => a,
            5 => b,
            6 => c,
            _ => literal,
        };
        match program[ip] {
            0 => a >>= combo,
            1 => b ^= literal,
            2 => b = combo & 7,
            3 => if a != 0 {ip = literal as usize; continue},
            4 => b ^= c,
            5 => { 
                if output.len() > 0 {
                    output.push(b',');
                }
                output.push((combo as u8 & 7) + b'0');
            }
            6 => b = a >> combo,
            _ => c = a >> combo,
        }
        ip += 2
    }
    output
}


fn quine(program: &[u64], a: u64, idx: usize) -> Option<u64> {
    if idx == 0 {
        return Some(a)
    }
    for i in 0..8 {
        let ai = a << 3 | i;
        if let Some(output) = run_first_value(program, ai, 0, 0) {
            if output == program[idx-1] {
                if let Some(q) = quine(program, ai, idx-1) {
                    return Some(q)
                }
            }
        }
    }
    None
}

fn run_first_value(program: &[u64], mut a: u64, mut b: u64, mut c: u64) -> Option<u64> {
    let mut ip = 0;
    while ip < program.len() {
        let literal = program[ip+1];
        let combo = match literal {
            4 => a,
            5 => b,
            6 => c,
            _ => literal,
        };
        match program[ip] {
            0 => a >>= combo,
            1 => b ^= literal,
            2 => b = combo & 7,
            3 => if a != 0 {ip = literal as usize; continue},
            4 => b ^= c,
            5 => return Some(combo & 7),
            6 => b = a >> combo,
            _ => c = a >> combo,
        }
        ip += 2
    }
    None
}