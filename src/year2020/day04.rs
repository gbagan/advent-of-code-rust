use ahash::HashMap;
use crate::util::parser::*;

type Passport<'a> = HashMap<&'a str, &'a str>;

const MANDATORY_FIELDS: [&str; 7] =  ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn solve(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;
    
    for passport in input.split("\n\n").map(parse_passport) {
        if check1(&passport) {
            p1 += 1;
            if check2(&passport) {
                p2 += 1;
            }
        }
    }

    (p1, p2)
}

fn parse_passport(input: &str) -> Passport<'_> {
    input
        .split_ascii_whitespace()
        .map(|token| token.split_once(':').unwrap())
        .collect()
}

fn check1(passport: &Passport) -> bool {
    MANDATORY_FIELDS.iter().all(|field| passport.contains_key(field))
}

fn check2(passport: &Passport) -> bool {
    check_range(passport["byr"], 1920, 2002)
        && check_range(passport["iyr"], 2010, 2020)
        && check_range(passport["eyr"], 2020, 2030)
        && check_height(passport["hgt"])
        && check_hair_color(passport["hcl"])
        && check_eye_color(passport["ecl"])
        && check_pid(passport["pid"])
}

fn check_range(field: &str, min: u32, max: u32) -> bool {
    if let Some(year) = field.try_unsigned::<u32>() {
        if year >= min && year <= max {
            return true;
        } 
    }
    false
}

fn check_height(field: &str) -> bool {
    if let Some(prefix) = field.strip_prefix("cm") {
        check_range(prefix, 150, 193)
    } else if let Some(prefix) = field.strip_prefix("in") {
        check_range(prefix, 59, 76)
    } else {
        false
    }
}

fn check_hair_color(field: &str) -> bool {
    let field = field.as_bytes();
    field.len() == 7 && field[0] == b'#' 
        && field[1..].iter().all(|&c| matches!(c, b'0'..=b'9' | b'a'..=b'f'))
}

fn check_eye_color(field: &str) -> bool {
    matches!(field, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn check_pid(field: &str) -> bool {
    let field = field.as_bytes();
    field.len() == 9 && field.iter().all(u8::is_ascii_digit)
}