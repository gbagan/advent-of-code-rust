pub fn solve(input: &str) -> Option<(i32, usize)> {
    let p1 = part1(input);
    let p2 = part2(input)?;
    Some((p1, p2))
}

pub fn part1(input: &str) -> i32 {
    input.bytes().fold(0, |acc, chr|
        match chr {
            b'(' => acc + 1,
            b')' => acc - 1,
            _ => acc
        }
    )
}

pub fn part2(input: &str) -> Option<usize> {
    input.bytes().scan(0, |acc, chr| {
        match chr {
            b'(' => *acc += 1,
            b')' => *acc -= 1,
            _ => ()
        };
        Some(*acc)
    }).position(|r| r < 0).map(|x| x + 1)
}
