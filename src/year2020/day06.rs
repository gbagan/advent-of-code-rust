pub fn solve(input: &str) -> (u32, u32) {
    let answers: Vec<Vec<_>> = input
        .split("\n\n")
        .map(|group| group.lines().map(parse_line).collect())
        .collect();
    let mut p1 = 0;
    let mut p2 = 0;

    for group in &answers {
        p1 += group.iter().fold(0, |x, &y| x | y).count_ones();
        p2 += group.iter().fold(u32::MAX, |x, &y| x & y).count_ones();
    }

    (p1, p2)
}

fn parse_line(line: &str) ->  u32 {
    line.bytes().fold(0, |acc, c| {
        acc | 1 << (c - b'a')
    })
}
