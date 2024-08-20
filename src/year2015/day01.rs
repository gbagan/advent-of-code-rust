pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> Option<i32> {
    Some(input.chars().fold(0, |acc, chr|
        match chr {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc
        }
    ))
}

pub fn part2(input: &str) -> Option<usize> {
    input.chars().scan(0, |acc, chr| {
        match chr {
            '(' => *acc = *acc + 1,
            ')' => *acc = *acc - 1,
            _ => ()
        };
        Some(*acc)
    }).position(|r| r < 0).map(|x| x + 1)
}
