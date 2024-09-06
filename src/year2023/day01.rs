use anyhow::*;

const PATTERNS: [(&[u8], u32); 9] = [
    (b"one", 1),
    (b"two", 2),
    (b"three", 3),
    (b"four", 4),
    (b"five", 5),
    (b"six", 6), 
    (b"seven", 7),
    (b"eight", 8),
    (b"nine", 9),
];

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        p1 += solve_one(line).ok_or_else(|| anyhow!("No pattern found: {line}"))?;
        p2 += solve_two(line).ok_or_else(|| anyhow!("No pattern found: {line}"))?;
    }
    Ok((p1, p2))
}

fn matches_pattern1(c: u8) -> Option<u32> {
    if c.is_ascii_digit() {
        Some((c - b'0') as u32)
    } else {
        None
    }
}

fn solve_one(line: &str) -> Option<u32> {
    let n: u32 = line.bytes().find_map(matches_pattern1)?;
    let m: u32 = line.bytes().rev().find_map(matches_pattern1)?;
    Some(n*10+m)
}

fn matches_pattern2(s: &[u8]) -> Option<u32> {
    if s[0].is_ascii_digit() {
        Some((s[0] - b'0') as u32)
    } else {
        PATTERNS.iter().find_map(|&(pat, n)| s.starts_with(pat).then_some(n))
    }
}

fn solve_two(line: &str) -> Option<u32> {
    let line = line.as_bytes();
    let n: u32 = (0..line.len()).find_map(|i| matches_pattern2(&line[i..]))?;
    let m: u32 = (0..line.len()).rev().find_map(|i| matches_pattern2(&line[i..]))?;
    Some(n*10+m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(solve_one("a1b2c3d4e5f"), Some(15));
        assert_eq!(solve_one("treb7uchet"), Some(77));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_two("abcone2threexyz"), Some(13));
        assert_eq!(solve_two("4nineeightseven2"), Some(42));
    }
}