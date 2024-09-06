use anyhow::*;
use itertools::Itertools;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let p1 = part1(input);
    let p2 = part2(input);

    Ok((p1, p2))
}

fn part1(input: &str) -> u32 {
    let mut inside = false;
    let mut inside_contains_abba = false;
    let mut outside_contains_abba = false;
    let mut counter = 0;
    for (a, b, c, d) in input.trim().bytes().tuple_windows() {
        if  a == d && b == c && a != b {
            if inside {
                inside_contains_abba = true;
            } else {
                outside_contains_abba = true;
            }
        } else if a == b'[' {
            inside = true;
        } else if a == b']' {
            inside = false;
        } else if a == b'\n' {
            if outside_contains_abba && !inside_contains_abba {
                counter += 1;
            }
            outside_contains_abba = false;
            inside_contains_abba = false;
        }
    }
    if outside_contains_abba && !inside_contains_abba {
        counter += 1
    }

    counter
}

fn part2(input: &str) -> u32 {
    let mut inside = false;
    let mut inside_aba = [0; 26*26];
    let mut outside_aba = [0; 26*26];
    let mut line_no = 1;
    let mut counter = 0;
    let mut found = false;
    for (a, b, c) in input.trim().bytes().tuple_windows() {
        if a.is_ascii_lowercase() {
            if a == c && a != b &&  b.is_ascii_lowercase() {
                let i = (a - b'a') as usize;
                let j = (b - b'a') as usize;
                if inside {
                    if outside_aba[26*j+i] == line_no {
                        found = true;
                    } else {
                    inside_aba[26*i+j] = line_no;
                    }
                } else if inside_aba[26*j+i] == line_no {
                    found = true;
                } else {
                    outside_aba[26*i+j] = line_no;
                }
            }
        } else if a == b'[' {
            inside = true;
        } else if a == b']' {
            inside = false;
        } else if a == b'\n' {
            if found {
                counter += 1;
            }
            found = false;
            line_no += 1;
        }
    }
    if found {
        counter += 1
    }

    counter
}