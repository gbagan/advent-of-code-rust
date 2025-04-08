use crate::util::parser::*;

fn fibonnaci(n: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;
    for _ in 0..n {
        (a, b) = (a+b, a);
    }
    a
}

pub fn solve(input: &str) -> (u32, u32) {
    let lines: Vec<_> = input.lines().collect();
    assert!(lines.len() >= 18, "The program must have at least 18 lines");
    let x: u32 = lines[2].try_unsigned().unwrap();
    let y: u32 = lines[5].try_unsigned().unwrap();
    let z: u32 = lines[16].try_unsigned().unwrap();
    let t: u32 = lines[17].try_unsigned().unwrap();

    let p1 = fibonnaci(x) + z * t;
    let p2 = fibonnaci(x+y) + z * t;

    (p1, p2)
}