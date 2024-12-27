pub fn solve(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        p1 += solve_one(line);
        p2 += solve_two(line);
    }
    (p1, p2)
}

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

fn solve_one(line: &str) -> u32 {
    let c1 = line.bytes().find(|c| c.is_ascii_digit()).unwrap();
    let c2 = line.bytes().rfind(|c| c.is_ascii_digit()).unwrap();
    let n = (c1 - b'0') as u32;
    let m = (c2 - b'0') as u32;
    n*10+m
}

fn matches_pattern2(s: &[u8]) -> Option<u32> {
    if s[0].is_ascii_digit() {
        Some((s[0] - b'0') as u32)
    } else {
        PATTERNS.iter().find_map(|&(pat, n)| s.starts_with(pat).then_some(n))
    }
}

fn solve_two(line: &str) -> u32 {
    let line = line.as_bytes();
    let n: u32 = (0..line.len()).find_map(|i| matches_pattern2(&line[i..])).unwrap();
    let m: u32 = (0..line.len()).rev().find_map(|i| matches_pattern2(&line[i..])).unwrap();
    n*10+m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(solve_one("a1b2c3d4e5f"), 15);
        assert_eq!(solve_one("treb7uchet"), 77);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_two("abcone2threexyz"), 13);
        assert_eq!(solve_two("4nineeightseven2"), 42);
    }
}