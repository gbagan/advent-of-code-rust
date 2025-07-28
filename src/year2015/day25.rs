use crate::util::{parser::*, power};

pub fn solve(input: &str) -> (u64, u64) {
    let [row, col] = input.iter_unsigned::<u64>().next_chunk().unwrap();
    let first_code = 20_151_125;
    let base = 252_533;
    let exp = (row + col - 1) * (row + col - 2) / 2 + col - 1;
    let p1 = mul(first_code, power(|&x, &y| mul(x, y), base, exp as usize));
    (p1, 0)
}

fn mul(x: u64, y: u64) -> u64 {
    (x * y) % 33_554_393
}