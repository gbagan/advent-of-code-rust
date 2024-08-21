use crate::util::iter::AOCIter;

fn diff1(s: &str) -> Option<u32> {
    let mut chars = s.chars();
    let mut count = 0;
    while let Some(c) = chars.next() {
        if c=='\\' {
            match chars.next() {
                Some('x') => {
                    chars.next();
                    chars.next();
                    count += 3;
                }
                Some(_) => count += 1,
                None => return None
            }
        }
    }
    Some(count + 2)
}

fn diff2(s: &str) -> u32 {
    2 + s.chars().count_by(|c| c == '\\' || c == '"') as u32
}

pub fn parse(input: &str) -> Option<Vec<&str>> {
    Some(input.lines().collect())
}

pub fn part1(input: &[&str]) -> Option<u32> {
    Some(input.iter().filter_map(|&line| diff1(line)).sum::<u32>())
}

pub fn part2(input: &[&str]) -> Option<u32> {
    Some(input.iter().map(|&line| diff2(line)).sum::<u32>())
}