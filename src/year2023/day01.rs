const PATTERNS: [(&str, u32); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6), 
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6), 
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn parse(input: &str) -> Option<Vec<&str>> {
    Some(input.lines().collect())
}

fn solve_one(line: &str) -> Option<u32> {
    let n: u32 = line.chars().find_map(|c| c.to_digit(10))?;
    let m: u32 = line.chars().rev().find_map(|c| c.to_digit(10))?;
    Some(n*10+m)
}

fn matches_pattern(s: &str) -> Option<u32> {
    PATTERNS.iter().find_map(|&(pat, n)| s.starts_with(pat).then_some(n))
}

fn solve_two(line: &str) -> Option<u32> {
    let n: u32 = (0..line.len()).find_map(|i| matches_pattern(&line[i..]))?;
    let m: u32 = (0..line.len()).rev().find_map(|i| matches_pattern(&line[i..]))?;
    Some(n*10+m)
}

pub fn part1(input: &[&str]) -> Option<u32> {
    Some(input.iter().filter_map(|&line| solve_one(line)).sum())
}

pub fn part2(input: &[&str]) -> Option<u32> {
    Some(input.iter().filter_map(|&line| solve_two(line)).sum())
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