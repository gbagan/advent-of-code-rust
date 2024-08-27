pub fn parse(input: &str) -> Option<&[u8]> {
    Some(input.trim().as_bytes())
}

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

pub fn part1(bytes: &[u8]) -> Option<usize> {
    Some(captcha_sum(bytes, 1))
}

pub fn part2(bytes: &[u8]) -> Option<usize> {
    Some(captcha_sum(bytes, bytes.len() / 2))
}
