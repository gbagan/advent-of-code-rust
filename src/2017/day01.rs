use aoc::aoc;

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
    aoc(|| {
        let p1 = captcha_sum(input, 1);
        let p2 = captcha_sum(input, input.len()/2);
        (p1, p2)
    })
}
