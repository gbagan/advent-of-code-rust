use itertools::Itertools;
use std::str::from_utf8;

fn increment_password(pwd: &mut Vec<u8>) {
    let n = pwd.len();
    for i in (0..n).rev() {
        if pwd[i] != b'z' {
            let mut c = pwd[i]+1;
            while c == b'i' || c == b'o'|| c == b'l' {
                c+=1;
            }
            pwd[i] = c;
            for j in i+1..n {
                pwd[j] = b'a'
            }
            return
        }
    }
    for i in 0..n {
        pwd[i] = b'a';
    }
    pwd.push(b'a');
}

fn has_straight(pwd: &Vec<u8>) -> bool {
	pwd.iter().tuple_windows().any(|(a, b, c)| *b == *a + 1 && *c == *b + 1)
}

fn two_pairs(pwd: &Vec<u8>) -> bool {
    let mut pairs = 0;
	let mut it = pwd.iter().tuple_windows();
	while let Some ((c1, c2)) = it.next() {
        if c1 == c2 {
            pairs += 1;
            it.next();
        }
    } 
	return pairs >= 2
}

pub fn parse(input: &str) -> (String, String) {
    let mut pwd = input.trim_end().as_bytes().to_vec();
    let mut i = 0;
    let mut p1 = "".to_string();
    loop {
        if has_straight(&pwd) && two_pairs(&pwd) {
            i += 1;
            let p = from_utf8(&pwd).map(|p| p.to_string()).unwrap();
            if i == 2 {
                return (p1, p);
            } else {
                p1 = p;
            }
        }
        increment_password(&mut pwd);
    }
}

pub fn part1(input: &(String, String)) -> Option<String> {
    Some(input.0.to_string())
}

pub fn part2(input: &(String, String)) -> Option<String> {
    Some(input.1.to_string())
}