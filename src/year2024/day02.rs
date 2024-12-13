use anyhow::*;
use crate::util::parser::*;

#[derive(PartialEq, Eq)]
enum Safety {
    Safe, QuasiSafe, Unsafe
}

#[inline]
fn is_safe_pair(a: u8, b: u8) -> bool {
    (1..=3).contains(&(b.wrapping_sub(a)))
}

fn is_safe(mut it: impl Iterator<Item=u8>) -> Safety {
    let [first, second, third] =
        match it.next_chunk() {
            std::result::Result::Ok(x) => x,
            _ => return Safety::Safe
        };

    let (mut inc_safe, mut largest, mut largest2) =
        if is_safe_pair(first, second) {
            if is_safe_pair(second, third) {
                (Safety::Safe, third, second)
            } else {
                (Safety::QuasiSafe, second, second)
            }
        } else if is_safe_pair(second, third) || is_safe_pair(first, third) {
            (Safety::QuasiSafe, third, third)
        } else {
            (Safety::Unsafe, 0, 0)
        };

    let (mut dec_safe, mut lowest, mut lowest2) =
        if is_safe_pair(second, first) {
            if is_safe_pair(third, second) {
                (Safety::Safe, third, second)
            } else {
                (Safety::QuasiSafe, second, second)
            }
        } else if is_safe_pair(third, first) || is_safe_pair(third, second) {
            (Safety::QuasiSafe, third, third)
        } else {
            (Safety::Unsafe, 0, 0)
        };
    
    for next in it {
        match inc_safe {
            Safety::Safe => {
                if is_safe_pair(largest, next) {
                    largest2 = largest;
                    largest = next;
                } else if is_safe_pair(largest2, next) {
                    largest2 = next;
                    inc_safe = Safety::QuasiSafe;
                } else {
                    largest2 = largest;
                    inc_safe = Safety::QuasiSafe;
                }
            }
            Safety::QuasiSafe => {
                if is_safe_pair(largest, next) || is_safe_pair(largest2, next) {
                    largest = next;
                    largest2 = next;
                } else {
                    inc_safe = Safety::Unsafe;
                }
            }
            Safety::Unsafe => {}
        }
        match dec_safe {
            Safety::Safe => {
                if is_safe_pair(next, lowest) {
                    lowest2 = lowest;
                    lowest = next;
                } else if is_safe_pair(next, lowest2) {
                    lowest2 = next;
                    dec_safe = Safety::QuasiSafe;
                } else {
                    lowest2 = lowest;
                    dec_safe = Safety::QuasiSafe;
                }
            }
            Safety::QuasiSafe => {
                if is_safe_pair(next, lowest) || is_safe_pair(next, lowest2) {
                    lowest = next;
                    lowest2 = next;
                } else {
                    dec_safe = Safety::Unsafe;
                }
            }
            Safety::Unsafe => {}
        }
    }

    if inc_safe == Safety::Safe || dec_safe == Safety::Safe {
        Safety::Safe
    } else if inc_safe == Safety::QuasiSafe || dec_safe == Safety::QuasiSafe {
        Safety::QuasiSafe
    } else {
        Safety::Unsafe
    }
}

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        let row = line.iter_unsigned::<u8>();
        match is_safe(row) {
            Safety::Safe => { p1 += 1; p2 += 1 },
            Safety::QuasiSafe => p2 += 1,
            Safety::Unsafe => {},
        }
    }
    Ok((p1, p2))
}