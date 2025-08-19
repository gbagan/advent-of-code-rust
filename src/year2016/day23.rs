use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, u32) {
    let mut lines = input.lines();
    let a: u32 = lines.nth(19).unwrap().try_unsigned().unwrap();
    let b: u32 = lines.next().unwrap().try_unsigned().unwrap();
    let p = a * b;
    let p1 = FACTORIAL_7 + p;
    let p2 = FACTORIAL_12 + p;
    (p1, p2)
}

const FACTORIAL_7: u32 = factorial(7);
const FACTORIAL_12: u32 = factorial(12);

const fn factorial(n: u32) -> u32 {
    let mut res = 1;
    let mut i = 2;
    while i <= n {
        res *= i;
        i += 1;
    }
    res
}