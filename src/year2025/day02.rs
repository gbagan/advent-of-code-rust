use crate::util::{iter::*, parser::*};

pub fn solve(input: &str) -> (u64, u64) {
    let mut p1 = 0;
    let mut p2 = 0;

    let tokens = input
        .trim()
        .as_bytes()
        .split(|&c| c == b',' || c == b'-');

    for (start_token, end_token) in tokens.tuples() {
        let start = start_token.to_unsigned::<u64>();
        let end = end_token.to_unsigned::<u64>();
        for len in start_token.len()..=end_token.len() {
            match len {
                2 => {
                    let start = start.max(10);
                    let end = end.min(99);
                    p1 += sum_invalid_ids(start, end, 11);
                },
                3 => {
                    let start = start.max(100);
                    let end = end.min(999);
                    p2 += sum_invalid_ids(start, end, 111);
                }
                4 => {
                    let start = start.max(1_000);
                    let end = end.min(9_999);
                    p1 += sum_invalid_ids(start, end, 101);
                },
                5 => {
                    let start = start.max(10_000);
                    let end = end.min(99_999);
                    p2 += sum_invalid_ids(start, end, 11_111);
                }
                6 => {
                    let start = start.max(100_000);
                    let end = end.min(999_999);
                    p1 += sum_invalid_ids(start, end, 1001);
                    p2 += sum_invalid_ids(start, end, 10_101)
                        - sum_invalid_ids(start, end, 111_111);
                },
                7 => {
                    let start = start.max(1_000_000);
                    let end = end.min(9_999_999);
                    p2 += sum_invalid_ids(start, end, 1_111_111);
                },
                8 => {
                    let start = start.max(10_000_000);
                    let end = end.min(99_999_999);
                    p1 += sum_invalid_ids(start, end, 10_001);
                },
                9 => {
                    let start = start.max(100_000_000);
                    let end = end.min(999_999_999);
                    p2 += sum_invalid_ids(start, end, 1_001_001);
                },
                10 => {
                    let start = start.max(1_000_000_000);
                    let end = end.min(9_999_999_999);
                    p1 += sum_invalid_ids(start, end, 100_001);
                    p2 += sum_invalid_ids(start, end, 101_010_101)
                        - sum_invalid_ids(start, end, 1_111_111_111);
                },
                _ => {}
            }
        }
    }

    (p1, p1+p2)
}

fn sum_invalid_ids(start: u64, end: u64, mult: u64) -> u64 {
    let first = start.div_ceil(mult);
    let last = end / mult;
    mult * (last - first + 1) * (first + last) / 2
}