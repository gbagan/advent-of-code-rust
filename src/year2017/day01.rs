pub fn solve(input: &str) -> Option<(usize, usize)> {
    let bytes = input.trim().as_bytes();
    let p1 = captcha_sum(bytes, 1);
    let p2 = captcha_sum(bytes, bytes.len() / 2);
    Some((p1, p2))
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