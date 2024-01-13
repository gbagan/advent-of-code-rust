use aoc::aoc;

fn digit_to_int(c: u8) -> Option<u32> {
    if c >= b'0' && c <= b'9' {
        Some ((c - b'0') as u32)
    } else {
        None
    }
}

fn part1(text: &str) -> u32 {
    let x = text.bytes().find_map(digit_to_int).unwrap();
    let y = text.bytes().rev().find_map(digit_to_int).unwrap();
    x * 10 + y
}

const DIGITS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn match_part2(text: &str) -> Option<u32> {
    DIGITS.iter().position(|&pat| text.starts_with(pat)).map(|idx| idx as u32 % 9 + 1)
}

fn part2(text: &str) -> u32 {
    let n = text.len();
    let x = (0..n).find_map(|i| match_part2(&text[i..])).unwrap();
    let y = (0..n).rev().find_map(|i| match_part2(&text[i..])).unwrap();
    x * 10 + y
}

fn main() {
    let input = include_str!("../../inputs/2023/01");
    aoc(|| (
        input.lines().map(part1).sum::<u32>(),
        input.lines().map(part2).sum::<u32>(),
    ))
}