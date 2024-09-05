use crate::util::parser::*;

fn fibonnaci(n: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;
    for _ in 0..n {
        (a, b) = (a+b, a);
    }
    a
}

pub fn solve(input: &str) -> Option<(u32, u32)> {
    let lines: Vec<_> = input.lines().collect();
    let x: u32 = lines[2].next_unsigned().unwrap();
    let y: u32 = lines[5].next_unsigned().unwrap();
    let z: u32 = lines[16].next_unsigned().unwrap();
    let t: u32 = lines[17].next_unsigned().unwrap();

    let p1 = fibonnaci(x) + z * t;
    let p2 = fibonnaci(x+y) + z * t;

    Some((p1, p2))
}