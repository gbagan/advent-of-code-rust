use std::time::Instant;

fn captcha_sum (bytes: &[u8], shift: usize) -> usize {
    let mut acc = 0;
    let n = bytes.len();
    for (i, &c) in bytes.iter().enumerate() {
        if bytes[(i+shift) % n] == c {
            acc += (c - b'0') as usize;
        }
    }
    acc
}

fn main() {
    let input = include_bytes!("../../inputs/2017/01");
    let start = Instant::now();

    let p1 = captcha_sum(input, 1);
    let p2 = captcha_sum(input, input.len()/2);
    let end = start.elapsed().as_micros();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Time: {} μs", end);
}
