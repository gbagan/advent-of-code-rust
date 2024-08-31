use itertools::Itertools;

pub fn solve(input: &str) -> Option<(u32, u32)> {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();

    let p1 = lines.iter().map(|line| {
        let mid = line.len() / 2;
        let mask1 = mask(&line[..mid]);
        let mask2 = mask(&line[mid..]);
        (mask1 & mask2).trailing_zeros()
    }).sum();

    let p2 = lines.iter().tuples().map(|(line1, line2, line3)| {
        (mask(line1) & mask(line2) & mask(line3)).trailing_zeros()
    }).sum();

    Some((p1, p2))
}

pub fn priority(c: u8) -> u32 {
    if c.is_ascii_uppercase() {
        (c - b'A' + 27) as u32
    } else {
        (c - b'a' + 1) as u32
    }
}

fn mask(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0u64, |acc, &c| acc | 1 << priority(c))
}