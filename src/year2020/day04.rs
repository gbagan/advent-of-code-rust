use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, u32) {
    let input = input.as_bytes();
    let mut p1 = 0;
    let mut p2 = 0;

    let mut nb_fields = 0;
    let mut is_ok = true;
    for field in input.split(|&c| c == b' ' || c == b'\n') {
        if field.is_empty() {
            p1 += (nb_fields == 7) as u32;
            p2 += (nb_fields == 7 && is_ok) as u32;
            nb_fields = 0;
            is_ok = true;
        } else {
            let f = encode(field[0], field[1], field[2]);
            if f != CID {
                nb_fields += 1;
            }
            if !is_ok {
                continue;
            }
            let field2 = &field[4..];
            is_ok = is_ok && match f {
                BYR => check_range(field2, 1920, 2002),
                IYR => check_range(field2, 2010, 2020),
                EYR => check_range(field2, 2020, 2030),
                HGT => check_height(field2),
                HCL => check_hair_color(field2),
                ECL => check_eye_color(field2),
                PID => check_pid(field2),
                _ => true,
            };
        }
    }

    (p1, p2)
}

const fn encode(a: u8, b: u8, c: u8) -> u32 {
    (a as u32) << 16 | (b as u32) << 8 | c as u32
}

const BYR: u32 = encode(b'b', b'y', b'r');
const EYR: u32 = encode(b'e', b'y', b'r');
const CID: u32 = encode(b'c', b'i', b'd');
const IYR: u32 = encode(b'i', b'y', b'r');
const HGT: u32 = encode(b'h', b'g', b't');
const PID: u32 = encode(b'p', b'i', b'd');
const HCL: u32 = encode(b'h', b'c', b'l');
const ECL: u32 = encode(b'e', b'c', b'l');

fn check_range(field: &[u8], min: u32, max: u32) -> bool {
    if let Some(year) = field.try_unsigned::<u32>() && year >= min && year <= max {
        return true; 
    }
    false
}

fn check_height(field: &[u8]) -> bool {
    if let Some(prefix) = field.strip_suffix(b"cm") {
        check_range(prefix, 150, 193)
    } else if let Some(prefix) = field.strip_suffix(b"in") {
        check_range(prefix, 59, 76)
    } else {
        false
    }
}

fn check_hair_color(field: &[u8]) -> bool {
    field.len() == 7 && field[0] == b'#' 
        && field[1..].iter().all(|&c| matches!(c, b'0'..=b'9' | b'a'..=b'f'))
}

fn check_eye_color(field: &[u8]) -> bool {
    matches!(field, b"amb" | b"blu" | b"brn" | b"gry" | b"grn" | b"hzl" | b"oth")
}

fn check_pid(field: &[u8]) -> bool {
    field.len() == 9 && field.iter().all(u8::is_ascii_digit)
}