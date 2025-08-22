use crate::util::{iter::*, parser::*};

struct Password<'a> {
    value1: usize,
    value2: usize,
    letter: u8,
    pwd: &'a [u8]
}

pub fn solve(input: &str) -> (usize, usize) {
    let passwords: Vec<_> = input.lines().map(parse_line).collect();

    let p1 = passwords.iter().filter(|&pwd| check1(pwd)).count();
    let p2 = passwords.iter().filter(|&pwd| check2(pwd)).count();

    (p1, p2)
}

fn parse_line(line: &str) -> Password<'_> {
    let (token1, token2, token3, token4) = line.split(&[' ', '-']).next_tuple().unwrap();
    let value1 = token1.try_unsigned().unwrap();
    let value2 = token2.try_unsigned().unwrap();
    let letter = token3.as_bytes()[0];
    let pwd = token4.as_bytes();
    Password{value1, value2, letter, pwd}
}

fn check1(p: &Password) -> bool {
    let freq = p.pwd.iter().filter(|&&c| c == p.letter).count();
    p.value1 <= freq && freq <= p.value2
}

fn check2(p: &Password) -> bool {
    (p.pwd[p.value1-1] == p.letter) ^ (p.pwd[p.value2-1] == p.letter)
}