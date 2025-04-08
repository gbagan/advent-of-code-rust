use num_integer::Integer;
use crate::util::iter::*;

pub fn solve(input: &str) -> (String, String) {
    let ones = input.trim().bytes().map(|b| if b == b'1' { 1 } else { 0 }).partial_sums();
    let p1 = checksum(&ones, 272);
    let p2 = checksum(&ones, 35_651_584);

    (p1, p2)
}

fn checksum(input_ones: &[u32], n: u32) -> String {
    // p is the largest power of 2 that divides n
    let p = n & !(n - 1);
    let q = n / p;

    (0..q+1)
        .map(|i| count_ones(input_ones, i*p))
        .map_windows(|&[w1, w2]| if (w2 - w1).is_even() { '1' } else { '0' })
        .collect()
} 

fn count_ones(input_ones: &[u32], mut length: u32) -> u32 {
    let input_len = input_ones.len() as u32 - 1;
    let mut half = input_len;
    let mut full = 2 * half + 1;

    while full < length {
        half = full;
        full = 2 * half + 1;
    }

    let mut ones = 0;

    while input_len < length {
        while length <= half {
            half /= 2;
            full /= 2;
        }
        let next = full - length;
        ones += half - next;
        length = next;
    }

    ones + input_ones[length as usize]
}
