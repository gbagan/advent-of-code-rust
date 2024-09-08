use anyhow::*;

use crate::util::{iter::*, parser::*};

struct Password<'a> {
    value1: usize,
    value2: usize,
    letter: u8,
    pwd: &'a [u8]
}

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let passwords: Vec<_> = input.try_parse_lines_and_collect(parse_line)?;

    let p1 = passwords.iter().count_if(check1);
    let p2 = passwords.iter().count_if(check2);

    Ok((p1, p2))
}

fn parse_line(line: &str) -> Result<Password> {
    let (token1, token2, token3, token4) = line.try_split_into_tuple(&[' ', '-'])?;
    let value1 = token1.next_unsigned()?;
    let value2 = token2.next_unsigned()?;
    let letter = token3.as_bytes()[0];
    let pwd = token4.as_bytes();
    Ok(Password{value1, value2, letter, pwd})
}

fn check1(p: &Password) -> bool {
    let freq = p.pwd.iter().count_if(|&c| c == p.letter);
    p.value1 <= freq && freq <= p.value2
}

fn check2(p: &Password) -> bool {
    (p.pwd[p.value1-1] == p.letter) ^ (p.pwd[p.value2-1] == p.letter)
}